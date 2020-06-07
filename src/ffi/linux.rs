// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use smelling_salts::{Device as AsyncDevice, Watcher};

use std::{
    collections::HashSet,
    convert::TryInto,
    fs::{self, File, OpenOptions},
    io::ErrorKind,
    mem::MaybeUninit,
    os::{
        raw::{c_char, c_int, c_long, c_ulong, c_ushort, c_void},
        unix::io::{IntoRawFd, RawFd},
    },
    task::{Context, Poll},
};

use crate::Event;

// This input offset when subtracted, gives a platform-agnostic button ID.
// Since Stick only looks for gamepads and joysticks, button IDs below this
// number shouldn't occur.
const LINUX_SPECIFIC_BTN_OFFSET: c_ushort = 0x120;

/// Evdev hat data.
struct PadStateHat {
    hor: c_int,
    ver: c_int,
}

/// Data associated with the state of the pad.  Used to produce the correct
/// platform-agnostic events.
struct PadState {
    // Trigger state
    trigger_l: Option<f32>,
    // Trigger state
    trigger_r: Option<f32>,
    // Minimum axis value
    min: f64,
    // Maximum axis value - Minimum axis value
    range: f64,
    queued: Option<Event>,
    hats: Vec<PadStateHat>,
}

/// Describes some hardware joystick mapping
struct PadDescriptor {
    // Pad name
    name: &'static str,
    
    // (Axis) value = Full range min to max axis
    axes: &'static [(&'static dyn Fn(f64) -> Event, c_ushort, Option<c_int>)],
    // (Button) value = Boolean 1 or 0
    buttons: &'static [(&'static dyn Fn(bool) -> Event, c_ushort)],
    // (Button) value = 0.0f64 or 1.0f64
    trigbtns: &'static [(&'static dyn Fn(f64) -> Event, c_ushort)],
    // (Axis) value = 0 thru 255
    triggers: &'static [(&'static dyn Fn(f64) -> Event, c_ushort, Option<c_int>)],
    // (Axis) value[0] = -1, 0, or 1; value[1] = -1, 0, or 1
    three_ways: &'static [(&'static dyn Fn(bool, bool) -> Event, c_ushort)],
    // (RelativeAxis) value = Full range min to max axis
    wheels: &'static [(&'static dyn Fn(f64) -> Event, c_ushort)],
}

