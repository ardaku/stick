// Copyright © 2017-2022 The Stick Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.
//
//! Evdev -> Stick event conversion

use std::io::{Result, Read};
use std::os::raw::{c_ushort, c_int, c_uint, c_long, c_ulong, c_void, c_char};
use std::cmp::Ordering;
use std::os::unix::io::RawFd;
use std::mem::MaybeUninit;

use crate::Event;
use super::controller::Controller;

pub(crate) const EVENT_SIZE: usize = std::mem::size_of::<EvdevEv>();

// https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
pub(crate) const ABS_MAX: usize = 0x3F;

extern "C" {
    fn ioctl(fd: RawFd, request: c_ulong, v: *mut c_void) -> c_int;
}

/// Query
pub(crate) unsafe fn haptic_query(fd: RawFd, b: *mut c_void) -> Option<i16> {
    if ioctl(fd, 0x40304580, b) == -1 {
        Some(-1)
    } else {
        None
    }
}

/// Get the hardware id of this controller.
pub(crate) fn hardware_id(fd: RawFd) -> u64 {
    let mut id = MaybeUninit::<u64>::uninit();
    assert_ne!(
        unsafe { ioctl(fd, 0x_8008_4502, id.as_mut_ptr().cast()) },
        -1
    );
    unsafe { id.assume_init() }.to_be()
}

pub(crate) fn hardware_name(fd: RawFd) -> String {
    let mut a = MaybeUninit::<[c_char; 256]>::uninit();
    assert_ne!(
        unsafe { ioctl(fd, 0x80FF_4506, a.as_mut_ptr().cast()) },
        -1
    );
    let a = unsafe { a.assume_init() };
    let name = unsafe { std::ffi::CStr::from_ptr(a.as_ptr()) };

    name.to_string_lossy().to_string()
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

#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct AbsRange {
    // Minimum
    min: c_uint,
    // Flat
    flat: c_uint,
    // Normalization (2.0 / (maximum - minimum))
    norm: f64,
}

impl AbsRange {
    /// Normalize evdev event as f64.
    fn normalize(&self, value: c_uint) -> f64 {
        if (value as c_int).abs() < (self.flat as c_int) {
            return 0.0;
        }

        let unsigned = (value.wrapping_sub(self.min)) as f64;
        unsigned * self.norm - 1.0
    }

    /// Query absolute axes.
    pub(crate) fn query(fd: RawFd) -> [Self; ABS_MAX] {
        let mut output = [Self::default(); ABS_MAX];

        fn test_bit(index: usize, axis_list: &[u8; ABS_MAX / 8 + 1]) -> bool {
            let byte = index / 8;
            let bit = index % 8;
            axis_list[byte] & (1 << bit) != 0
        }

        let mut axis_list = MaybeUninit::<[u8; ABS_MAX / 8 + 1]>::uninit();
        assert_ne!(unsafe { ioctl(fd, 0x_8008_4523, axis_list.as_mut_ptr().cast()) }, -1);
        let axis_list = unsafe { axis_list.assume_init() };

        for i in 0..ABS_MAX {
            if test_bit(i, &axis_list) {
                let mut info = MaybeUninit::<AbsInfo>::uninit();
                if unsafe {
                    ioctl(fd, 0x_8018_4540 + i as c_ulong, info.as_mut_ptr().cast())
                } == -1 {
                    continue;
                }
                let info = unsafe { info.assume_init() };
                let _value = info.value; // FIXME: Send event.
                let min = info.minimum;
                let norm = 2.0 / info.maximum.wrapping_sub(info.minimum) as f64;
                let flat = info.flat as c_uint;

                output[i] = AbsRange {
                    min,
                    norm,
                    flat,
                };
            }
        }

        output
    }
}

#[repr(C)]
pub(crate) struct TimeVal {
    // struct timeval, from C.
    pub(crate) tv_sec: c_long,
    pub(crate) tv_usec: c_long,
}

#[repr(C)]
pub(crate) struct EvdevEv {
    // struct input_event, from C.
    pub(crate) ev_time: TimeVal,
    pub(crate) ev_type: c_ushort,
    pub(crate) ev_code: c_ushort,
    pub(crate) ev_value: c_uint,
}

