use smelling_salts::{Device as AsyncDevice, Watcher};

use std::fs;
use std::fs::File;
use std::mem::MaybeUninit;
use std::collections::HashSet;
use std::os::unix::io::{RawFd, IntoRawFd};
use std::os::raw::{c_int, c_uint, c_ulong, c_ushort, c_void};
use std::convert::TryInto;
use std::task::{Poll, Context};

use crate::Event;

#[repr(C)]
struct TimeVal {
    tv_sec: isize,
    tv_usec: isize,
}

#[repr(C)]
struct InputId { // struct input_id, from C.
	bustype: u16,
	vendor: u16,
	product: u16,
	version: u16,
}

#[repr(C)]
struct EvdevEv { // struct input_event, from C.
    ev_time: TimeVal,
    ev_type: c_ushort,
    ev_code: c_ushort,
    ev_value: c_uint,
}

#[repr(C)]
struct AbsInfo { // struct input_absinfo, from C.
    value: i32,
    minimum: u32,
    maximum: u32,
    fuzz: i32,
    flat: i32,
    resolution: i32,
}

#[repr(C)]
struct InotifyEv {
    wd: i32,   /* Watch descriptor */
    mask: u32, /* Mask describing event */
    cookie: u32, /* Unique cookie associating related
               events (for rename(2)) */
    len: u32,        /* Size of name field */
    name: [u8; 256], /* Optional null-terminated name */
}

extern "C" {
    // fn open(pathname: *const u8, flags: c_int) -> c_int;
    fn read(fd: RawFd, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: RawFd) -> c_int;
    fn fcntl(fd: RawFd, cmd: c_int, v: c_int) -> c_int;
    fn ioctl(fd: RawFd, request: c_ulong, v: *mut c_void) -> c_int;

    fn inotify_init1(flags: c_int) -> c_int;
    fn inotify_add_watch(fd: RawFd, pathname: *const u8, mask: u32) -> c_int;
    
    fn __errno_location() -> *mut c_int;
}

/// Port
pub(crate) struct Port {
    device: AsyncDevice,
    connected: HashSet<String>,
}

impl Port {
    pub(super) fn new() -> Self {
        // Create an inotify on the directory where gamepad filedescriptors are.
        let inotify = unsafe { inotify_init1(0o0004000 /*IN_NONBLOCK*/) };
        if inotify == -1 {
            panic!("Couldn't create inotify (1)!");
        }
        if unsafe {
            inotify_add_watch(
                inotify,
                b"/dev/input/by-id/\0".as_ptr() as *const _,
                0x0000_0100 | 0x0000_0200,
            )
        } == -1
        {
            panic!("Couldn't create inotify (2)!");
        }
        
        // Create watcher, and register with fd as a "device".
        let watcher = Watcher::new().input();
        let device = AsyncDevice::new(inotify, watcher);

        // Start off with an empty hash set of connected devices.
        let connected = HashSet::new();

        // Return
        Port { device, connected }
    }
    
    pub(super) fn poll(&mut self, cx: &mut Context) -> Poll<(usize, Event)> {
        // Read an event.
        let mut ev = MaybeUninit::<InotifyEv>::uninit();
        let ev = unsafe {
            if read(
                self.device.fd(),
                ev.as_mut_ptr().cast(),
                std::mem::size_of::<InotifyEv>(),
            ) <= 0 {
                // Search directory for new controllers.
                'fds: for file in fs::read_dir("/dev/input/by-id/").unwrap() {
                    let file = file.unwrap().file_name().into_string().unwrap();
                    if file.ends_with("-event-joystick") {
                        // Found an evdev gamepad
                        if self.connected.contains(&file) {
                            // Already connected.
                            continue 'fds;
                        }
                        // New gamepad
                        let mut filename = "/dev/input/by-id/".to_string();
                        filename.push_str(&file);
                        let fd = if let Ok(f) = File::open(filename) {
                            f
                        } else {
                            continue 'fds;
                        };
                        self.connected.insert(file);
                        return Poll::Ready((usize::MAX, Event::Connect(Box::new(crate::Gamepad(Gamepad::new(fd))))));
                    }
                }
                // Register waker for this device
                self.device.register_waker(cx.waker());
                // If no new controllers found, return pending.
                return Poll::Pending;
            }
            ev.assume_init()
        };
        