impl PadDescriptor {
    // Convert evdev event into JoyPush event.
    fn event_from(&self, ev: EvdevEv, state: &mut PadState) -> Option<Event> {
        let joyaxis_float = |x| (x as f64 - state.min) / state.range;
        let trigger_float = |x| x as f64 / 255.0;

        match ev.ev_type {
            0x00 => { /* Ignore SYN events. */ }
            0x01 => {
                // button press / release (key)
                let ev_code = ev.ev_code.checked_sub(LINUX_SPECIFIC_BTN_OFFSET)
                    .expect(&format!("Out of range ev_code: {}, report at \
                        https://github.com/libcala/stick/issues", ev.ev_code));
                for (new, evcode) in self.buttons {
                    if ev_code == *evcode {
                        return Some(new(ev.ev_value != 0));
                    }
                }
                for (new, evcode) in self.trigbtns {
                    if ev_code == *evcode {
                        return Some(new(if ev.ev_value != 0 { 1.0 } else { 0.0 }));
                    }
                }
                eprintln!(
                    "*Evdev* Unknown Button Code: {}, report at \
                    https://github.com/libcala/stick/issues",
                    ev_code
                );
            }
            0x02 => {
                // Relative axis movement
                for (new, evcode) in self.wheels {
                    if ev.ev_code == *evcode {
                        return Some(new(joyaxis_float(ev.ev_value)));
                    }
                }
                eprintln!(
                    "*Evdev* Unknown Relative Axis Code: {}, report at \
                    https://github.com/libcala/stick/issues",
                    ev.ev_code
                );
            }
            0x03 => {
                // Absolute axis movement
                for (new, evcode) in self.axes {
                    if ev.ev_code == *evcode {
                        return Some(new(joyaxis_float(ev.ev_value)));
                    }
                }
                for (new, evcode) in self.triggers {
                    if ev.ev_code == *evcode {
                        return Some(new(trigger_float(ev.ev_value)));
                    }
                }
                for (i, [(up, down, vercode), (left, right, horcode)]) in
                    self.hats.iter().enumerate()
                {
                    match ev.ev_code {
                        _x if _x == *vercode => {
                            let hat_value = state.hats[i].ver;
                            state.hats[i].ver = ev.ev_value;
                            return match ev.ev_value {
                                -1 => match hat_value {
                                    -1 => None,
                                    1 => {
                                        state.queued = Some(up(true));
                                        Some(down(false))
                                    }
                                    0 => Some(up(true)),
                                    _ => unreachable!(),
                                },
                                0 => match hat_value {
                                    -1 => Some(up(false)),
                                    1 => Some(down(false)),
                                    0 => None,
                                    _ => unreachable!(),
                                },
                                1 => match hat_value {
                                    -1 => {
                                        state.queued = Some(down(true));
                                        Some(up(false))
                                    }
                                    1 => None,
                                    0 => Some(down(true)),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                        }
                        _x if _x == *horcode => {
                            let hat_value = state.hats[i].hor;
                            state.hats[i].hor = ev.ev_value;
                            return match ev.ev_value {
                                -1 => match hat_value {
                                    -1 => None,
                                    1 => {
                                        state.queued = Some(left(true));
                                        Some(right(false))
                                    }
                                    0 => Some(left(true)),
                                    _ => unreachable!(),
                                },
                                0 => match hat_value {
                                    -1 => Some(left(false)),
                                    1 => Some(right(false)),
                                    0 => None,
                                    _ => unreachable!(),
                                },
                                1 => match hat_value {
                                    -1 => {
                                        state.queued = Some(right(true));
                                        Some(left(false))
                                    }
                                    1 => None,
                                    0 => unreachable!(),
                                    _ => Some(right(true)),
                                },
                                _ => unreachable!(),
                            };
                        }
                        _ => { /* Keep looking */ }
                    }
                }
                eprintln!(
                    "*Evdev* Unknown Absolute Axis Code: {}, report at \
                    https://github.com/libcala/stick/issues",
                    ev.ev_code
                );
            }
            0x04 => {
                if ev.ev_code != /* scan */ 4 {
                    eprintln!(
                        "*Evdev* Unknown Misc Code: {} value: {}, report \
                        at https://github.com/libcala/stick/issues",
                        ev.ev_code, ev.ev_value
                    );
                }
            }
            0x15 => { /* Force Feedback echo, ignore */ }
            u => {
                eprintln!(
                    "*Evdev* Unknown Event: {}, Code: {} value: {}, \
                    report at https://github.com/libcala/stick/issues.",
                    u, ev.ev_code, ev.ev_value
                );
            }
        };
        None
    }
}

include!(concat!(env!("OUT_DIR"), "/database.rs"));

const HARDWARE_ID_MAYFLASH_ARCADE_FIGHTSTICK_PS3_COMPAT: u32 = 0x_7900_3018;
const HARDWARE_ID_MAYFLASH_GAMECUBE: u32 = 0x_7900_4418;

const HARDWARE_ID_THRUSTMASTER2: u32 = 0x_4F04_0AB1;

const HARDWARE_ID_PLAYSTATION_LOGITECH: u32 = 0x_6D04_16C2;

const HARDWARE_ID_SIXAXIS_PS3: u32 = 0x_4C05_6802;
const HARDWARE_ID_DUALSHOCK_PS4: u32 = 0x_4C05_C405;

const HARDWARE_ID_MAD_CATZ_RAT_MOUSE: u32 = 0x_3807_1817;

const HARDWARE_ID_THRUSTMASTER1: u32 = 0x_B507_1603;

const HARDWARE_ID_XBOX_PDP: u32 = 0x_6F0E_A802;

const HARDWARE_ID_SPEEDLINK_PS3_COMPAT: u32 = 0x_8F0E_7530;
const HARDWARE_ID_AFTERGLOW_PS3_COMPAT: u32 = 0x_6F0E_0263;

struct HardwareId(u32);

impl HardwareId {
    fn is_playstation_official(&self) -> bool {
        self.0 == HARDWARE_ID_SIXAXIS_PS3 || self.0 == HARDWARE_ID_DUALSHOCK_PS4
    }

    fn is_playstation_compat(&self) -> bool {
        self.0 == HARDWARE_ID_SPEEDLINK_PS3_COMPAT
            || self.0 == HARDWARE_ID_AFTERGLOW_PS3_COMPAT
            || self.0 == HARDWARE_ID_MAYFLASH_ARCADE_FIGHTSTICK_PS3_COMPAT
    }

    fn is_gamecube(&self) -> bool {
        self.0 == HARDWARE_ID_MAYFLASH_GAMECUBE
    }

    fn is_mouse(&self) -> bool {
        self.0 == HARDWARE_ID_MAD_CATZ_RAT_MOUSE
    }

    fn is_thrustmaster(&self) -> bool {
        self.0 == HARDWARE_ID_THRUSTMASTER1
            || self.0 == HARDWARE_ID_THRUSTMASTER2
    }

    fn is_playstation_logitech(&self) -> bool {
        self.0 == HARDWARE_ID_PLAYSTATION_LOGITECH
    }
}

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
    // Though in the C header it's defined as uint, define as int because that's
    // how it's meant to be interpreted.
    ev_value: c_int,
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
    fn write(fd: RawFd, buf: *const c_void, count: usize) -> isize;
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

struct HubTimer {
    device: AsyncDevice,
}

impl HubTimer {
    fn new(cx: &mut Context<'_>) -> Self {
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
        HubTimer { device }
    }
}

impl Drop for HubTimer {
    fn drop(&mut self) {
        let fd = self.device.fd();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}

pub(crate) struct Hub {
    device: AsyncDevice,
    connected: HashSet<String>,
    timer: Option<HubTimer>,
}

impl Hub {
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
        Hub {
            device,
            connected,
            timer,
        }
    }

    pub(super) fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<(usize, Event)> {
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
                        let fd = match OpenOptions::new()
                            .read(true)
                            .append(true)
                            .open(filename)
                        {
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
                            std::usize::MAX,
                            Event::Connect(Box::new(crate::Pad(Pad::new(fd)))),
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
                // Remove it if it exists, sometimes gamepads get "removed"
                // twice because adds are condensed in innotify (not 100% sure).
                let _ = self.connected.remove(&file);
            }
        }
        // Add flag is set, wait for permissions (unfortunately, can't rely on
        // epoll events for this, so check every 10 milliseconds).
        if (ev.mask & 0x0000_0100) != 0 && self.timer.is_none() {
            self.timer = Some(HubTimer::new(cx));
        }
        // Check for more events, Search for new controllers again, and return
        // Pending if neither have anything to process.
        self.poll(cx)
    }
}

impl Drop for Hub {
    fn drop(&mut self) {
        let fd = self.device.fd();
        self.device.old();
        assert_ne!(unsafe { close(fd) }, -1);
    }
}

/// Gamepad / Other HID
pub(crate) struct Pad {
    device: AsyncDevice,
    hardware_id: u32, // Which type of controller?
    abs_min: c_int,
    abs_range: c_int,
    queued: Option<Event>,
    emulated: u8, // lower 4 bits are for D-pad.
    rumble: i16,
    movx: f64,
    movy: f64,
    camx: f64,
    camy: f64,
    lt: f64,
    rt: f64,
}

impl Pad {
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
            ((u32::from(a.vendor.to_be())) << 16) | (u32::from(a.product.to_be()));
        // Get the min and max absolute values for axis.
        let mut a = MaybeUninit::<AbsInfo>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x_8018_4540, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        let abs_min = a.minimum as c_int;
        let abs_range = a.maximum as c_int - a.minimum as c_int;
        // Query the controller for haptic support.
        let rumble = joystick_haptic(fd, -1, 0.0);
        // Construct device from fd, looking for input events.
        Pad {
            hardware_id,
            abs_min,
            abs_range,
            queued: None,
            device: AsyncDevice::new(fd, Watcher::new().input()),
            emulated: 0,
            rumble,
            movx: 0.0,
            movy: 0.0,
            camx: 0.0,
            camy: 0.0,
            lt: 0.0,
            rt: 0.0,
        }
    }

