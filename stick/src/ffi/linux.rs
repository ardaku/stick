// Stick
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use crate::{Event, Remap};
use smelling_salts::{Device, Watcher};
use std::cmp::Ordering;
use std::convert::TryInto;
use std::fs::read_dir;
use std::future::Future;
use std::mem::{size_of, MaybeUninit};
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
use std::os::unix::io::RawFd;
use std::pin::Pin;
use std::task::{Context, Poll};

// Event codes taken from
// https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h

// Convert Linux BTN press to stick Event.
fn linux_btn_to_stick_event(btn: c_ushort, pushed: bool) -> Option<Event> {
    Some(match btn {
        0x120 /* BTN_TRIGGER */ => Event::Trigger(pushed),
        0x121 /* BTN_THUMB */ => {
            eprintln!("FIXME: BTN_THUMB - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
        0x122 /* BTN_THUMB2 */ => {
            eprintln!("FIXME: BTN_THUMB2 - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
        0x123 /* BTN_TOP */ => {
            eprintln!("FIXME: BTN_TOP - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
        0x124 /* BTN_TOP2 */ => {
            eprintln!("FIXME: BTN_TOP2 - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
        0x125 /* BTN_PINKIE */ => {
            eprintln!("FIXME: BTN_PINKIE - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
        0x126 /* BTN_BASE1 */ => Event::Number(1, pushed),
        0x127 /* BTN_BASE2 */ => Event::Number(2, pushed),
        0x128 /* BTN_BASE3 */ => Event::Number(3, pushed),
        0x129 /* BTN_BASE4 */ => Event::Number(4, pushed),
        0x12A /* BTN_BASE5 */ => Event::Number(5, pushed),
        0x12B /* BTN_BASE6 */ => Event::Number(6, pushed),
        0x12C /* BTN_BASE7 */ => Event::Number(7, pushed),
        0x12D /* BTN_BASE8 */ => Event::Number(8, pushed),
        0x12E /* BTN_BASE9 */ => Event::Number(9, pushed),
        0x12F /* BTN_BASE10 */ => Event::Number(10, pushed),

        0x130 /* BTN_A / BTN_SOUTH */ => Event::ActionA(pushed),
        0x131 /* BTN_B / BTN_EAST */ => Event::ActionB(pushed),
        0x132 /* BTN_C */ => Event::ActionC(pushed),
        0x133 /* BTN_X / BTN_NORTH */ => Event::ActionV(pushed),
        0x134 /* BTN_Y / BTN_WEST */ => Event::ActionH(pushed),
        0x135 /* BTN_Z */ => Event::ActionD(pushed),
        0x136 /* BTN_TL */ => Event::BumperL(pushed),
        0x137 /* BTN_TR */ => Event::BumperR(pushed),
        0x138 /* BTN_TL2 */ => Event::TriggerL(f64::from(u8::from(pushed))),
        0x139 /* BTN_TR2 */ => Event::TriggerR(f64::from(u8::from(pushed))),
        0x13A /* BTN_SELECT */ => Event::MenuL(pushed),
        0x13B /* BTN_START */ => Event::MenuR(pushed),
        0x13C /* BTN_MODE */ => Event::Exit(pushed),
        0x13D /* BTN_THUMBL */ => Event::Joy(pushed),
        0x13E /* BTN_THUMBR */ => Event::Cam(pushed),

        0x220 /* BTN_DPAD_UP */ => Event::Up(pushed),
		0x221 /* BTN_DPAD_DOWN */ => Event::Down(pushed),
 		0x222 /* BTN_DPAD_LEFT */ => Event::Left(pushed),
 		0x223 /* BTN_DPAD_RIGHT */ => Event::Right(pushed),

        0x2C0 /* BTN_TRIGGER_HAPPY1 */ => Event::Number(11, pushed),
        0x2c1 /* BTN_TRIGGER_HAPPY2 */ => Event::Number(12, pushed),
        0x2c2 /* BTN_TRIGGER_HAPPY3 */ => Event::Number(13, pushed),
        0x2c3 /* BTN_TRIGGER_HAPPY4 */ => Event::Number(14, pushed),
        0x2c4 /* BTN_TRIGGER_HAPPY5 */ => Event::Number(15, pushed),
        0x2c5 /* BTN_TRIGGER_HAPPY6 */ => Event::Number(16, pushed),
        0x2c6 /* BTN_TRIGGER_HAPPY7 */ => Event::Number(17, pushed),
        0x2c7 /* BTN_TRIGGER_HAPPY8 */ => Event::Number(18, pushed),
        0x2c8 /* BTN_TRIGGER_HAPPY9 */ => Event::Number(19, pushed),
        0x2c9 /* BTN_TRIGGER_HAPPY10 */ => Event::Number(20, pushed),
        0x2ca /* BTN_TRIGGER_HAPPY11 */ => Event::Number(21, pushed),
        0x2cb /* BTN_TRIGGER_HAPPY12 */ => Event::Number(22, pushed),
        0x2cc /* BTN_TRIGGER_HAPPY13 */ => Event::Number(23, pushed),
        0x2cd /* BTN_TRIGGER_HAPPY14 */ => Event::Number(24, pushed),
        0x2ce /* BTN_TRIGGER_HAPPY15 */ => Event::Number(25, pushed),
        0x2cf /* BTN_TRIGGER_HAPPY16 */ => Event::Number(26, pushed),
        0x2d0 /* BTN_TRIGGER_HAPPY17 */ => Event::Number(27, pushed),
        0x2d1 /* BTN_TRIGGER_HAPPY18 */ => Event::Number(28, pushed),
        0x2d2 /* BTN_TRIGGER_HAPPY19 */ => Event::Number(29, pushed),
        0x2d3 /* BTN_TRIGGER_HAPPY20 */ => Event::Number(30, pushed),
        0x2d4 /* BTN_TRIGGER_HAPPY21 */ => Event::Number(31, pushed),
        0x2d5 /* BTN_TRIGGER_HAPPY22 */ => Event::Number(32, pushed),
        0x2d6 /* BTN_TRIGGER_HAPPY23 */ => Event::Number(33, pushed),
        0x2d7 /* BTN_TRIGGER_HAPPY24 */ => Event::Number(34, pushed),
        0x2d8 /* BTN_TRIGGER_HAPPY25 */ => Event::Number(35, pushed),
        0x2d9 /* BTN_TRIGGER_HAPPY26 */ => Event::Number(36, pushed),
        0x2da /* BTN_TRIGGER_HAPPY27 */ => Event::Number(37, pushed),
        0x2db /* BTN_TRIGGER_HAPPY28 */ => Event::Number(38, pushed),
        0x2dc /* BTN_TRIGGER_HAPPY29 */ => Event::Number(39, pushed),
        0x2dd /* BTN_TRIGGER_HAPPY30 */ => Event::Number(40, pushed),
        0x2de /* BTN_TRIGGER_HAPPY31 */ => Event::Number(41, pushed),
        0x2df /* BTN_TRIGGER_HAPPY32 */ => Event::Number(42, pushed),
        0x2e0 /* BTN_TRIGGER_HAPPY33 */ => Event::Number(43, pushed),
        0x2e1 /* BTN_TRIGGER_HAPPY34 */ => Event::Number(44, pushed),
        0x2e2 /* BTN_TRIGGER_HAPPY35 */ => Event::Number(45, pushed),
        0x2e3 /* BTN_TRIGGER_HAPPY36 */ => Event::Number(46, pushed),
        0x2e4 /* BTN_TRIGGER_HAPPY37 */ => Event::Number(47, pushed),
        0x2e5 /* BTN_TRIGGER_HAPPY38 */ => Event::Number(48, pushed),
        0x2e6 /* BTN_TRIGGER_HAPPY39 */ => Event::Number(49, pushed),
        0x2e7 /* BTN_TRIGGER_HAPPY40 */ => Event::Number(50, pushed),

        _unknown => {
            eprintln!("Unknown Linux Button {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
    })
}

// Convert Linux REL axis to stick Event.
fn linux_rel_to_stick_event(axis: c_ushort, value: c_int) -> Option<Event> {
    Some(match axis {
		0x00 /* REL_X */ => Event::MouseX(value as f64),
		0x01 /* REL_Y */ => Event::MouseY(value as f64),
		0x02 /* REL_Z */ => {
            eprintln!("FIXME: REL_Z - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x03 /* REL_RX */ => {
            eprintln!("FIXME: REL_RX - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x04 /* REL_RY */ => {
            eprintln!("FIXME: REL_RY - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x05 /* REL_RZ */ => {
            eprintln!("FIXME: REL_RZ - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x06 /* REL_HWHEEL */ => {
            eprintln!("FIXME: REL_HWHEEL - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x07 /* REL_DIAL */ => {
            eprintln!("FIXME: REL_DIAL - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x08 /* REL_WHEEL */ => {
            eprintln!("FIXME: REL_WHEEL - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x09 /* REL_MISC */ => {
            eprintln!("FIXME: REL_MISC - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
        _unknown => {
            eprintln!("Unknown Linux Axis {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
    })
}

// Convert Linux ABS axis to stick Event.
fn linux_abs_to_stick_event(axis: c_ushort, value: c_int) -> Option<Event> {
    Some(match axis {
		0x00 /* ABS_X */ => Event::JoyX(value as f64),
		0x01 /* ABS_Y */ => Event::JoyY(value as f64),
		0x02 /* ABS_Z */ => Event::JoyZ(value as f64),
		0x03 /* ABS_RX */ => Event::CamX(value as f64),
		0x04 /* ABS_RY */ => Event::CamY(value as f64),
		0x05 /* ABS_RZ */ => Event::CamZ(value as f64),
		0x06 /* ABS_THROTTLE */ => Event::Throttle(value as f64),
		0x07 /* ABS_RUDDER */ => Event::Rudder(value as f64),
		0x08 /* ABS_WHEEL */ => Event::Wheel(value as f64),
		0x09 /* ABS_GAS */ => Event::Gas(value as f64),
		0x0a /* ABS_BRAKE */ => Event::Brake(value as f64),
		0x10 /* ABS_HAT0X */ => match value.cmp(&0) {
            Ordering::Greater => Event::HatRight(true),
            Ordering::Less => Event::HatLeft(true),
            Ordering::Equal => Event::HatLeft(false)
        },
		0x11 /* ABS_HAT0Y */ => match value.cmp(&0) {
            Ordering::Greater => Event::HatDown(true),
            Ordering::Less => Event::HatUp(true),
            Ordering::Equal => Event::HatUp(false)
        },
		0x12 /* ABS_HAT1X */ => {
            eprintln!("FIXME: ABS_HAT1X - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x13 /* ABS_HAT1Y */ => {
            eprintln!("FIXME: ABS_HAT1Y - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x14 /* ABS_HAT2X */ => {
            eprintln!("FIXME: ABS_HAT2X - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x15 /* ABS_HAT2Y */ => {
            eprintln!("FIXME: ABS_HAT2Y - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x16 /* ABS_HAT3X */ => {
            eprintln!("FIXME: ABS_HAT3X - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x17 /* ABS_HAT3Y */ => {
            eprintln!("FIXME: ABS_HAT3Y - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x18 /* ABS_PRESSURE */ => {
            eprintln!("FIXME: ABS_PRESSURE - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x19 /* ABS_DISTANCE */ => {
            eprintln!("FIXME: ABS_DISTANCE - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x1a /* ABS_TILT_X */ => {
            eprintln!("FIXME: ABS_TILT_X - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x1b /* ABS_TILT_Y */ => {
            eprintln!("FIXME: ABS_TILT_Y - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x1c /* ABS_TOOL_WIDTH */ => {
            eprintln!("FIXME: ABS_TOOL_WIDTH - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x20 /* ABS_VOLUME */ => {
            eprintln!("FIXME: ABS_VOLUME - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
		0x28 /* ABS_MISC */ => {
            eprintln!("FIXME: ABS_MISC - Better Event Name");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
        _unknown => {
            eprintln!("Unknown Linux Axis {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return None;
        }
    })
}

fn linux_evdev_to_stick_event(e: EvdevEv) -> Option<Event> {
    match e.ev_type {
        0x00 /* SYN */ => None, // Ignore Syn Input Events
        0x01 /* BTN */ => linux_btn_to_stick_event(e.ev_code, e.ev_value != 0),
        0x02 /* REL */ => linux_rel_to_stick_event(e.ev_code, e.ev_value),
        0x03 /* ABS */ => linux_abs_to_stick_event(e.ev_code, e.ev_value),
        0x04 /* MSC */ => {
            if e.ev_code != 4 { // Ignore Misc./Scan Events
                let (code, val) = (e.ev_code, e.ev_value);
                eprintln!("Unknown Linux Misc Code: {}, Value: {}", code, val);
                eprintln!("Report at https://github.com/libcala/stick/issues");
            }
            None
        }
        0x15 /* FF */ => None, // Ignore Force Feedback Input Events
        _unknown => {
            eprintln!("Unknown Linux Event Type: {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            None
        }
    }
}

#[repr(C)]
struct InotifyEv {
    // struct inotify_event, from C.
    wd: c_int, /* Watch descriptor */
    mask: u32, /* Mask describing event */
    cookie: u32, /* Unique cookie associating related
               events (for rename(2)) */
    len: u32,        /* Size of name field */
    name: [u8; 256], /* Optional null-terminated name */
}

#[repr(C)]
struct TimeVal {
    // struct timeval, from C.
    tv_sec: c_long,
    tv_usec: c_long,
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
    // Though in the C header it's defined as uint, define as int because that's
    // how it's meant to be interpreted.
    ev_value: c_int,
}

#[repr(C)]
struct AbsInfo {
    // struct input_absinfo, from C.
    value: i32,
    // Though in the C header it's defined as uint32, define as int32 because
    // that's how it's meant to be interpreted.
    minimum: i32,
    // Though in the C header it's defined as uint32, define as int32 because
    // that's how it's meant to be interpreted.
    maximum: i32,
    fuzz: i32,
    flat: i32,
    resolution: i32,
}

extern "C" {
    fn strlen(s: *const u8) -> usize;

    fn open(pathname: *const u8, flags: c_int) -> c_int;
    fn read(fd: RawFd, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: RawFd, buf: *const c_void, count: usize) -> isize;
    fn close(fd: RawFd) -> c_int;
    fn fcntl(fd: RawFd, cmd: c_int, v: c_int) -> c_int;
    fn ioctl(fd: RawFd, request: c_ulong, v: *mut c_void) -> c_int;

    fn inotify_init1(flags: c_int) -> c_int;
    fn inotify_add_watch(fd: RawFd, path: *const u8, mask: u32) -> c_int;

    fn __errno_location() -> *mut c_int;
}

// FIXME: First poll should do a file search within the directory.
pub(crate) struct Hub {
    device: Device,
    read_dir: Option<Box<std::fs::ReadDir>>,
    remap: Remap,
}

impl Hub {
    pub(super) fn new(remap: Remap) -> Self {
        const CLOEXEC: c_int = 0o2000000;
        const NONBLOCK: c_int = 0o0004000;
        const CREATE: c_uint = 0x00000100;
        const DIR: &[u8] = b"/dev/input/by-id/\0";

        // Create an inotify.
        let listen = unsafe { inotify_init1(NONBLOCK | CLOEXEC) };
        if listen == -1 {
            panic!("Couldn't create inotify!");
        }

        // Start watching the controller directory.
        if unsafe { inotify_add_watch(listen, DIR.as_ptr(), CREATE) } == -1 {
            panic!("Couldn't add inotify watch!");
        }

        Hub {
            // Create watcher, and register with fd as a "device".
            device: Device::new(listen, Watcher::new().input()),
            //
            read_dir: Some(Box::new(read_dir("/dev/input/by-id/").unwrap())),
            //
            remap,
        }
    }

    // FIXME: split to disable/enable methods
    pub(super) fn enable(_flag: bool) {
        // do nothing
    }

    fn controller(
        remap: &Remap,
        mut filename: String,
    ) -> Poll<crate::Controller> {
        if filename.ends_with("-event-joystick") {
            filename.push('\0');
            let mut timeout = 1024; // Quit after 1024 tries with no access
            let fd = loop {
                timeout -= 1;
                let fd = unsafe {
                    open(filename.as_ptr(), 2 /*read&write*/)
                };
                let errno = unsafe { *__errno_location() };
                if errno != 13 || fd != -1 {
                    break fd;
                }
                if timeout == 0 {
                    break -1;
                }
            };
            if fd != -1 {
                return Poll::Ready(crate::Controller::new(
                    Ctlr::new(fd),
                    remap,
                ));
            }
        }
        Poll::Pending
    }
}

impl Future for Hub {
    type Output = crate::Controller;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = &mut *self.as_mut();

        // Read the directory for ctrls if initialization hasn't completed yet.
        if let Some(ref mut read_dir) = &mut this.read_dir {
            for dir_entry in read_dir.flatten() {
                let file = dir_entry.path();
                let path = file.as_path().to_string_lossy().to_string();
                if let Poll::Ready(controller) =
                    Self::controller(&this.remap, path)
                {
                    return Poll::Ready(controller);
                }
            }
            this.read_dir = None;
        }

        // Read the Inotify Event.
        let mut ev = MaybeUninit::<InotifyEv>::zeroed();
        let read = unsafe {
            read(
                this.device.raw(),
                ev.as_mut_ptr().cast(),
                size_of::<InotifyEv>(),
            )
        };
        if read > 0 {
            let ev = unsafe { ev.assume_init() };
            let len = unsafe { strlen(&ev.name[0]) };
            let filename = String::from_utf8_lossy(&ev.name[..len]);
            let path = format!("/dev/input/by-id/{}", filename);
            if let Poll::Ready(controller) = Self::controller(&this.remap, path)
            {
                return Poll::Ready(controller);
            }
        }

        // Register waker for this device
        this.device.register_waker(cx.waker());
        Poll::Pending
    }
}

impl Drop for Hub {
    fn drop(&mut self) {
        let fd = self.device.raw();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}

/// Gamepad / Other HID
pub(crate) struct Ctlr {
    // Async device handle
    device: Device,
    // Hexadecimal controller type ID
    id: [u8; 8],
    // Rumble effect id.
    rumble: i16,
    /// Signed axis multiplier
    norm: f64,
    /// Signed axis zero
    zero: f64,
    /// Don't process near 0
    flat: f64,
}

impl Ctlr {
    fn new(fd: c_int) -> Self {
        // Enable evdev async.
        assert_ne!(unsafe { fcntl(fd, 0x4, 0x800) }, -1);

        // Get the hardware id of this controller.
        let mut a = MaybeUninit::<InputId>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x_8008_4502, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        // Convert raw integers from the linux kernel to endian-independant ids
        let bustype = a.bustype.to_be_bytes();
        let vendor = a.vendor.to_be_bytes();
        let product = a.product.to_be_bytes();
        let version = a.version.to_be_bytes();
        let id = [
            bustype[0], bustype[1], vendor[0], vendor[1], product[0],
            product[1], version[0], version[1],
        ];

        // Get the min and max absolute values for axis.
        let mut a = MaybeUninit::<AbsInfo>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x_8018_4540, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        let norm = (a.maximum as f64 - a.minimum as f64) * 0.5;
        let zero = a.minimum as f64 + norm;
        // Invert so multiplication can be used instead of division
        let norm = norm.recip();
        let flat = a.flat as f64 * norm;

        // Query the controller for haptic support.
        let rumble = joystick_haptic(fd, -1, 0.0, 0.0);
        // Construct device from fd, looking for input events.
        let device = Device::new(fd, Watcher::new().input());
        // Return
        Self {
            device,
            id,
            rumble,
            norm,
            zero,
            flat,
        }
    }

    pub(super) fn id(&self) -> [u8; 8] {
        self.id
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
        // Read an event.
        let mut ev = MaybeUninit::<EvdevEv>::uninit();
        let ev = {
            let bytes = unsafe {
                read(
                    self.device.raw(),
                    ev.as_mut_ptr().cast(),
                    size_of::<EvdevEv>(),
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
            assert_eq!(size_of::<EvdevEv>() as isize, bytes);
            unsafe { ev.assume_init() }
        };

        // Convert the event.
        if let Some(event) = linux_evdev_to_stick_event(ev) {
            Poll::Ready(event)
        } else {
            self.poll(cx)
        }
    }

    pub(super) fn name(&self) -> String {
        let fd = self.device.raw();
        let mut a = MaybeUninit::<[c_char; 256]>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x80FF_4506, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        let name = unsafe { std::ffi::CStr::from_ptr(a.as_ptr()) };
        name.to_string_lossy().to_string()
    }

    pub(super) fn rumble(&mut self, left: f32, right: f32) {
        if self.rumble >= 0 {
            joystick_ff(self.device.raw(), self.rumble, left, right);
        }
    }

    pub(super) fn pressure(&self, input: f64) -> f64 {
        input * (1.0 / 255.0)
    }

    pub(super) fn axis(&self, input: f64) -> f64 {
        let input = (input - self.zero) * self.norm;
        if input.abs() <= self.flat {
            0.0
        } else {
            input
        }
    }
}

impl Drop for Ctlr {
    fn drop(&mut self) {
        let fd = self.device.raw();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}

// From: https://github.com/torvalds/linux/blob/master/include/uapi/linux/input.h

#[repr(C)]
struct FfTrigger {
    button: u16,
    interval: u16,
}

#[repr(C)]
struct FfReplay {
    length: u16,
    delay: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfEnvelope {
    attack_length: u16,
    attack_level: u16,
    fade_length: u16,
    fade_level: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfConstantEffect {
    level: i16,
    envelope: FfEnvelope,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfRampEffect {
    start_level: i16,
    end_level: i16,
    envelope: FfEnvelope,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfPeriodicEffect {
    waveform: u16,
    period: u16,
    magnitude: i16,
    offset: i16,
    phase: u16,

    envelope: FfEnvelope,

    custom_len: u32,
    custom_data: *mut i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfConditionEffect {
    right_saturation: u16,
    left_saturation: u16,

    right_coeff: i16,
    left_coeff: i16,

    deadband: u16,
    center: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FfRumbleEffect {
    strong_magnitude: u16,
    weak_magnitude: u16,
}

#[repr(C)]
union FfUnion {
    constant: FfConstantEffect, // Not supported.
    ramp: FfRampEffect,
    periodic: FfPeriodicEffect,
    condition: [FfConditionEffect; 2], /* One for each axis */
    rumble: FfRumbleEffect,            // Not supported
}

#[repr(C)]
struct FfEffect {
    stype: u16,
    id: i16,
    direction: u16,

    trigger: FfTrigger,
    replay: FfReplay,

    u: FfUnion,
}

fn joystick_ff(fd: RawFd, code: i16, strong: f32, weak: f32) {
    // Update haptic effect `code`.
    if strong != 0.0 || weak != 0.0 {
        joystick_haptic(fd, code, strong, weak);
    }
    //
    let ev_code = code.try_into().unwrap();

    let play = &EvdevEv {
        ev_time: TimeVal {
            tv_sec: 0,
            tv_usec: 0,
        },
        ev_type: 0x15, /*EV_FF*/
        ev_code,
        ev_value: (strong > 0.0 || weak > 0.0) as _,
    };
    let play: *const _ = play;
    unsafe {
        if write(fd, play.cast(), size_of::<EvdevEv>())
            != size_of::<EvdevEv>() as isize
        {
            let errno = *__errno_location();
            if errno != 19 {
                // 19 = device unplugged, ignore
                panic!("Write exited with {}", *__errno_location());
            }
        }
    }
}

// Get ID's for rumble and vibrate, if they're supported (otherwise, -1).
fn joystick_haptic(fd: RawFd, id: i16, strong: f32, weak: f32) -> i16 {
    let a = &mut FfEffect {
        stype: 0x50,
        id, /*allocate new effect*/
        direction: 0,
        trigger: FfTrigger {
            button: 0,
            interval: 0,
        },
        replay: FfReplay {
            length: 0,
            delay: 0,
        },
        u: FfUnion {
            rumble: FfRumbleEffect {
                strong_magnitude: (u16::MAX as f32 * strong) as u16,
                weak_magnitude: (u16::MAX as f32 * weak) as u16,
            },
        },
    };
    let b: *mut _ = a;
    if unsafe { ioctl(fd, 0x40304580, b.cast()) } == -1 {
        -1
    } else {
        a.id
    }
}