        // Add or remove
        if (ev.mask & 0x0000_0200) != 0 {
            // Remove flag is set.
            let mut file = "".to_string();
            for c in ev.name.iter().cloned() {
                if c == b'\0' {
                    break;
                }
                let c: u32 = c.into();
                file.push(c.try_into().unwrap());
            }
            if file.ends_with("-event-joystick") {
                let s = self.connected.remove(&file);
            }
        }
        if (ev.mask & 0x0000_0100) != 0 {
            // Add flag is set.
            let mut file = "/dev/input/by-id/".to_string();
            let mut name = "".to_string();
            for c in ev.name.iter().cloned() {
                if c == b'\0' {
                    break;
                }
                let c: u32 = c.into();
                name.push(c.try_into().unwrap());
            }
            if name.ends_with("-event-joystick") {
                // Found an evdev gamepad
                if self.connected.contains(&name) {
                    // Already connected.
                    return self.poll(cx);
                }
                // New gamepad
                file.push_str(&name);
                let fd = if let Ok(fd) = File::open(&file) {
                    fd
                } else {
                    return self.poll(cx);
                };
                self.connected.insert(name);
                return Poll::Ready((usize::MAX, Event::Connect(Box::new(crate::Gamepad(Gamepad::new(fd))))));
            } else {
                return self.poll(cx);
            }
        } else {
            self.poll(cx)
        }
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        let fd = self.device.fd();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}

/// Gamepad
pub(crate) struct Gamepad {
    device: AsyncDevice,
    hardware_id: u32, // Which type of controller?
    abs_min: c_int,
    abs_range: c_int,
    queued: Option<Event>,
}

impl Gamepad {
    fn new(file: File) -> Self {
        let fd = file.into_raw_fd();
        // Enable evdev async.
        assert_ne!(unsafe { fcntl(fd, 0x4, 0x800) }, -1);
        // Get the hardware id of this controller.
        let mut a = MaybeUninit::<InputId>::uninit();
        assert_ne!(unsafe { ioctl(fd, 0x_8008_4502, a.as_mut_ptr().cast()) }, -1);
        let a = unsafe { a.assume_init() };
        let hardware_id = ((u32::from(a.vendor)) << 16) | (u32::from(a.product));
        // Get the min and max absolute values for axis.
        let mut a = MaybeUninit::<AbsInfo>::uninit();
        assert_ne!(unsafe { ioctl(fd, 0x_8018_4540, a.as_mut_ptr().cast()) }, -1);
        let a = unsafe { a.assume_init() };
        let abs_min = a.minimum as c_int;
        let abs_range = a.maximum as c_int - a.minimum as c_int;
        // Construct device from fd, looking for input events.
        Gamepad {
            hardware_id, abs_min, abs_range, queued: None,
            device: AsyncDevice::new(fd, Watcher::new().input())
        }
    }
    
    fn to_float(&self, value: u32) -> f32 {
        (value as i32) as f32 * 0.00392156862745098
    }
    