    // Convert value as though it were a trigger axis (returns 0 to 1).
    fn trigger_float(&self, x: c_int) -> f64 {
        // Deadzone multiply
        let dm = match self.hardware_id {
            HARDWARE_ID_MAYFLASH_GAMECUBE => 2.0,
            _ => 1.0,
        };

        let scale = match self.hardware_id {
            HARDWARE_ID_XBOX_PDP => 0.25,
            _ => 1.0,
        };

        let x = (x as f64 * scale / 255.0).min(1.0).max(0.0);
        if x < dm * 0.075 {
            0.0
        } else {
            x
        }
    }

    // Convert value as though it were a joystick axis (returns -1 to 1).
    fn joyaxis_float(&self, x: c_int) -> f64 {
        // Deadzone multiply
        let dm = if self.hardware_id == HARDWARE_ID_THRUSTMASTER1 {
            2.0
        } else {
            1.0
        };

        let scale = if self.hardware_id == HARDWARE_ID_MAYFLASH_GAMECUBE {
            1.5
        } else {
            1.0
        };

        let x = (x - self.abs_min) as f64 / self.abs_range as f64;
        // Deadzone
        if (x - 0.5).abs() < dm * 0.0625 {
            0.0
        } else {
            (x * 2.0 * scale - scale).min(1.0).max(-1.0)
        }
    }