// Event codes taken from
// https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h

// Convert Linux BTN press to stick Event.
fn to_stick_btn(
    btn: c_ushort,
    pushed: bool,
) -> Result<Option<Event>> {
    Ok(Some(match btn {
        0x08B /* KEY_MENU */ => Event::Context(pushed),

        0x09E /* KEY_BACK */ => Event::PaddleLeft(pushed),
        0x09F /* KEY_FORWARD */ => Event::PaddleRight(pushed),

        0x120 /* BTN_TRIGGER */ => Event::Trigger(pushed),
        0x121 /* BTN_THUMB */ => Event::ActionM(pushed),
        0x122 /* BTN_THUMB2 */ => Event::Bumper(pushed),
        0x123 /* BTN_TOP */ => Event::ActionR(pushed),
        0x124 /* BTN_TOP2 */ => Event::ActionL(pushed),
        0x125 /* BTN_PINKIE */ => Event::Pinky(pushed),
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
        0x138 /* BTN_TL2 */ => Event::TriggerL(f64::from(u8::from(pushed)) * 255.0),
        0x139 /* BTN_TR2 */ => Event::TriggerR(f64::from(u8::from(pushed)) * 255.0),
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
        0x2C1 /* BTN_TRIGGER_HAPPY2 */ => Event::Number(12, pushed),
        0x2C2 /* BTN_TRIGGER_HAPPY3 */ => Event::Number(13, pushed),
        0x2C3 /* BTN_TRIGGER_HAPPY4 */ => Event::Number(14, pushed),
        0x2C4 /* BTN_TRIGGER_HAPPY5 */ => Event::Number(15, pushed),
        0x2C5 /* BTN_TRIGGER_HAPPY6 */ => Event::Number(16, pushed),
        0x2C6 /* BTN_TRIGGER_HAPPY7 */ => Event::Number(17, pushed),
        0x2C7 /* BTN_TRIGGER_HAPPY8 */ => Event::Number(18, pushed),
        0x2C8 /* BTN_TRIGGER_HAPPY9 */ => Event::Number(19, pushed),
        0x2C9 /* BTN_TRIGGER_HAPPY10 */ => Event::Number(20, pushed),
        0x2CA /* BTN_TRIGGER_HAPPY11 */ => Event::Number(21, pushed),
        0x2CB /* BTN_TRIGGER_HAPPY12 */ => Event::Number(22, pushed),
        0x2CC /* BTN_TRIGGER_HAPPY13 */ => Event::Number(23, pushed),
        0x2CD /* BTN_TRIGGER_HAPPY14 */ => Event::Number(24, pushed),
        0x2CE /* BTN_TRIGGER_HAPPY15 */ => Event::Number(25, pushed),
        0x2CF /* BTN_TRIGGER_HAPPY16 */ => Event::Number(26, pushed),
        0x2D0 /* BTN_TRIGGER_HAPPY17 */ => Event::Number(27, pushed),
        0x2D1 /* BTN_TRIGGER_HAPPY18 */ => Event::Number(28, pushed),
        0x2D2 /* BTN_TRIGGER_HAPPY19 */ => Event::Number(29, pushed),
        0x2D3 /* BTN_TRIGGER_HAPPY20 */ => Event::Number(30, pushed),
        0x2D4 /* BTN_TRIGGER_HAPPY21 */ => Event::Number(31, pushed),
        0x2D5 /* BTN_TRIGGER_HAPPY22 */ => Event::Number(32, pushed),
        0x2D6 /* BTN_TRIGGER_HAPPY23 */ => Event::Number(33, pushed),
        0x2D7 /* BTN_TRIGGER_HAPPY24 */ => Event::Number(34, pushed),
        0x2D8 /* BTN_TRIGGER_HAPPY25 */ => Event::Number(35, pushed),
        0x2D9 /* BTN_TRIGGER_HAPPY26 */ => Event::Number(36, pushed),
        0x2DA /* BTN_TRIGGER_HAPPY27 */ => Event::Number(37, pushed),
        0x2DB /* BTN_TRIGGER_HAPPY28 */ => Event::Number(38, pushed),
        0x2DC /* BTN_TRIGGER_HAPPY29 */ => Event::Number(39, pushed),
        0x2DD /* BTN_TRIGGER_HAPPY30 */ => Event::Number(40, pushed),
        0x2DE /* BTN_TRIGGER_HAPPY31 */ => Event::Number(41, pushed),
        0x2DF /* BTN_TRIGGER_HAPPY32 */ => Event::Number(42, pushed),
        0x2E0 /* BTN_TRIGGER_HAPPY33 */ => Event::Number(43, pushed),
        0x2E1 /* BTN_TRIGGER_HAPPY34 */ => Event::Number(44, pushed),
        0x2E2 /* BTN_TRIGGER_HAPPY35 */ => Event::Number(45, pushed),
        0x2E3 /* BTN_TRIGGER_HAPPY36 */ => Event::Number(46, pushed),
        0x2E4 /* BTN_TRIGGER_HAPPY37 */ => Event::Number(47, pushed),
        0x2E5 /* BTN_TRIGGER_HAPPY38 */ => Event::Number(48, pushed),
        0x2E6 /* BTN_TRIGGER_HAPPY39 */ => Event::Number(49, pushed),
        0x2E7 /* BTN_TRIGGER_HAPPY40 */ => Event::Number(50, pushed),

        _unknown => {
            eprintln!("Unknown Linux Button {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None);
        }
    }))
}

// Convert Linux REL axis to stick Event.
fn to_stick_rel(
    axis: c_ushort,
    value: c_uint,
) -> Result<Option<Event>> {
    Ok(Some(match axis {
		0x00 /* REL_X */ => Event::MouseX(value as f64),
		0x01 /* REL_Y */ => Event::MouseY(value as f64),
		0x02 /* REL_Z */ => {
            eprintln!("FIXME: REL_Z");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x03 /* REL_RX */ => {
            eprintln!("FIXME: REL_RX");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x04 /* REL_RY */ => {
            eprintln!("FIXME: REL_RY");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x05 /* REL_RZ */ => {
            eprintln!("FIXME: REL_RZ");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x06 /* REL_HWHEEL */ => {
            eprintln!("FIXME: REL_HWHEEL");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x07 /* REL_DIAL */ => {
            eprintln!("FIXME: REL_DIAL");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x08 /* REL_WHEEL */ => {
            eprintln!("FIXME: REL_WHEEL");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x09 /* REL_MISC */ => {
            eprintln!("FIXME: REL_MISC");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
        _unknown => {
            eprintln!("Unknown Linux Axis {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
    }))
}

// Convert Linux ABS axis to stick Event.
fn to_stick_abs(
    controller: &mut Controller,
    axis: c_ushort,
    value: c_uint,
) -> Result<Option<Event>> {
    let value = controller.abs_ranges[usize::from(axis)].normalize(value);
    Ok(Some(match axis {
		0x00 /* ABS_X */ => Event::JoyX(value),
		0x01 /* ABS_Y */ => Event::JoyY(value),
		0x02 /* ABS_Z */ => Event::JoyZ(value),
		0x03 /* ABS_RX */ => Event::CamX(value),
		0x04 /* ABS_RY */ => Event::CamY(value),
		0x05 /* ABS_RZ */ => Event::CamZ(value),
		0x06 /* ABS_THROTTLE */ => Event::Throttle(value),
		0x07 /* ABS_RUDDER */ => Event::Rudder(value),
		0x08 /* ABS_WHEEL */ => Event::Wheel(value),
		0x09 /* ABS_GAS */ => Event::Gas(value),
		0x0A /* ABS_BRAKE */ => Event::Brake(value),
		0x0B /* ABS_UNKNOWN0 */ => Event::Slew(value),
		0x0C /* ABS_UNKNOWN1 */ => Event::ThrottleL(value),
		0x0D /* ABS_UNKNOWN2 */ => Event::ThrottleR(value),
		0x0E /* ABS_UNKNOWN3 */ => Event::ScrollX(value),
		0x0F /* ABS_UNKNOWN4 */ => Event::ScrollY(value),
		0x10 /* ABS_HAT0X */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::PovRight(true),
            Ordering::Less => Event::PovLeft(true),
            Ordering::Equal => {
                controller.queued = Some(Event::PovLeft(false));
                Event::PovRight(false)
            }
        },
		0x11 /* ABS_HAT0Y */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::PovDown(true),
            Ordering::Less => Event::PovUp(true),
            Ordering::Equal => {
                controller.queued = Some(Event::PovUp(false));
                Event::PovDown(false)
            }
        },
		0x12 /* ABS_HAT1X */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::HatRight(true),
            Ordering::Less => Event::HatLeft(true),
            Ordering::Equal => {
                controller.queued = Some(Event::HatLeft(false));
                Event::HatRight(false)
            }
        },
		0x13 /* ABS_HAT1Y */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::HatDown(true),
            Ordering::Less => Event::HatUp(true),
            Ordering::Equal => {
                controller.queued = Some(Event::HatUp(false));
                Event::HatDown(false)
            }
        },
		0x14 /* ABS_HAT2X */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::TrimRight(true),
            Ordering::Less => Event::TrimLeft(true),
            Ordering::Equal => {
                controller.queued = Some(Event::TrimLeft(false));
                Event::TrimRight(false)
            }
        },
		0x15 /* ABS_HAT2Y */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::TrimDown(true),
            Ordering::Less => Event::TrimUp(true),
            Ordering::Equal => {
                controller.queued = Some(Event::TrimUp(false));
                Event::TrimDown(false)
            }
        },
		0x16 /* ABS_HAT3X */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::MicRight(true),
            Ordering::Less => Event::MicLeft(true),
            Ordering::Equal => {
                controller.queued = Some(Event::MicLeft(false));
                Event::MicRight(false)
            }
        },
		0x17 /* ABS_HAT3Y */ => match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => Event::MicDown(true),
            Ordering::Less => Event::MicUp(true),
            Ordering::Equal => {
                controller.queued = Some(Event::MicUp(false));
                Event::MicDown(false)
            }
        },
		0x18 /* ABS_PRESSURE */ => {
            eprintln!("Unknown Event: ABS_PRESSURE");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x19 /* ABS_DISTANCE */ => {
            eprintln!("Unknown Event: ABS_DISTANCE");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x1a /* ABS_TILT_X */ => {
            eprintln!("Unknown Event: ABS_TILT_X");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x1b /* ABS_TILT_Y */ => {
            eprintln!("Unknown Event: ABS_TILT_Y");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x1c /* ABS_TOOL_WIDTH */ => {
            eprintln!("Unknown Event: ABS_TOOL_WIDTH");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x20 /* ABS_VOLUME */ => {
            eprintln!("Unknown Event: ABS_VOLUME");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
		0x28 /* ABS_MISC */ => {
            eprintln!("Unknown Event: ABS_MISC");
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
        _unknown => {
            eprintln!("Unknown Linux Axis {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            return Ok(None)
        }
    }))
}

pub(crate) fn to_stick_events(controller: &mut Controller) -> Result<Option<Event>> {
    let mut event = [0u8; EVENT_SIZE];
    let v = controller.stream.read(&mut event[..])?;
    assert_eq!(v, EVENT_SIZE);

    // If input is disabled, don't send events.
    if !super::ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
        return Ok(None);
    }

    let e: EvdevEv = unsafe { std::mem::transmute(event) };
    match e.ev_type {
        0x00 /* SYN */ => Ok(None), // Ignore Syn Input Events
        0x01 /* BTN */ => to_stick_btn(e.ev_code, e.ev_value != 0),
        0x02 /* REL */ => to_stick_rel(e.ev_code, e.ev_value),
        0x03 /* ABS */ => to_stick_abs(controller, e.ev_code, e.ev_value),
        0x04 /* MSC */ => {
            if e.ev_code != 4 { // Ignore Misc./Scan Events
                let (code, val) = (e.ev_code, e.ev_value);
                eprintln!("Unknown Linux Misc Code: {}, Value: {}", code, val);
                eprintln!("Report at https://github.com/libcala/stick/issues");
            }
            Ok(None)
        }
        0x15 /* FF */ => Ok(None), // Ignore Force Feedback Input Events
        _unknown => {
            eprintln!("Unknown Linux Event Type: {}", _unknown);
            eprintln!("Report at https://github.com/libcala/stick/issues");
            Ok(None)
        }
    }
}
