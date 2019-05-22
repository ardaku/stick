use std::ffi::CString;
use std::fs;
use std::mem;

extern "C" {
    fn open(pathname: *const u8, flags: i32) -> i32;
    fn close(fd: i32) -> i32;
    fn fcntl(fd: i32, cmd: i32, v: i32) -> i32;
}

struct Device {
    name: Option<String>,
    fd: i32,
}

impl PartialEq for Device {
    fn eq(&self, other: &Device) -> bool {
        if let Some(ref name) = self.name {
            if let Some(ref name2) = other.name {
                name == name2
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub struct NativeManager {
    // Epoll File Descriptor.
    pub(crate) fd: i32,
    // Controller File Descriptors.
    devices: Vec<Device>,
}

impl NativeManager {
    pub fn new() -> NativeManager {
        NativeManager {
            fd: epoll_new(),
            devices: Vec::new(),
        }
    }

    /// Do a search for controllers.  Returns number of controllers.
    pub fn search(&mut self) -> (usize, usize) {
        let devices = find_devices();

        // Add devices
        for mut i in devices {
            if self.devices.contains(&i) {
                continue;
            }

            open_joystick(&mut i);

            // Setup device for asynchronous reads
            if i.fd != -1 {
                joystick_async(i.fd);

                let index = self.add(i);
                return (self.devices.len(), index);
            }
        }

        (self.num_plugged_in(), ::std::usize::MAX)
    }

    pub fn get_id(&self, id: usize) -> (u32, bool) {
        if id >= self.devices.len() {
            (0, true)
        } else {
            let (a, b) = joystick_id(self.devices[id].fd);

            (a, b)
        }
    }

    pub fn get_abs(&self, id: usize) -> (i32, i32, bool) {
        if id >= self.devices.len() {
            (0, 0, true)
        } else {
            joystick_abs(self.devices[id].fd)
        }
    }

    pub fn get_fd(&self, id: usize) -> (i32, bool, bool) {
        let (_, unplug) = self.get_id(id);

        (self.devices[id].fd, unplug, self.devices[id].name == None)
    }

    pub fn num_plugged_in(&self) -> usize {
        self.devices.len()
    }

    pub fn disconnect(&mut self, fd: i32) {
        for i in 0..self.devices.len() {
            if self.devices[i].fd == fd {
                epoll_del(self.fd, fd);
                joystick_drop(fd);
                self.devices[i].name = None;
                return;
            }
        }

        panic!("There was no fd of {}", fd);
    }

    /*    pub(crate) fn poll_event(&self, i: usize, state: &mut State) {
        while joystick_poll_event(self.devices[i].fd, state) {}
    }*/

    fn add(&mut self, device: Device) -> usize {
        let mut r = 0;

        for i in &mut self.devices {
            if i.name == None {
                *i = device;
                return r;
            }

            r += 1;
        }

        epoll_add(self.fd, device.fd);
        self.devices.push(device);

        r
    }
}
impl Drop for NativeManager {
    fn drop(&mut self) {
        while let Some(device) = self.devices.pop() {
            self.disconnect(device.fd);
        }
        unsafe { close(self.fd); }
    }
}

// Find the evdev device.
fn find_devices() -> Vec<Device> {
    let mut rtn = Vec::new();
    let paths = fs::read_dir("/dev/input/by-id/");
    let paths = if let Ok(paths) = paths {
        paths
    } else {
        return vec![];
    };

    for path in paths {
        let path_str = path.unwrap().path();
        let path_str = path_str.to_str().unwrap();

        // An evdev device.
        if path_str.ends_with("-event-joystick") {
            rtn.push(Device {
                name: Some(path_str.to_string()),
                fd: -1,
            });
        }
    }

    rtn
}

// Open the evdev device.
fn open_joystick(device: &mut Device) {
    let file_name = CString::new(device.name.clone().unwrap()).unwrap();

    device.fd = unsafe { open(file_name.as_ptr() as *const _, 0) };
}

// Set up file descriptor for asynchronous reading.
fn joystick_async(fd: i32) {
    let error = unsafe { fcntl(fd, 0x4, 0x800) } == -1;

    if error {
        panic!("Joystick unplugged 2!");
    }
}

// Get the joystick id.
fn joystick_id(fd: i32) -> (u32, bool) {
    let mut a = [0u16; 4];

    extern "C" {
        fn ioctl(fd: i32, request: usize, v: *mut u16) -> i32;
    }

    if unsafe { ioctl(fd, 0x_8008_4502, &mut a[0]) } == -1 {
        return (0, true);
    }

    (((a[1] as u32) << 16) | (a[2] as u32), false)
}

fn joystick_abs(fd: i32) -> (i32, i32, bool) {
    #[derive(Debug)]
    #[repr(C)]
    struct AbsInfo {
        value: i32,
        minimum: i32,
        maximum: i32,
        fuzz: i32,
        flat: i32,
        resolution: i32,
    }

    let mut a = unsafe { mem::uninitialized() };

    extern "C" {
        fn ioctl(fd: i32, request: usize, v: *mut AbsInfo) -> i32;
    }

    if unsafe { ioctl(fd, 0x_8018_4540, &mut a) } == -1 {
        return (0, 0, true);
    }

    (a.minimum, a.maximum, false)
}

// Disconnect the joystick.
fn joystick_drop(fd: i32) {
    if unsafe { close(fd) == -1 } {
        panic!("Failed to disconnect joystick.");
    }
}

// // // // // //
//    EPOLL    //
// // // // // //

#[repr(C)]
union EpollData {
    ptr: *mut std::ffi::c_void,
    fd: i32,
    uint32: u32,
    uint64: u64,
}

#[repr(C)]
struct EpollEvent {
    events: u32,        /* Epoll events */
    data: EpollData,    /* User data variable */
}

extern "C" {
    fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut EpollEvent) -> i32;
}

fn epoll_new() -> i32 {
    extern "C" {
        fn epoll_create1(flags: i32) -> i32;
    }

    let fd = unsafe { epoll_create1(0) };

    if fd == -1 {
        panic!("Couldn't create epoll!");
    }

    fd
}

fn epoll_add(epoll: i32, newfd: i32) {
    let mut event = EpollEvent {
        events: 0x001 /*EPOLLIN*/,
        data: EpollData { fd: newfd },
    };

    if unsafe { epoll_ctl(epoll, 1 /*EPOLL_CTL_ADD*/, newfd, &mut event) } == -1
    {
        unsafe { close(newfd); }
        panic!("Failed to add file descriptor to epoll");
    }
}

fn epoll_del(epoll: i32, newfd: i32) {
    let mut event = EpollEvent {
        events: 0x001 /*EPOLLIN*/,
        data: EpollData { fd: newfd },
    };

    if unsafe { epoll_ctl(epoll, 2 /*EPOLL_CTL_DEL*/, newfd, &mut event) } == -1
    {
        unsafe { close(newfd); }
        panic!("Failed to add file descriptor to epoll");
    }
}

pub(crate) fn epoll_wait(epoll_fd: i32) -> Option<i32> {
    extern "C" {
        fn epoll_wait(epfd: i32, events: *mut EpollEvent,
                      maxevents: i32, timeout: i32) -> i32;
    }

    let mut events: EpollEvent = EpollEvent {
        events: 0,
        data: EpollData { fd: 0 },
    };

    if unsafe { epoll_wait(epoll_fd, &mut events, 1 /*MAX_EVENTS*/, -1) } == 1 {
        return Some(unsafe { events.data.fd });
    } else {
        return None;
    }
}