    pub(super) fn id(&self) -> u32 {
        self.hardware_id
    }

    fn dpad_h(&mut self, value: c_int) -> Option<Event> {
        let emulated = self.emulated;
        let left = 0b0000_0001;
        let right = 0b0000_0010;
        Some(if value < 0 {
            // Left
            if emulated & left != 0 {
                return None;
            }
            self.emulated |= left;
            if emulated & right != 0 {
                self.emulated &= !right;
                self.queued = Some(Event::DpadLeft(true));
                Event::DpadRight(false)
            } else {
                Event::DpadLeft(true)
            }
        } else if value > 0 {
            // Right
            if emulated & right != 0 {
                return None;
            }
            self.emulated |= right;
            if emulated & left != 0 {
                self.emulated &= !left;
                self.queued = Some(Event::DpadRight(true));
                Event::DpadLeft(false)
            } else {
                Event::DpadRight(true)
            }
        } else {
            self.emulated &= !(left | right);
            if emulated & left != 0 {
                Event::DpadLeft(false)
            } else if emulated & right != 0 {
                Event::DpadRight(false)
            } else {
                return None;
            }
        })
    }

    fn dpad_v(&mut self, value: c_int) -> Option<Event> {
        let emulated = self.emulated;
        let up = 0b0000_0100;
        let down = 0b0000_1000;
        Some(if value < 0 {
            // Up
            if emulated & up != 0 {
                return None;
            }
            self.emulated |= up;
            if emulated & down != 0 {
                self.emulated &= !down;
                self.queued = Some(Event::DpadUp(true));
                Event::DpadDown(false)
            } else {
                Event::DpadUp(true)
            }
        } else if value > 0 {
            // Down
            if emulated & down != 0 {
                return None;
            }
            self.emulated |= down;
            if emulated & up != 0 {
                self.emulated &= !up;
                self.queued = Some(Event::DpadDown(true));
                Event::DpadUp(false)
            } else {
                Event::DpadDown(true)
            }
        } else {
            self.emulated &= !(up | down);
            if emulated & up != 0 {
                Event::DpadUp(false)
            } else if emulated & down != 0 {
                Event::DpadDown(false)
            } else {
                return None;
            }
        })
    }