    // Apply mods
    fn apply_mods(&self, mut event: Event) -> Event {
        let s = |x: f32| {
            // Scale based on advertized min and max values
            let v = ((255.0 * x) - self.abs_min as f32) / (self.abs_range as f32);
            // Noise Filter
            let v = (255.0 * v).trunc() / 255.0;
            // Deadzone
            if (v - 0.5).abs() < 0.0625 {
                0.5
            } else {
                v
            }
        };
    
        // Mods (xbox has A & B opposite of other controllers, and ps3 has X & Y
        // opposite, gamecube needs axis to be scaled).
        match event {
            Event::Accept(p) => if self.hardware_id == 0x_0E6F_0501 /*xbox*/ {
                event = Event::Cancel(p);
            },
            Event::Cancel(p) => if self.hardware_id == 0x_0E6F_0501 /*xbox*/ {
                event = Event::Accept(p);
            },
            Event::Common(p) => if self.hardware_id == 0x_054C_0268 /*ps3*/ {
                event = Event::Action(p);
            },
            Event::Action(p) => if self.hardware_id == 0x_054C_0268 /*ps3*/ {
                event = Event::Common(p);
            },
            Event::MotionH(v) => if self.hardware_id == 0x_0079_1844 /*gc*/ {
                event = Event::MotionH((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
            } else {
                event = Event::MotionH((s(v) * 2.0 - 1.0).min(1.0).max(-1.0));
            },
            Event::MotionV(v) => if self.hardware_id == 0x_0079_1844 /*gc*/ {
                event = Event::MotionV((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
            } else {
                event = Event::MotionV((s(v) * 2.0 - 1.0).min(1.0).max(-1.0));
            },
            Event::CameraH(v) => if self.hardware_id == 0x_0079_1844 /*gc*/ {
                event = Event::Lz((v * 2.0 - 0.5).min(1.0).max(-1.0));
            } else {
                event = Event::CameraH((s(v) * 2.0 - 1.0).min(1.0).max(-1.0));
            },
            Event::CameraV(v) => if self.hardware_id == 0x_0079_1844 /*gc*/ {
                event = Event::Rz((v * 2.0 - 0.5).min(1.0).max(-1.0));
            } else {
                event = Event::CameraV((s(v) * 2.0 - 1.0).min(1.0).max(-1.0))
            },
            Event::Lz(v) => if self.hardware_id == 0x_0079_1844 /*gc*/ {
                event = Event::CameraV((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
            } else {
                event = Event::Lz(v.min(1.0).max(-1.0));
            },
            Event::Rz(v) => if self.hardware_id == 0x_0079_1844 /*gc*/ {
                event = Event::CameraH((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
            } else {
                event = Event::Rz(v.min(1.0).max(-1.0))
            },
            _ => {}
        }
        event
    }

    pub(super) fn id(&self) -> u32 {
        self.hardware_id
    }

    pub(super) fn poll(&mut self, cx: &mut Context) -> Poll<Event> {
        if let Some(event) = self.queued.take() {
            return Poll::Ready(self.apply_mods(event));
        }
    
        // Read an event.
        let mut ev = MaybeUninit::<EvdevEv>::uninit();
        let ev = unsafe {
            let bytes = read(
                self.device.fd(),
                ev.as_mut_ptr().cast(),
                std::mem::size_of::<EvdevEv>(),
            );
            if bytes <= 0 {
                let errno = unsafe { *__errno_location() };
                if errno == 19 {
                    return Poll::Ready(Event::Disconnect);
                }
                assert_eq!(errno, 11);
                // Register waker for this device
                self.device.register_waker(cx.waker());
                // If no new controllers found, return pending.
                return Poll::Pending;
            }
            assert_eq!(std::mem::size_of::<EvdevEv>() as isize, bytes);
            ev.assume_init()
        };
        
        let event = match ev.ev_type {
            // button press / release (key)
            0x01 => {
                let is = ev.ev_value == 1;

                match ev.ev_code - 0x120 {
                    // ABXY
                    0 | 19 => Event::Common(is),
                    1 | 17 => Event::Accept(is),
                    2 | 16 => Event::Cancel(is),
                    3 | 20 => Event::Action(is),
                    // LT/RT
                    4 | 24 => return self.poll(cx), // Event::Lz(is),
                    5 | 25 => return self.poll(cx), // Event::Rz(is),
                    // Ignore LB/RB
                    6 | 22 => Event::L(is), // 6 is a guess.
                    7 | 23 => Event::R(is),
                    // Select/Start
                    8 | 26 => Event::Back(is), // 8 is a guess.
                    9 | 27 => Event::Forward(is),
                    // ?
                    10 => {
                        eprintln!("Button 10 is Unknown, report at https://github.com/libcala/stick/issues");
                        return self.poll(cx);
                    },
                    // D-PAD
                    12 | 256 => Event::Up(is),
                    13 | 259 => Event::Right(is),
                    14 | 257 => Event::Down(is),
                    15 | 258 => Event::Left(is),
                    // 16-17 already matched
                    18 => {
                        eprintln!("Button 18 is Unknown, report at https://github.com/libcala/stick/issues");
                        return self.poll(cx);
                    },
                    // 19-20 already matched
                    21 => {
                        eprintln!("Button 21 is Unknown, report at https://github.com/libcala/stick/issues");
                        return self.poll(cx);
                    },
                    // 22-27 already matched
                    28 => if is { Event::Quit } else { return self.poll(cx) },
                    29 => Event::MotionButton(is),
                    30 => Event::CameraButton(is),
                    a => {
                        eprintln!("Button {} is Unknown, report at https://github.com/libcala/stick/issues", a);
                        return self.poll(cx);
                    },
                }
            }
            // axis move (abs)
            0x03 => {
                match ev.ev_code {
                    0 => Event::MotionH(self.to_float(ev.ev_value)),
                    1 => Event::MotionV(self.to_float(ev.ev_value)),
                    2 => Event::Lz(self.to_float(ev.ev_value)),
                    3 => Event::CameraH(self.to_float(ev.ev_value)),
                    4 => Event::CameraV(self.to_float(ev.ev_value)),
                    5 => Event::Rz(self.to_float(ev.ev_value)),
                    16 => {
                        let value = self.to_float(ev.ev_value);
                        if value < 0.0 {
                            self.queued = Some(Event::Left(true));
                            Event::Right(false)
                        } else if value > 0.0 {
                            self.queued = Some(Event::Right(true));
                            Event::Left(false)
                        } else {
                            self.queued = Some(Event::Right(false));
                            Event::Left(false)
                        }
                    }
                    17 => {
                        let value = self.to_float(ev.ev_value);
                        if value < 0.0 {
                            self.queued = Some(Event::Up(true));
                            Event::Down(false)
                        } else if value > 0.0 {
                            self.queued = Some(Event::Down(true));
                            Event::Up(false)
                        } else {
                            self.queued = Some(Event::Down(false));
                            Event::Up(false)
                        }
                    }
                    40 => return self.poll(cx), // IGNORE: Duplicate axis.
                    a => {
                        eprintln!("Unknown Axis: {}", a);
                        return self.poll(cx);
                    },
                }
            }
            // ignore autorepeat, relative.
            _ => return self.poll(cx)
        };
        
        Poll::Ready(self.apply_mods(event))
    }
}

impl Drop for Gamepad {
    fn drop(&mut self) {
        let fd = self.device.fd();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}

/*    pub fn get_id(&self, id: usize) -> (u32, bool) {
        if id >= self.devices.len() {
            (0, true)
        } else {
            let (a, b) = joystick_id(self.devices[id].async_device.fd());

            (a, b)
        }
    }

    pub fn get_abs(&self, id: usize) -> (i32, i32, bool) {
        if id >= self.devices.len() {
            (0, 0, true)
        } else {
            joystick_abs(self.devices[id].async_device.fd())
        }
    }

    pub fn get_fd(&self, id: usize) -> (i32, bool, bool) {
        let (_, unplug) = self.get_id(id);

        (
            self.devices[id].async_device.fd(),
            unplug,
            self.devices[id].name[0] == b'\0',
        )
    }

    pub fn num_plugged_in(&self) -> usize {
        self.devices.len()
    }

    pub fn disconnect(&mut self, fd: i32) -> usize {
        for i in 0..self.devices.len() {
            if self.devices[i].async_device.fd() == fd {
                self.async_device.old();
                joystick_drop(fd);
                self.devices[i].name[0] = b'\0';
                return i;
            }
        }

        panic!("There was no fd of {}", fd);
    }
}
impl Drop for NativeManager {
    fn drop(&mut self) {
        let fds: Vec<i32> =
            self.devices.iter().map(|dev| dev.async_device.fd());
        for fd in fds {
            self.disconnect(fd);
        }
        self.devices.clear();
        unsafe {
            let fd = self.async_device.fd();
            self.async_device.old();
            close(fd);
        }
    }
}*/

/*// Get the joystick id.
fn joystick_id(fd: i32) -> (u32, bool) {

}

fn joystick_abs(fd: i32) -> (i32, i32, bool) {
    #[derive(Debug)]



}

// Disconnect the joystick.
fn joystick_drop(fd: i32) {
    if unsafe { close(fd) == -1 } {
        panic!("Failed to disconnect joystick.");
    }
}*/

/*// Add or remove joystick
fn inotify_read2(port: &mut NativeManager, ev: InotifyEv) -> Option<(bool, usize)> {
    let mut name = [0; 256 + 17];
    name[0] = b'/';
    name[1] = b'd';
    name[2] = b'e';
    name[3] = b'v';
    name[4] = b'/';
    name[5] = b'i';
    name[6] = b'n';
    name[7] = b'p';
    name[8] = b'u';
    name[9] = b't';
    name[10] = b'/';
    name[11] = b'b';
    name[12] = b'y';
    name[13] = b'-';
    name[14] = b'i';
    name[15] = b'd';
    name[16] = b'/';
    let mut length = 0;
    for i in 0..256 {
        name[i + 17] = ev.name[i];
        if ev.name[i] == b'\0' {
            length = i + 17;
            break;
        }
    }

    let namer = String::from_utf8_lossy(&name[0..length]);
    let mut fd = unsafe { open(name.as_ptr() as *const _, 0) };
    if !namer.ends_with("-event-joystick") || ev.mask != 0x0000_0100 {
        return None;
    }

    if fd == -1 {
        // Avoid race condition
        std::thread::sleep(std::time::Duration::from_millis(16));
        fd = unsafe { open(name.as_ptr() as *const _, 0) };
        if fd == -1 {
            return None;
        }
    }

    let async_device = AsyncDevice::new(fd, Watcher::new().input());
    let device = Device { name, async_device };

    for i in 0..port.devices.len() {
        if port.devices[i].name[0] == b'\0' {
            port.devices[i] = device;
            return Some((true, i));
        }
    }

    port.devices.push(device);
    Some((true, port.devices.len() - 1))
}*/
