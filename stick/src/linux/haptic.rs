// Copyright © 2017-2021 The Stick Crate Developers.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use std::mem::size_of;
use std::convert::TryInto;
use std::os::unix::prelude::*;
use std::os::raw::{c_int, c_void};

extern "C" {
    fn write(fd: RawFd, buf: *const c_void, count: usize) -> isize;
    fn __errno_location() -> *mut c_int;
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

pub(crate) fn joystick_ff(fd: RawFd, code: i16, strong: f32, weak: f32) {
    // Update haptic effect `code`.
    if strong != 0.0 || weak != 0.0 {
        joystick_haptic(fd, code, strong, weak);
    }
    //
    let ev_code = code.try_into().unwrap();

    let play = &super::evdev::EvdevEv {
        ev_time: super::evdev::TimeVal {
            tv_sec: 0,
            tv_usec: 0,
        },
        ev_type: 0x15, /*EV_FF*/
        ev_code,
        ev_value: (strong > 0.0 || weak > 0.0) as _,
    };
    let play: *const _ = play;
    unsafe {
        if write(fd, play.cast(), size_of::<super::evdev::EvdevEv>())
            != size_of::<super::evdev::EvdevEv>() as isize
        {
            let errno = *__errno_location();
            if errno != 19 && errno != 9 {
                // 19 = device unplugged, ignore
                // 9 = device openned read-only, ignore
                panic!("Write exited with {}", *__errno_location());
            }
        }
    }
}

// Get ID's for rumble and vibrate, if they're supported (otherwise, -1).
pub(crate) fn joystick_haptic(fd: RawFd, id: i16, strong: f32, weak: f32) -> i16 {
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
    #[allow(trivial_casts)]
    let v = unsafe { super::evdev::haptic_query(fd, (a as *mut FfEffect).cast()).unwrap_or(a.id) };
    drop(a);
    v
}