    // Trigger axis disabled when trigger button pressed?
    fn tad(&self) -> bool {
        let hwid = HardwareId(self.hardware_id);
        hwid.is_gamecube()
    }

    fn remapping(&self, mut id: u16) -> u16 {
        let hwid = HardwareId(self.hardware_id);

        // Swap Accept and Cancel Buttons, Action and Common Buttons
        if hwid.is_playstation_official() {
            id = match id {
                17 => 16, // Accept <-> Cancel
                16 => 17, // Accept <-> Cancel
                19 => 20, // Common <-> Action
                20 => 19, // Common <-> Action
                x => x,
            };
        } else if hwid.is_playstation_logitech() {
            id = match id {
                17 => 16, // Accept <-> Cancel
                16 => 17, // Accept <-> Cancel
                19 => 20, // Common <-> Action
                20 => 19, // Common <-> Action
                4 => 6,   // LT <-> L
                6 => 4,   // LT <-> L
                5 => 7,   // RT <-> R
                7 => 5,   // RT <-> R
                x => x,
            };
        } else if hwid.is_playstation_compat() {
            id = match id {
                17 => 16, // "Cancel" -> Accept
                16 => 20, // "Accept" -> Action
                20 => 22, // "Action" -> L
                22 => 24, // "L" -> LT
                23 => 25, // "R" -> RT
                24 => 26, // "LT" -> Back
                25 => 27, // "RT" -> Forward
                26 => 29, // "Back" -> MotionJoyPush
                27 => 30, // "Forward" -> CameraJoyPush
                x => x,
            };
        } else if hwid.is_mouse() {
            id = match id {
                3 => 26, // "Common" -> Back
                4 => 27, // "Lt" -> Forward
                5 => 4,  // "Rt" -> "Lt"
                x => x,
            };
        } else if hwid.is_thrustmaster() {
            id = match id {
                // Left Custom Buttons
                4 => 48, // Guess
                5 => 50, // Guess
                6 => 52, // Guess
                7 => 54, // Guess
                8 => 56, // Guess
                9 => 58, // Guess
                // Right Custom Buttons
                10 => 49, // Guess
                11 => 51, // Guess
                12 => 53, // Guess
                13 => 55, // Guess
                14 => 57, // Guess
                15 => 59, // Guess
                x => x,
            };
        }

        id
    }

