// Copyright Daniel Parnell 2021.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.


use std::mem;

use std::{future::Future, task::{Context, Poll}, pin::Pin};

use crate::Event;
use std::mem::MaybeUninit;
use std::task::Waker;

type Tchar = i16;
const MAXPNAMELEN: usize = 32;
const MAX_JOYSTICKOEMVXDNAME: usize = 260;

#[repr(C)]
#[derive(Copy, Clone)]
struct JoyCaps {
    w_mid: u16,
    w_pid: u16,
    sz_pname: [Tchar; MAXPNAMELEN],
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
    z_min: u32,
    z_max: u32,
    num_buttons: u32,
    period_min: u32,
    period_max: u32,
    r_min: u32,
    r_max: u32,
    u_min: u32,
    u_max: u32,
    v_min: u32,
    v_max: u32,
    caps: u32,
    max_axes: u32,
    num_axes: u32,
    max_buttons: u32,
    sz_reg_key: [Tchar; MAXPNAMELEN],
    sz_oem_vxd: [Tchar; MAX_JOYSTICKOEMVXDNAME],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct JoyInfoEx {
    size: u32,
    flags: u32,
    x_pos: u32,
    y_pos: u32,
    z_pos: u32,
    r_pos: u32,
    u_pos: u32,
    v_pos: u32,
    buttons: u32,
    button_number: u32,
    pov: u32,
    reserved_1: u32,
    reserved_2: u32,
}

impl JoyInfoEx {
    pub fn new() -> Self {
        JoyInfoEx {
            size: std::mem::size_of::<JoyInfoEx>() as u32,
            flags: 0xffffffff - 0x100,
            x_pos: 0,
            y_pos: 0,
            z_pos: 0,
            r_pos: 0,
            u_pos: 0,
            v_pos: 0,
            buttons: 0,
            button_number: 0,
            pov: 0,
            reserved_1: 0,
            reserved_2: 0
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Timecaps {
    w_period_min: u32,
    w_period_max: u32
}

type LPTIMECALLBACK = extern "C" fn(timer_id: u32, msg: u32, dw_user: usize, dw1: usize, dw2: usize);

const TIME_ONESHOT: u32 = 0;

// Link to the windows multimedia library.
#[link(name = "winmm")]
extern "system" {
    // Get number of joysticks that the system supports.
    fn joyGetNumDevs() -> u32;
    //
    fn joyGetDevCapsW(joy_id: usize, caps: *mut JoyCaps, cbjc: u32) -> u32;
    //
    fn joyGetPosEx(joy_id: u32, pji: *mut JoyInfoEx) -> u32;

    // set a callback to be triggered after a given amout of time has passed
    fn timeSetEvent(delay: u32, resolution: u32, lpTimeProc: LPTIMECALLBACK, dw_user: usize, fu_event: u32) -> u32;
}

extern "C" fn waker_callback(_timer_id: u32, _msg: u32, dw_user: usize, _dw1: usize, _dw2: usize) {
    unsafe {
        let waker = std::mem::transmute::<usize, &Waker>(dw_user);

        waker.wake_by_ref();
    }
}

fn register_wake_timeout(delay: u32, waker: &Waker) {
    unsafe {
        let waker = std::mem::transmute::<&Waker, usize>(waker);

        timeSetEvent(delay, 0, waker_callback, waker, TIME_ONESHOT);
    }
}

pub(crate) struct Hub {
    supported: usize,
    connected: u64
}

impl Hub {
    pub(super) fn new() -> Self {
        let supported = unsafe { joyGetNumDevs() } as usize;

        Hub {
            supported,
            connected: 0
        }
    }
}

impl Future for Hub {
    type Output = (usize, Event);

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        for i in 0..self.supported {
            let mask = 1 << i;
            let connected = (self.connected & mask) != 0;

            let mut joy_caps = MaybeUninit::uninit();

            let (error_code, joy_caps) = unsafe {
                let result = joyGetDevCapsW(i, joy_caps.as_mut_ptr(), mem::size_of::<JoyCaps>() as u32);

                (result, joy_caps.assume_init())
            };

            if error_code == 0 {
                if connected {
                    // do nothing
                } else {
                    // a new device has been connected
                    self.connected = self.connected + mask;

                    return Poll::Ready((
                        usize::MAX,
                        Event::Connect(Box::new(crate::Controller(
                            Ctlr::new(i as usize, joy_caps),
                        ))),
                    ));
                }
            } else {
                // device is not present
                if connected {
                    // it was disconnected
                    println!("device {} disconnected", i);
                    println!("connected before = {:#x}", self.connected);
                    self.connected = self.connected - mask;
                    println!("connected after = {:#x}", self.connected);
                }
            }
        }

        register_wake_timeout(100, cx.waker());

        Poll::Pending
    }
}

pub(crate) struct Ctlr {
    device_id: usize,
    caps: JoyCaps,
    info: JoyInfoEx,
    pending_events: Vec<Event>
}

impl Ctlr {
    #[allow(unused)]
    fn new(device_id: usize, caps: JoyCaps) -> Self {
        Self {
            device_id,
            caps,
            info: JoyInfoEx::new(),
            pending_events: Vec::new()
        }
    }

    pub(super) fn id(&self) -> [u16; 4] {
        [0, 0, self.caps.w_mid, self.caps.w_pid]
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
        if let Some(e) = self.pending_events.pop() {
            return Poll::Ready(e);
        }
        let mut now = JoyInfoEx::new();
        let error_code = unsafe { joyGetPosEx(self.device_id as u32, &mut now) };

        if error_code == 0 {
            // println!("{} now = {:?}", self.device_id, now);
            if now.x_pos != self.info.x_pos {
                self.pending_events.push(Event::JoyX(now.x_pos as f64 / self.caps.x_max as f64))
            }
            if now.y_pos != self.info.y_pos {
                self.pending_events.push(Event::JoyY(now.y_pos as f64 / self.caps.y_max as f64))
            }
            if now.u_pos != self.info.u_pos {
                self.pending_events.push(Event::CamX(now.u_pos as f64 / self.caps.u_max as f64))
            }
            if now.v_pos != self.info.v_pos {
                self.pending_events.push(Event::CamY(now.v_pos as f64 / self.caps.v_max as f64))
            }
            if now.buttons != self.info.buttons {

            }

            // save off the state for next time
            self.info = now;
        } else {
            // the device has been removed
            return Poll::Ready(Event::Disconnect);
        }

        register_wake_timeout(10, cx.waker());

        Poll::Pending
    }

    pub(super) fn name(&self) -> String {
        let mut result = String::with_capacity(MAXPNAMELEN);
        for i in 0..MAXPNAMELEN {
            let ch = self.caps.sz_pname[i] as u8 as char;
            if ch == '\0' {
                break;
            }
            result.push(ch);
        }

        result
    }

    pub(super) fn rumble(&mut self, v: f32) {
        let _ = v;
    }
}