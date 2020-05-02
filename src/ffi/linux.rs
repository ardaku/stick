use smelling_salts::{Device as AsyncDevice, Watcher};

use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
use std::os::unix::io::{IntoRawFd, RawFd};
use std::task::{Context, Poll};

use crate::Event;

#[repr(C)]
struct InotifyEv {
    // struct inotify_event, from C.
    wd: c_int, /* Watch descriptor */
    mask: u32, /* Mask describing event */
    cookie: u32, /* Unique cookie associating related
               events (for rename(2)) */
    len: u32,            /* Size of name field */
    name: [c_char; 256], /* Optional null-terminated name */
}

#[repr(C)]
struct TimeVal {
    // struct timeval, from C.
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct TimeSpec {
    // struct timespec, from C.
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct ItimerSpec {
    // struct itimerspec, from C.
    it_interval: TimeSpec,
    it_value: TimeSpec,
}

#[repr(C)]
struct InputId {
    // struct input_id, from C.
    bustype: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

#[repr(C)]
struct EvdevEv {
    // struct input_event, from C.
    ev_time: TimeVal,
    ev_type: c_ushort,
    ev_code: c_ushort,
    ev_value: c_uint,
}

#[repr(C)]
struct AbsInfo {
    // struct input_absinfo, from C.
    value: i32,
    minimum: u32,
    maximum: u32,
    fuzz: i32,
    flat: i32,
    resolution: i32,
}

extern "C" {
    fn read(fd: RawFd, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: RawFd) -> c_int;
    fn fcntl(fd: RawFd, cmd: c_int, v: c_int) -> c_int;
    fn ioctl(fd: RawFd, request: c_ulong, v: *mut c_void) -> c_int;

    fn inotify_init1(flags: c_int) -> c_int;
    fn inotify_add_watch(fd: RawFd, path: *const c_char, mask: u32) -> c_int;

    fn timerfd_create(clockid: c_int, flags: c_int) -> RawFd;
    fn timerfd_settime(
        fd: RawFd,
        flags: c_int,
        new_value: *const ItimerSpec,
        old_value: *mut ItimerSpec,
    ) -> c_int;

    fn __errno_location() -> *mut c_int;
}

struct PortTimer {
    device: AsyncDevice,
}

impl PortTimer {
    fn new(cx: &mut Context) -> Self {
        // Create the timer.
        let timerfd = unsafe {
            timerfd_create(
                1,      /*CLOCK_MONOTONIC*/
                0o4000, /*TFD_NONBLOCK*/
            )
        };
        assert_ne!(timerfd, -1); // Should never fail (unless out of memory).
                                 // Arm the timer for every 10 millis, starting in 10 millis.
        unsafe {
            timerfd_settime(
                timerfd,
                0,
                &ItimerSpec {
                    it_interval: TimeSpec {
                        tv_sec: 0,
                        tv_nsec: 10_000_000, // 10 milliseconds
                    },
                    it_value: TimeSpec {
                        tv_sec: 0,
                        tv_nsec: 10_000_000, // 10 milliseconds
                    },
                },
                std::ptr::null_mut(),
            );
        }
        // Create timer device, watching for input events.
        let device = AsyncDevice::new(timerfd, Watcher::new().input());
        // Wake up Future when timer goes off.
        device.register_waker(cx.waker());

        // Return timer
        PortTimer { device }
    }
}

impl Drop for PortTimer {
    fn drop(&mut self) {
        let fd = self.device.fd();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}

/// Port
pub(crate) struct Port {
    device: AsyncDevice,
    connected: HashSet<String>,
    timer: Option<PortTimer>,
}

impl Port {
    pub(super) fn new() -> Self {
        // Create an inotify on the directory where gamepad filedescriptors are.
        let inotify = unsafe {
            inotify_init1(0o0004000 /*IN_NONBLOCK*/)
        };
        if inotify == -1 {
            panic!("Couldn't create inotify (1)!");
        }
        if unsafe {
            inotify_add_watch(
                inotify,
                b"/dev/input/by-id/\0".as_ptr() as *const _,
                0x0000_0200 | 0x0000_0100,
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

        // Start off with timer disabled.
        let timer = None;

        // Return
        Port {
            device,
            connected,
            timer,
        }
    }

    pub(super) fn poll(&mut self, cx: &mut Context) -> Poll<(usize, Event)> {
        // Timeout after joystick doesn't give up permissions for 1 second.
        if let Some(ref timer) = self.timer {
            let mut num = MaybeUninit::<u64>::uninit();
            if unsafe {
                read(
                    timer.device.fd(),
                    num.as_mut_ptr().cast(),
                    std::mem::size_of::<u64>(),
                )
            } == std::mem::size_of::<u64>() as isize
            {
                if unsafe { num.assume_init() } >= 100 {
                    self.timer = None;
                }
            }
        }

        // Read an event.
        let mut ev = MaybeUninit::<InotifyEv>::uninit();
        let ev = unsafe {
            if read(
                self.device.fd(),
                ev.as_mut_ptr().cast(),
                std::mem::size_of::<InotifyEv>(),
            ) <= 0
            {
                let mut all_open = true;
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
                        let fd = match File::open(filename) {
                            Ok(f) => f,
                            Err(e) => {
                                if e.kind() == ErrorKind::PermissionDenied {
                                    all_open = false;
                                }
                                continue 'fds;
                            }
                        };
                        self.connected.insert(file);
                        return Poll::Ready((
                            usize::MAX,
                            Event::Connect(Box::new(crate::Gamepad(
                                Gamepad::new(fd),
                            ))),
                        ));
                    }
                }
                // If all gamepads are openned, disable timer.
                if all_open && self.timer.is_some() {
                    self.timer = None;
                }
                // Register waker for this device
                self.device.register_waker(cx.waker());
                // If no new controllers found, return pending.
                return Poll::Pending;
            }
            ev.assume_init()
        };

        // Remove flag is set, remove from HashSet.
        if (ev.mask & 0x0000_0200) != 0 {
            let mut file = "".to_string();
            let name = unsafe { std::ffi::CStr::from_ptr(ev.name.as_ptr()) };
            file.push_str(&name.to_string_lossy());
            if file.ends_with("-event-joystick") {
                assert!(self.connected.remove(&file));
            }
        }
        // Add flag is set, wait for permissions (unfortunately, can't rely on
        // epoll events for this, so check every 10 milliseconds).
        if (ev.mask & 0x0000_0100) != 0 && self.timer.is_none() {
            self.timer = Some(PortTimer::new(cx));
        }
        // Check for more events, Search for new controllers again, and return
        // Pending if neither have anything to process.
        self.poll(cx)
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
    emulated: u8,
}

impl Gamepad {
    fn new(file: File) -> Self {
        let fd = file.into_raw_fd();
        // Enable evdev async.
        assert_ne!(unsafe { fcntl(fd, 0x4, 0x800) }, -1);
        // Get the hardware id of this controller.
        let mut a = MaybeUninit::<InputId>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x_8008_4502, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        let hardware_id =
            ((u32::from(a.vendor)) << 16) | (u32::from(a.product));
        // Get the min and max absolute values for axis.
        let mut a = MaybeUninit::<AbsInfo>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x_8018_4540, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        let abs_min = a.minimum as c_int;
        let abs_range = a.maximum as c_int - a.minimum as c_int;
        // Construct device from fd, looking for input events.
        Gamepad {
            hardware_id,
            abs_min,
            abs_range,
            queued: None,
            device: AsyncDevice::new(fd, Watcher::new().input()),
            emulated: 0,
        }
    }

    fn to_float(&self, value: u32) -> f32 {
        (value as i32) as f32 * 0.00392156862745098
    }

    // Apply mods
    fn apply_mods(&self, mut event: Event) -> Event {
        let s = |x: f32| {
            // Scale based on advertized min and max values
            let v = ((255.0 * x) - self.abs_min as f32) / self.abs_range as f32;
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
            Event::Accept(p) => {
                if self.hardware_id == 0x_0E6F_0501
                /*xbox*/
                {
                    event = Event::Cancel(p);
                }
            }
            Event::Cancel(p) => {
                if self.hardware_id == 0x_0E6F_0501
                /*xbox*/
                {
                    event = Event::Accept(p);
                }
            }
            Event::Common(p) => {
                if self.hardware_id == 0x_054C_0268
                /*ps3*/
                {
                    event = Event::Action(p);
                }
            }
            Event::Action(p) => {
                if self.hardware_id == 0x_054C_0268
                /*ps3*/
                {
                    event = Event::Common(p);
                }
            }
            Event::MotionH(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event =
                        Event::MotionH((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
                } else {
                    event =
                        Event::MotionH((s(v) * 2.0 - 1.0).min(1.0).max(-1.0));
                }
            }
            Event::MotionV(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event =
                        Event::MotionV((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
                } else {
                    event =
                        Event::MotionV((s(v) * 2.0 - 1.0).min(1.0).max(-1.0));
                }
            }
            Event::CameraH(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event = Event::Lz((v * 2.0 - 0.5).min(1.0).max(-1.0));
                } else {
                    event =
                        Event::CameraH((s(v) * 2.0 - 1.0).min(1.0).max(-1.0));
                }
            }
            Event::CameraV(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event = Event::Rz((v * 2.0 - 0.5).min(1.0).max(-1.0));
                } else {
                    event =
                        Event::CameraV((s(v) * 2.0 - 1.0).min(1.0).max(-1.0))
                }
            }
            Event::Lz(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event =
                        Event::CameraV((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
                } else {
                    event = Event::Lz(v.min(1.0).max(-1.0));
                }
            }
            Event::Rz(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event =
                        Event::CameraH((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
                } else {
                    event = Event::Rz(v.min(1.0).max(-1.0))
                }
            }
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
        let ev = {
            let bytes = unsafe {
                read(
                    self.device.fd(),
                    ev.as_mut_ptr().cast(),
                    std::mem::size_of::<EvdevEv>(),
                )
            };
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
            unsafe { ev.assume_init() }
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
                        eprintln!("Button 10 is Unknown, report at \
                            https://github.com/libcala/stick/issues");
                        return self.poll(cx);
                    }
                    // D-PAD
                    12 | 256 => Event::Up(is),
                    13 | 259 => Event::Right(is),
                    14 | 257 => Event::Down(is),
                    15 | 258 => Event::Left(is),
                    // 16-17 already matched
                    18 => {
                        eprintln!("Button 18 is Unknown, report at \
                            https://github.com/libcala/stick/issues");
                        return self.poll(cx);
                    }
                    // 19-20 already matched
                    21 => {
                        eprintln!("Button 21 is Unknown, report at \
                            https://github.com/libcala/stick/issues");
                        return self.poll(cx);
                    }
                    // 22-27 already matched
                    28 => {
                        if is {
                            Event::Quit
                        } else {
                            return self.poll(cx);
                        }
                    }
                    29 => Event::MotionButton(is),
                    30 => Event::CameraButton(is),
                    a => {
                        eprintln!("Button {} is Unknown, report at \
                            https://github.com/libcala/stick/issues", a);
                        return self.poll(cx);
                    }
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
                    /*16 => {
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
                    }*/
                    40 => return self.poll(cx), // IGNORE: Duplicate axis.
                    a => {
                        eprintln!("Unknown Axis: {}, report at \
                            https://github.com/libcala/stick/issues", a);
                        return self.poll(cx);
                    }
                }
            }
            // ignore autorepeat, relative.
            _ => return self.poll(cx),
        };

        Poll::Ready(self.apply_mods(event))
    }
    
    pub(super) fn name(&self) -> String {
        "Unknown".to_string() // FIXME
    }
}

impl Drop for Gamepad {
    fn drop(&mut self) {
        let fd = self.device.fd();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}