    fn axis_remapping(&self, mut id: u16) -> u16 {
        let hwid = HardwareId(self.hardware_id);

        // Swap axis on GameCube & Speedlink
        if hwid.is_gamecube()
            || hwid.is_playstation_compat()
            || hwid.is_thrustmaster()
            || hwid.is_playstation_logitech()
        {
            id = match id {
                2 => 4,
                5 => 3,
                3 => 2,
                4 => 5,
                x => x,
            };
        } else if hwid.is_mouse() {
            id = match id {
                0 => 3, // Mouse movement -> Camera
                1 => 4, // Mouse movement -> Camera
                5 => 0, // Scroll Wheel Horizontal -> Movement
                x => x,
            };
        }

        id
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
        if let Some(event) = self.queued.take() {
            return Poll::Ready(event);
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

        // Convert the event.
        let event = match ev.ev_type {
            0x00 => return self.poll(cx), // Ignore SYN events.
            // button press / release (key)
            0x01 => {
                let is = ev.ev_value == 1;

                match self.remapping(ev.ev_code - LINUX_SPECIFIC_BTN_OFFSET) {
                    // Fallback Event IDs
                    0 | 20 => Event::ActionV(is),
                    1 | 16 => Event::ActionA(is),
                    2 | 17 | 18 => Event::ActionB(is),
                    3 | 19 => Event::ActionH(is),
                    4 | 24 => Event::TriggerL(if is {
                        self.emulated |= 0b0001_0000;
                        1.0
                    } else {
                        self.emulated &= !0b0001_0000;
                        self.lt
                    }),
                    5 | 25 => Event::TriggerR(if is {
                        self.emulated |= 0b0010_0000;
                        1.0
                    } else {
                        self.emulated &= !0b0010_0000;
                        self.rt
                    }),
                    6 | 22 => Event::BumperL(is), // 6 is Guess
                    7 | 23 | 21 => Event::BumperR(is),
                    8 | 26 => Event::Prev(is), // 8 is Guess
                    9 | 27 => Event::Next(is),
                    10 | 29 => Event::JoyPush(is),
                    11 | 30 => Event::PovPush(is),
                    // D-PAD
                    12 | 256 => {
                        if let Some(ev) = self.dpad_v(if is { -1 } else { 0 }) {
                            ev
                        } else {
                            return self.poll(cx);
                        }
                    }
                    13 | 259 => {
                        if let Some(ev) = self.dpad_h(if is { 1 } else { 0 }) {
                            ev
                        } else {
                            return self.poll(cx);
                        }
                    }
                    14 | 257 => {
                        if let Some(ev) = self.dpad_v(if is { 1 } else { 0 }) {
                            ev
                        } else {
                            return self.poll(cx);
                        }
                    }
                    15 | 258 => {
                        if let Some(ev) = self.dpad_h(if is { -1 } else { 0 }) {
                            ev
                        } else {
                            return self.poll(cx);
                        }
                    }
                    28 => {
                        if is {
                            Event::Home
                        } else {
                            return self.poll(cx);
                        }
                    }
                    // 31 thru 47 are unknown
                    48 => Event::Action(0, is),
                    49 => Event::Action(1, is),
                    50 => Event::Action(2, is), // Guess
                    51 => Event::Action(3, is), // Guess
                    52 => Event::Action(4, is), // Guess
                    53 => Event::Action(5, is), // Guess
                    54 => Event::Action(6, is), // Guess
                    55 => Event::Action(7, is), // Guess
                    56 => Event::Action(8, is), // Guess
                    57 => Event::Action(9, is), // Guess
                    58 => Event::Action(10, is), // Guess
                    59 => Event::Action(11, is), // Guess
                    // 60 thru 255 are unknown
                    a => {
                        eprintln!(
                            "Button {} is Unknown, report at \
                            https://github.com/libcala/stick/issues",
                            a
                        );
                        return self.poll(cx);
                    }
                }
            }
            // Relative axis movement
            0x02 => match ev.ev_code {
                8 => Event::JoyY({
                    let value = self.joyaxis_float(ev.ev_value);
                    if value == self.movy {
                        return self.poll(cx);
                    }
                    self.movy = value;
                    value
                }),
                u => {
                    eprintln!("Unknown Relative Axis {}", u);
                    return self.poll(cx);
                }
            },
            // Absolute axis movement (abs)
            0x03 => {
                match self.axis_remapping(ev.ev_code) {
                    0 => Event::JoyX({
                        let value = self.joyaxis_float(ev.ev_value);
                        if value == self.movx {
                            return self.poll(cx);
                        }
                        self.movx = value;
                        value
                    }),
                    1 => Event::JoyY({
                        let value = self.joyaxis_float(ev.ev_value);
                        if value == self.movy {
                            return self.poll(cx);
                        }
                        self.movy = value;
                        value
                    }),
                    21 | 2 => Event::TriggerL({
                        let old = self.lt;
                        self.lt = self.trigger_float(ev.ev_value);
                        if (self.emulated & 0b0001_0000 != 0 && self.tad())
                            || old == self.lt
                        {
                            return self.poll(cx);
                        }
                        self.lt
                    }),
                    3 => Event::PovX({
                        let value =
                            if HardwareId(self.hardware_id).is_thrustmaster() {
                                self.trigger_float(ev.ev_value) * 2.0 - 1.0
                            } else {
                                self.joyaxis_float(ev.ev_value)
                            };
                        if value == self.camx {
                            return self.poll(cx);
                        }
                        self.camx = value;
                        value
                    }),
                    4 => Event::PovY({
                        let value = self.joyaxis_float(ev.ev_value);
                        if value == self.camy {
                            return self.poll(cx);
                        }
                        self.camy = value;
                        value
                    }),
                    6 => Event::PovY({
                        let value = self.trigger_float(ev.ev_value) * 2.0 - 1.0;
                        if value == self.camy {
                            return self.poll(cx);
                        }
                        self.camy = value;
                        value
                    }),
                    20 | 5 => Event::TriggerR({
                        let old = self.rt;
                        self.rt = self.trigger_float(ev.ev_value);
                        if (self.emulated & 0b0010_0000 != 0 && self.tad())
                            || old == self.rt
                        {
                            return self.poll(cx);
                        }
                        self.rt
                    }),
                    16 => {
                        if let Some(event) = self.dpad_h(ev.ev_value) {
                            event
                        } else {
                            return self.poll(cx);
                        }
                    }
                    17 => {
                        if let Some(event) = self.dpad_v(ev.ev_value) {
                            event
                        } else {
                            return self.poll(cx);
                        }
                    }
                    40 | 11 => return self.poll(cx), // IGNORE: Duplicate axis.
                    a => {
                        eprintln!(
                            "Unknown Axis: {}, report at \
                            https://github.com/libcala/stick/issues",
                            a
                        );
                        return self.poll(cx);
                    }
                }
            }
            0x04 => {
                if ev.ev_code == 4
                /* scan */
                { /* ignore */
                } else {
                    eprintln!("Misc {} {}.", ev.ev_code, ev.ev_value);
                }
                return self.poll(cx);
            }
            0x15 => {
                // Force Feedback echo, ignore
                return self.poll(cx);
            }
            u => {
                eprintln!("Unknown {} {} {}.", u, ev.ev_code, ev.ev_value);
                return self.poll(cx);
            }
        };

        Poll::Ready(event)
    }

    pub(super) fn name(&self) -> String {
        let fd = self.device.fd();
        let mut a = MaybeUninit::<[c_char; 256]>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x80FF_4506, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        let name = unsafe { std::ffi::CStr::from_ptr(a.as_ptr()) };
        name.to_string_lossy().to_string()
    }

    pub(super) fn rumble(&mut self, v: f32) {
        if self.rumble >= 0 {
            joystick_ff(self.device.fd(), self.rumble, v);
        }
    }
}

impl Drop for Pad {
    fn drop(&mut self) {
        let fd = self.device.fd();
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

fn joystick_ff(fd: RawFd, code: i16, value: f32) {
    let is_powered = value != 0.0;
    let new_id = if is_powered {
        joystick_haptic(fd, code, value)
    } else {
        -3
    };

    let ev_code = code.try_into().unwrap();

    let play = &EvdevEv {
        ev_time: TimeVal {
            tv_sec: 0,
            tv_usec: 0,
        },
        ev_type: 0x15, /*EV_FF*/
        ev_code,
        ev_value: if is_powered { 1 } else { 0 },
    };
    let play: *const _ = play;
    unsafe {
        if write(fd, play.cast(), std::mem::size_of::<EvdevEv>())
            != std::mem::size_of::<EvdevEv>() as isize
        {
            let errno = *__errno_location();
            if errno != 19
            /* 19 = device unplugged, ignore */
            {
                panic!(
                    "Write {:?} exited with {}",
                    (code, new_id),
                    *__errno_location()
                );
            }
        }
    }
}

// Get ID's for rumble and vibrate, if they're supported (otherwise, -1).
fn joystick_haptic(fd: RawFd, id: i16, power: f32) -> i16 {
    let a = &mut FfEffect {
        stype: 0x51,
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
            periodic: FfPeriodicEffect {
                waveform: 0x5a,                      /*sine wave*/
                period: 100,                         /*milliseconds*/
                magnitude: (32767.0 * power) as i16, /*peak value*/
                offset: 0,                           /*mean value of wave*/
                phase: 0,                            /*horizontal shift*/
                envelope: FfEnvelope {
                    attack_length: 0,
                    attack_level: 0,
                    fade_length: 0,
                    fade_level: 0,
                },
                custom_len: 0,
                custom_data: std::ptr::null_mut(),
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
