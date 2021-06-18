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

use crate::Event;
use smelling_salts::{Device, Watcher};
use std::convert::TryInto;
use std::fs::read_dir;
use std::future::Future;
use std::mem::{size_of, MaybeUninit};
use std::num::FpCategory;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
use std::os::unix::io::RawFd;
use std::pin::Pin;
use std::str;
use std::task::{Context, Poll};

// This input offset when subtracted, gives a platform-agnostic button ID.
// Since Stick only looks for gamepads and joysticks, button IDs below this
// number shouldn't occur.
const LINUX_SPECIFIC_BTN_OFFSET: c_ushort = 0x120;

/// State of a hat or dpad in order to remove duplicated events, because
/// sometimes evdev produces both an axis and button event for hats and dpads.
#[derive(Default)]
struct CtlrStateHat {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

/// Data associated with the state of the pad.  Used to produce the correct
/// platform-agnostic events.
struct CtlrState {
    // Trigger state
    trigger_l: f64,
    trigger_l_held: bool,
    // Trigger state
    trigger_r: f64,
    trigger_r_held: bool,
    // If last three-way state was negative (1 for each three-way).
    neg: Vec<Option<bool>>,
    // If last three-way axis state was negative (1 for each three-way).
    neg_axis: Vec<Option<bool>>,
    // If axis is zero (1 for each axis).
    dead: Vec<bool>,
    dead_trig: Vec<bool>,
    //
    dpad: CtlrStateHat,
    mic: CtlrStateHat,
    pov: CtlrStateHat,
    // Zero point
    zero: f64,
    // Normalization (-1.0, 1.0)
    norm: f64,
    // Flat value
    flat: f64,
    queued: Option<Event>,
}

type CtlrDescriptorAxes =
    (&'static dyn Fn(f64) -> Event, c_ushort, Option<f64>);
type CtlrDescriptorButtons = (&'static dyn Fn(bool) -> Event, c_ushort);
type CtlrDescriptorTrigButtons = (&'static dyn Fn(f64) -> Event, c_ushort);
type CtlrDescriptorTriggers = (
    &'static dyn Fn(f64) -> Event,
    c_ushort,
    Option<c_int>,
    Option<f64>,
);
type CtlrDescriptorThreeWays = (&'static dyn Fn(bool, bool) -> Event, c_ushort);
type CtlrDescriptorThreeAxes = (&'static dyn Fn(bool, f64) -> Event, c_ushort);
type CtlrDescriptorWheels = (&'static dyn Fn(f64) -> Event, c_ushort);

/// Describes some hardware joystick mapping
struct CtlrDescriptor {
    // Controller name
    name: &'static str,
    // Deadzone override
    deadzone: Option<f64>,

    // (Axis) value = Full range min to max axis
    axes: &'static [CtlrDescriptorAxes],
    // (Button) value = Boolean 1 or 0
    buttons: &'static [CtlrDescriptorButtons],
    // (Button) value = 0.0f64 or 1.0f64
    trigbtns: &'static [CtlrDescriptorTrigButtons],
    // (Axis) value = 0 thru 255
    triggers: &'static [CtlrDescriptorTriggers],
    // (Axis) value = -1, 0, or 1
    three_ways: &'static [CtlrDescriptorThreeWays],
    // (Axis) value = -1.0f64, 0, or 1.0f64
    three_axes: &'static [CtlrDescriptorThreeAxes],
    // (RelativeAxis) value = Full range min to max axis
    wheels: &'static [CtlrDescriptorWheels],
}

impl CtlrDescriptor {
    // Convert evdev event into Stick event.
    fn event_from(&self, ev: EvdevEv, state: &mut CtlrState) -> Option<Event> {
        fn check_held(value: c_int) -> Option<bool> {
            match value {
                0 => Some(false),
                1 => Some(true),
                2 => None, // Skip repeat "held" events
                v => {
                    eprintln!(
                        "Unknown Button State {}, report at \
                        https://github.com/libcala/stick/issues",
                        v
                    );
                    None
                }
            }
        }
        fn joyaxis_float(x: c_int, max: f64, state: &mut CtlrState) -> f64 {
            let v = (x as f64 - state.zero) * state.norm / max;
            if v.abs() <= state.flat {
                0.0
            } else {
                v.min(1.0).max(-1.0)
            }
        }
        fn trigger_float(x: c_int, flat: f64, max: c_int) -> f64 {
            let v = x as f64 / max as f64;
            if v.abs() <= flat {
                0.0
            } else {
                v.min(1.0).max(0.0)
            }
        }

        let event = match ev.ev_type {
            0x00 => {
                // Ignore SYN events.
                None
            }
            0x01 => {
                // button press / release (key)
                let mut event = None;
                let mut unknown = true;
                let ev_code = ev
                    .ev_code
                    .checked_sub(LINUX_SPECIFIC_BTN_OFFSET)
                    .unwrap_or_else(|| {
                        panic!(
                            "Out of range ev_code: {}, report at \
                        https://github.com/libcala/stick/issues",
                            ev.ev_code
                        )
                    });
                for (new, evcode) in self.buttons {
                    if ev_code == *evcode {
                        unknown = false;
                        let held = if let Some(held) = check_held(ev.ev_value) {
                            held
                        } else {
                            continue;
                        };
                        event = Some(new(held));
                    }
                }
                for (new, evcode) in self.trigbtns {
                    if ev_code == *evcode {
                        unknown = false;
                        let held = if let Some(held) = check_held(ev.ev_value) {
                            held
                        } else {
                            continue;
                        };
                        event = Some(match new(if held { 1.0 } else { 0.0 }) {
                            Event::TriggerL(v) => {
                                state.trigger_l_held = held;
                                Event::TriggerL(
                                    if v.classify() == FpCategory::Zero {
                                        state.trigger_l
                                    } else {
                                        v
                                    },
                                )
                            }
                            Event::TriggerR(v) => {
                                state.trigger_r_held = held;
                                Event::TriggerR(
                                    if v.classify() == FpCategory::Zero {
                                        state.trigger_r
                                    } else {
                                        v
                                    },
                                )
                            }
                            event => event,
                        });
                    }
                }
                if unknown && ev.ev_value != 2 {
                    eprintln!(
                        "*Evdev* Unknown Button Code: {}, Value: {}, report at \
                        https://github.com/libcala/stick/issues",
                        ev_code, ev.ev_value
                    );
                    None
                } else {
                    event
                }
            }
            0x02 => {
                // Relative axis movement
                let mut event = None;
                let mut unknown = true;
                for (new, evcode) in self.wheels {
                    if ev.ev_code == *evcode {
                        unknown = false;
                        event =
                            Some(new(joyaxis_float(ev.ev_value, 1.0, state)));
                    }
                }
                if unknown {
                    eprintln!(
                        "*Evdev* Unknown Relative Axis Code: {}, Value: {}, \
                        report at https://github.com/libcala/stick/issues",
                        ev.ev_code, ev.ev_value
                    );
                }
                event
            }
            0x03 => {
                let mut event = None;
                let mut unknown = true;
                // Absolute axis movement
                for (i, (new, evcode, max)) in self.axes.iter().enumerate() {
                    if ev.ev_code == *evcode {
                        unknown = false;
                        let v = joyaxis_float(
                            ev.ev_value,
                            max.unwrap_or(1.0),
                            state,
                        );
                        let is_zero = v.classify() == FpCategory::Zero;
                        if !(is_zero && state.dead[i]) {
                            state.dead[i] = is_zero;
                            event = Some(new(v));
                        }
                    }
                }
                for (i, (new, evcode, max, dead)) in
                    self.triggers.iter().enumerate()
                {
                    if ev.ev_code == *evcode {
                        unknown = false;
                        let v = trigger_float(
                            ev.ev_value,
                            dead.unwrap_or(0.0),
                            max.unwrap_or(255),
                        );
                        let is_zero = v.classify() == FpCategory::Zero;
                        if !(is_zero && state.dead_trig[i]) {
                            state.dead_trig[i] = is_zero;
                            match new(v) {
                                Event::TriggerL(v) => {
                                    state.trigger_l = v;
                                    if !state.trigger_l_held {
                                        event = Some(Event::TriggerL(v));
                                    }
                                }
                                Event::TriggerR(v) => {
                                    state.trigger_r = v;
                                    if !state.trigger_r_held {
                                        event = Some(Event::TriggerR(v));
                                    }
                                }
                                ev => event = Some(ev),
                            }
                        }
                    }
                }
                for (i, (new, evcode)) in self.three_ways.iter().enumerate() {
                    if ev.ev_code == *evcode {
                        unknown = false;
                        event = match ev.ev_value {
                            0 => state.neg[i].take().map(|old| new(old, false)),
                            v if v > 0 => {
                                let old = state.neg[i];
                                state.neg[i] = Some(false);
                                state.queued = Some(new(false, true));
                                if old == Some(true) {
                                    Some(new(true, false))
                                } else {
                                    state.queued.take()
                                }
                            }
                            _ => {
                                let old = state.neg[i];
                                state.neg[i] = Some(true);
                                state.queued = Some(new(true, true));
                                if old == Some(false) {
                                    Some(new(false, false))
                                } else {
                                    state.queued.take()
                                }
                            }
                        };
                    }
                }
                for (i, (new, evcode)) in self.three_axes.iter().enumerate() {
                    if ev.ev_code == *evcode {
                        unknown = false;
                        event = match ev.ev_value {
                            0 => state.neg_axis[i]
                                .take()
                                .map(|old| new(old, 0.0)),
                            v if v > 0 => {
                                let old = state.neg_axis[i];
                                state.neg_axis[i] = Some(false);
                                state.queued = Some(new(false, 1.0));
                                if old == Some(true) {
                                    Some(new(true, 0.0))
                                } else {
                                    state.queued.take()
                                }
                            }
                            _ => {
                                let old = state.neg_axis[i];
                                state.neg_axis[i] = Some(true);
                                state.queued = Some(new(true, 1.0));
                                if old == Some(false) {
                                    Some(new(false, 0.0))
                                } else {
                                    state.queued.take()
                                }
                            }
                        };
                    }
                }
                if unknown {
                    eprintln!(
                        "*Evdev* Unknown Absolute Axis Code: {}, Value: {}, \
                        report at https://github.com/libcala/stick/issues",
                        ev.ev_code, ev.ev_value
                    );
                }
                event
            }
            0x04 => {
                if ev.ev_code != /* scan */ 4 {
                    eprintln!(
                        "*Evdev* Unknown Misc Code: {}, Value: {}, report \
                        at https://github.com/libcala/stick/issues",
                        ev.ev_code, ev.ev_value
                    );
                }
                None
            }
            0x15 => {
                // Force Feedback echo, ignore
                None
            }
            u => {
                eprintln!(
                    "*Evdev* Unknown Event: {}, Code: {}, Value: {}, \
                    report at https://github.com/libcala/stick/issues.",
                    u, ev.ev_code, ev.ev_value
                );
                None
            }
        };

        // Remove duplicated events
        match event {
            Some(Event::DpadUp(p)) => {
                if p == state.dpad.up {
                    None
                } else {
                    state.dpad.up = p;
                    event
                }
            }
            Some(Event::DpadDown(p)) => {
                if p == state.dpad.down {
                    None
                } else {
                    state.dpad.down = p;
                    event
                }
            }
            Some(Event::DpadRight(p)) => {
                if p == state.dpad.right {
                    None
                } else {
                    state.dpad.right = p;
                    event
                }
            }
            Some(Event::DpadLeft(p)) => {
                if p == state.dpad.left {
                    None
                } else {
                    state.dpad.left = p;
                    event
                }
            }
            Some(Event::PovUp(p)) => {
                if p == state.pov.up {
                    None
                } else {
                    state.pov.up = p;
                    event
                }
            }
            Some(Event::PovDown(p)) => {
                if p == state.pov.down {
                    None
                } else {
                    state.pov.down = p;
                    event
                }
            }
            Some(Event::PovRight(p)) => {
                if p == state.pov.right {
                    None
                } else {
                    state.pov.right = p;
                    event
                }
            }
            Some(Event::PovLeft(p)) => {
                if p == state.pov.left {
                    None
                } else {
                    state.pov.left = p;
                    event
                }
            }
            Some(Event::MicUp(p)) => {
                if p == state.mic.up {
                    None
                } else {
                    state.mic.up = p;
                    event
                }
            }
            Some(Event::MicDown(p)) => {
                if p == state.mic.down {
                    None
                } else {
                    state.mic.down = p;
                    event
                }
            }
            Some(Event::MicRight(p)) => {
                if p == state.mic.right {
                    None
                } else {
                    state.mic.right = p;
                    event
                }
            }
            Some(Event::MicLeft(p)) => {
                if p == state.mic.left {
                    None
                } else {
                    state.mic.left = p;
                    event
                }
            }
            Some(Event::Nil(_)) => None,
            event => event,
        }
    }
}

mod gen {
    #![allow(clippy::if_same_then_else)]

    use super::*;

    include!(concat!(env!("OUT_DIR"), "/database.rs"));
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
}

impl Hub {
    pub(super) fn new() -> Self {
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
        }
    }

    // FIXME: split to disable/enable methods
    pub(super) fn enable(_flag: bool) {
        // do nothing
    }

    fn controller(mut filename: String) -> Poll<crate::Controller> {
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
                return Poll::Ready(crate::Controller(Ctlr::new(fd)));
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
        let mut this = self.as_mut();

        // Read the directory for ctrls if initialization hasn't completed yet.
        if let Some(ref mut read_dir) = this.read_dir {
            for dir_entry in read_dir.flatten() {
                let file = dir_entry.path();
                let path = file.as_path().to_string_lossy().to_string();
                if let Poll::Ready(controller) = Self::controller(path) {
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
            if let Poll::Ready(controller) = Self::controller(path) {
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
    hardware_id: [u16; 4],
    // Userspace driver data
    state: CtlrState,
    // Remappings
    desc: &'static CtlrDescriptor,
    // Rumble effect id.
    rumble: i16,
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
        let bustype = a.bustype.to_be();
        let vendor = a.vendor.to_be();
        let product = a.product.to_be();
        let version = a.version.to_be();
        let hardware_id = [bustype, vendor, product, version];

        // Get the controller's descriptor
        let desc = gen::ctlr_desc(bustype, vendor, product, version);

        // Get the min and max absolute values for axis.
        let mut a = MaybeUninit::<AbsInfo>::uninit();
        assert_ne!(
            unsafe { ioctl(fd, 0x_8018_4540, a.as_mut_ptr().cast()) },
            -1
        );
        let a = unsafe { a.assume_init() };
        let norm = (a.maximum as f64 - a.minimum as f64) * 0.5;
        let zero = a.minimum as f64 + norm;
        let flat = if let Some(flat) = desc.deadzone {
            flat
        } else {
            a.flat as f64 / norm
        };
        // Invert so multiplication can be used instead of division
        let norm = norm.recip();

        // Initialize driver state
        let state = CtlrState {
            trigger_l: 0.0,
            trigger_r: 0.0,
            trigger_l_held: false,
            trigger_r_held: false,
            neg: {
                let mut neg = vec![];
                for _ in desc.three_ways {
                    neg.push(None);
                }
                neg
            },
            neg_axis: {
                let mut neg = vec![];
                for _ in desc.three_axes {
                    neg.push(None);
                }
                neg
            },
            dead: vec![true; desc.axes.len()],
            dead_trig: vec![true; desc.triggers.len()],
            norm,
            zero,
            flat,
            queued: None,
            dpad: CtlrStateHat::default(),
            mic: CtlrStateHat::default(),
            pov: CtlrStateHat::default(),
        };

        // Query the controller for haptic support.
        let rumble = joystick_haptic(fd, -1, 0.0, 0.0);
        // Construct device from fd, looking for input events.
        let device = Device::new(fd, Watcher::new().input());
        // Return
        Self {
            hardware_id,
            device,
            state,
            desc,
            rumble,
        }
    }

    pub(super) fn id(&self) -> [u16; 4] {
        self.hardware_id
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
        if let Some(event) = self.state.queued.take() {
            return Poll::Ready(event);
        }

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
        if let Some(event) = self.desc.event_from(ev, &mut self.state) {
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
        let name = name.to_string_lossy().to_string();
        format!("{} ({})", self.desc.name, name)
    }

    pub(super) fn rumble(&mut self, left: f32, right: f32) {
        if self.rumble >= 0 {
            joystick_ff(
                self.device.raw(),
                self.rumble,
                left,
                right,
            );
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
            if errno != 19 { // 19 = device unplugged, ignore
                panic!(
                    "Write exited with {}",
                    *__errno_location()
                );
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
