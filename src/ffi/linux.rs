// Stick
//
// Copyright (c) 2017-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use smelling_salts::{Device as AsyncDevice, Watcher};

use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
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

struct PortTimer {
    device: AsyncDevice,
}

impl PortTimer {
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
    emulated: u8, // lower 4 bits are for D-pad.
    rumble: i16,
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
        // Query the controller for haptic support.
        let rumble = joystick_haptic(fd, -1, 0.0);
        // Construct device from fd, looking for input events.
        Gamepad {
            hardware_id,
            abs_min,
            abs_range,
            queued: None,
            device: AsyncDevice::new(fd, Watcher::new().input()),
            emulated: 0,
            rumble,
        }
    }

    fn to_float(&self, value: u32) -> f32 {
        (value as i32) as f32 * 0.005
    }

    // Apply mods
    fn apply_mods(&self, mut event: Event) -> Event {
        // Deadzone multiply
        let dm = if self.hardware_id == 0x_07B5_0316 {
            2.0
        } else {
            1.0
        };

        let s = |x: f32| {
            // Scale based on advertized min and max values
            let v = ((200.0 * x) - self.abs_min as f32) / self.abs_range as f32;
            // Noise Filter
            let v = (200.0 * v).trunc() / 199.0;
            // Deadzone
            if (v - 0.5).abs() < dm * 0.0625 {
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
                    event = Event::Lz((v * 2.0 - 0.5).min(1.0).max(0.0));
                } else {
                    event =
                        Event::CameraH((s(v) * 2.0 - 1.0).min(1.0).max(-1.0));
                }
            }
            Event::CameraV(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event = Event::Rz((v * 2.0 - 0.5).min(1.0).max(0.0));
                } else {
                    event =
                        Event::CameraV((s(v) * 2.0 - 1.0).min(1.0).max(-1.0))
                }
            }
            Event::Lz(v) => {
                if self.hardware_id == 0x_0079_1844 {
                    // GameCube
                    event =
                        Event::CameraV((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
                } else if self.hardware_id == 0x_07B5_0316 {
                    // Flight Controller
                    event = Event::Lz((v + 0.5).min(1.0).max(0.0));
                } else {
                    event = Event::Lz(v.min(1.0).max(0.0));
                }
            }
            Event::Rz(v) => {
                if self.hardware_id == 0x_0079_1844
                /*gc*/
                {
                    event =
                        Event::CameraH((s(v) * 4.0 - 2.0).min(1.0).max(-1.0));
                } else {
                    event = Event::Rz(v.min(1.0).max(0.0))
                }
            }
            _ => {}
        }
        event
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
                self.queued = Some(Event::Left(true));
                Event::Right(false)
            } else {
                Event::Left(true)
            }
        } else if value > 0 {
            // Right
            if emulated & right != 0 {
                return None;
            }
            self.emulated |= right;
            if emulated & left != 0 {
                self.emulated &= !left;
                self.queued = Some(Event::Right(true));
                Event::Left(false)
            } else {
                Event::Right(true)
            }
        } else {
            self.emulated &= !(left | right);
            if emulated & left != 0 {
                Event::Left(false)
            } else if emulated & right != 0 {
                Event::Right(false)
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
                self.queued = Some(Event::Up(true));
                Event::Down(false)
            } else {
                Event::Up(true)
            }
        } else if value > 0 {
            // Down
            if emulated & down != 0 {
                return None;
            }
            self.emulated |= down;
            if emulated & up != 0 {
                self.emulated &= !up;
                self.queued = Some(Event::Down(true));
                Event::Up(false)
            } else {
                Event::Down(true)
            }
        } else {
            self.emulated &= !(up | down);
            if emulated & up != 0 {
                Event::Up(false)
            } else if emulated & down != 0 {
                Event::Down(false)
            } else {
                return None;
            }
        })
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
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
                        eprintln!(
                            "Button 10 is Unknown, report at \
                            https://github.com/libcala/stick/issues"
                        );
                        return self.poll(cx);
                    }
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
                    // 16-17 already matched
                    18 => {
                        eprintln!(
                            "Button 18 is Unknown, report at \
                            https://github.com/libcala/stick/issues"
                        );
                        return self.poll(cx);
                    }
                    // 19-20 already matched
                    21 => {
                        eprintln!(
                            "Button 21 is Unknown, report at \
                            https://github.com/libcala/stick/issues"
                        );
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
                        eprintln!(
                            "Button {} is Unknown, report at \
                            https://github.com/libcala/stick/issues",
                            a
                        );
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
                    16 => {
                        if let Some(event) = self.dpad_h(ev.ev_value as c_int) {
                            event
                        } else {
                            return self.poll(cx);
                        }
                    }
                    17 => {
                        if let Some(event) = self.dpad_v(ev.ev_value as c_int) {
                            event
                        } else {
                            return self.poll(cx);
                        }
                    }
                    40 => return self.poll(cx), // IGNORE: Duplicate axis.
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
            // ignore autorepeat, relative.
            _ => return self.poll(cx),
        };

        Poll::Ready(self.apply_mods(event))
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

impl Drop for Gamepad {
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
    if is_powered {
        joystick_haptic(fd, code, value);
    }

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
            panic!("Write exited with {}", *__errno_location());
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
                period: 0,                           /*milliseconds*/
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
    let rumble = if unsafe { ioctl(fd, 0x40304580, b.cast()) } == -1 {
        -1
    } else {
        a.id
    };
    rumble
}
