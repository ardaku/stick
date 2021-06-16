// Copyright Daniel Parnell 2021.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

// This is based on code found here https://github.com/Lokathor/rusty-xinput by Lokathor

use std::{future::Future, task::{Context, Poll}, pin::Pin};

use crate::Event;
use std::task::Waker;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::{BOOL, BYTE, DWORD, HMODULE, UINT};
use winapi::shared::ntdef::LPWSTR;
use winapi::shared::winerror::{ERROR_DEVICE_NOT_CONNECTED, ERROR_EMPTY, ERROR_SUCCESS};
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryW};
use winapi::um::xinput::*;

use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

type XInputEnableFunc = unsafe extern "system" fn(BOOL);
type XInputGetStateFunc = unsafe extern "system" fn(DWORD, *mut XINPUT_STATE) -> DWORD;
type XInputSetStateFunc = unsafe extern "system" fn(DWORD, *mut XINPUT_VIBRATION) -> DWORD;
type XInputGetCapabilitiesFunc =
unsafe extern "system" fn(DWORD, DWORD, *mut XINPUT_CAPABILITIES) -> DWORD;

// Removed in xinput1_4.dll.
type XInputGetDSoundAudioDeviceGuidsFunc =
unsafe extern "system" fn(DWORD, *mut GUID, *mut GUID) -> DWORD;

// Added in xinput1_3.dll.
type XInputGetKeystrokeFunc = unsafe extern "system" fn(DWORD, DWORD, PXINPUT_KEYSTROKE) -> DWORD;
type XInputGetBatteryInformationFunc =
unsafe extern "system" fn(DWORD, BYTE, *mut XINPUT_BATTERY_INFORMATION) -> DWORD;

// Added in xinput1_4.dll.
type XInputGetAudioDeviceIdsFunc =
unsafe extern "system" fn(DWORD, LPWSTR, *mut UINT, LPWSTR, *mut UINT) -> DWORD;

struct ScopedHMODULE(HMODULE);
impl Drop for ScopedHMODULE {
    fn drop(&mut self) {
        unsafe { FreeLibrary(self.0) };
    }
}

/// A handle to a loaded XInput DLL.
#[derive(Clone)]
struct XInputHandle {
    handle: Arc<ScopedHMODULE>,
    xinput_enable: XInputEnableFunc,
    xinput_get_state: XInputGetStateFunc,
    xinput_set_state: XInputSetStateFunc,
    xinput_get_capabilities: XInputGetCapabilitiesFunc,
    opt_xinput_get_keystroke: Option<XInputGetKeystrokeFunc>,
    opt_xinput_get_battery_information: Option<XInputGetBatteryInformationFunc>,
    opt_xinput_get_audio_device_ids: Option<XInputGetAudioDeviceIdsFunc>,
    opt_xinput_get_dsound_audio_device_guids: Option<XInputGetDSoundAudioDeviceGuidsFunc>,
}

impl Debug for XInputHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "XInputHandle(handle = {:?})", self.handle.0)
    }
}

/// Quick and dirty wrapper to let us format log messages easier.
struct WideNullU16<'a>(&'a [u16; ::winapi::shared::minwindef::MAX_PATH]);
impl ::std::fmt::Debug for WideNullU16<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        for &u in self.0.iter() {
            if u == 0 {
                break;
            } else {
                write!(f, "{}", u as u8 as char)?
            }
        }
        Ok(())
    }
}

/// Converts a rusty string into a win32 string.
fn wide_null<S: AsRef<str>>(s: S) -> [u16; ::winapi::shared::minwindef::MAX_PATH] {
    let mut output: [u16; ::winapi::shared::minwindef::MAX_PATH] =
        [0; ::winapi::shared::minwindef::MAX_PATH];
    let mut i = 0;
    for u in s.as_ref().encode_utf16() {
        if i == output.len() - 1 {
            break;
        } else {
            output[i] = u;
        }
        i += 1;
    }
    output[i] = 0;
    output
}

unsafe impl Send for XInputHandle {}
unsafe impl Sync for XInputHandle {}

/// The ways that a dynamic load of XInput can fail.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum XInputLoadingFailure {
    /// No DLL for XInput could be found. This places the system back into an
    /// "uninitialized" status, and you could potentially try again later if the
    /// user fiddles with the program's DLL path or whatever.
    NoDLL,
    /// A DLL was found that matches one of the expected XInput DLL names, but it
    /// didn't contain both of the expected functions. This is probably a weird
    /// situation to find. Either way, the xinput status is set to "uninitialized"
    /// and as with the NoDLL error you could potentially try again.
    NoPointers,
}

impl XInputHandle {
    /// Attempts to dynamically load an XInput DLL and get the function pointers.
    ///
    /// # Failure
    ///
    /// This can fail in a few ways, as explained in the `XInputLoadingFailure`
    /// type. The most likely failure case is that the user's system won't have the
    /// required DLL, in which case you should probably allow them to play with just
    /// a keyboard/mouse instead.
    ///
    /// # Current DLL Names
    ///
    /// Currently the following DLL names are searched for in this order:
    ///
    /// * `xinput1_4.dll`
    /// * `xinput1_3.dll`
    /// * `xinput1_2.dll`
    /// * `xinput1_1.dll`
    /// * `xinput9_1_0.dll`
    pub(crate) fn load_default() -> Result<XInputHandle, XInputLoadingFailure> {
        let xinput14 = "xinput1_4.dll";
        let xinput13 = "xinput1_3.dll";
        let xinput12 = "xinput1_2.dll";
        let xinput11 = "xinput1_1.dll";
        let xinput91 = "xinput9_1_0.dll";

        for lib_name in [xinput14, xinput13, xinput12, xinput11, xinput91].iter() {
            if let Ok(handle) = XInputHandle::load(lib_name) {
                return Ok(handle);
            }
        }

        debug!("Failure: XInput could not be loaded.");
        Err(XInputLoadingFailure::NoDLL)
    }

    /// Attempt to load a specific XInput DLL and get the function pointers.
    pub(crate) fn load<S: AsRef<str>>(s: S) -> Result<XInputHandle, XInputLoadingFailure> {
        let lib_name = wide_null(s);
        trace!(
            "Attempting to load XInput DLL: {:?}",
            WideNullU16(&lib_name)
        );
        // It's always safe to call `LoadLibraryW`, the worst that can happen is
        // that we get a null pointer back.
        let xinput_handle = unsafe { LoadLibraryW(lib_name.as_ptr()) };
        if !xinput_handle.is_null() {
            debug!("Success: XInput Loaded: {:?}", WideNullU16(&lib_name));
        }

        let xinput_handle = ScopedHMODULE(xinput_handle);

        let enable_name = b"XInputEnable\0";
        let get_state_name = b"XInputGetState\0";
        let set_state_name = b"XInputSetState\0";
        let get_capabilities_name = b"XInputGetCapabilities\0";
        let get_keystroke_name = b"XInputGetKeystroke\0";
        let get_battery_information_name = b"XInputGetBatteryInformation\0";
        let get_audio_device_ids_name = b"XInputGetAudioDeviceIds\0";
        let get_dsound_audio_device_guids_name = b"XInputGetDSoundAudioDeviceGuids\0";

        let mut opt_xinput_enable = None;
        let mut opt_xinput_get_state = None;
        let mut opt_xinput_set_state = None;
        let mut opt_xinput_get_capabilities = None;
        let mut opt_xinput_get_keystroke = None;
        let mut opt_xinput_get_battery_information = None;
        let mut opt_xinput_get_audio_device_ids = None;
        let mut opt_xinput_get_dsound_audio_device_guids = None;

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let enable_ptr = GetProcAddress(xinput_handle.0, enable_name.as_ptr() as *mut i8);
            if !enable_ptr.is_null() {
                trace!("Found XInputEnable.");
                opt_xinput_enable = Some(::std::mem::transmute(enable_ptr));
            } else {
                trace!("Could not find XInputEnable.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_state_ptr = GetProcAddress(xinput_handle.0, get_state_name.as_ptr() as *mut i8);
            if !get_state_ptr.is_null() {
                trace!("Found XInputGetState.");
                opt_xinput_get_state = Some(::std::mem::transmute(get_state_ptr));
            } else {
                trace!("Could not find XInputGetState.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let set_state_ptr = GetProcAddress(xinput_handle.0, set_state_name.as_ptr() as *mut i8);
            if !set_state_ptr.is_null() {
                trace!("Found XInputSetState.");
                opt_xinput_set_state = Some(::std::mem::transmute(set_state_ptr));
            } else {
                trace!("Could not find XInputSetState.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_capabilities_ptr =
                GetProcAddress(xinput_handle.0, get_capabilities_name.as_ptr() as *mut i8);
            if !get_capabilities_ptr.is_null() {
                trace!("Found XInputGetCapabilities.");
                opt_xinput_get_capabilities = Some(::std::mem::transmute(get_capabilities_ptr));
            } else {
                trace!("Could not find XInputGetCapabilities.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_keystroke_ptr =
                GetProcAddress(xinput_handle.0, get_keystroke_name.as_ptr() as *mut i8);
            if !get_keystroke_ptr.is_null() {
                trace!("Found XInputGetKeystroke.");
                opt_xinput_get_keystroke = Some(::std::mem::transmute(get_keystroke_ptr));
            } else {
                trace!("Could not find XInputGetKeystroke.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_battery_information_ptr = GetProcAddress(
                xinput_handle.0,
                get_battery_information_name.as_ptr() as *mut i8,
            );
            if !get_battery_information_ptr.is_null() {
                trace!("Found XInputGetBatteryInformation.");
                opt_xinput_get_battery_information =
                    Some(::std::mem::transmute(get_battery_information_ptr));
            } else {
                trace!("Could not find XInputGetBatteryInformation.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_dsound_audio_device_guids_ptr = GetProcAddress(
                xinput_handle.0,
                get_dsound_audio_device_guids_name.as_ptr() as *mut i8,
            );
            if !get_dsound_audio_device_guids_ptr.is_null() {
                trace!("Found XInputGetDSoundAudioDeviceGuids.");
                opt_xinput_get_dsound_audio_device_guids =
                    Some(::std::mem::transmute(get_dsound_audio_device_guids_ptr));
            } else {
                trace!("Could not find XInputGetDSoundAudioDeviceGuids.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_audio_device_ids_ptr = GetProcAddress(
                xinput_handle.0,
                get_audio_device_ids_name.as_ptr() as *mut i8,
            );
            if !get_audio_device_ids_ptr.is_null() {
                trace!("Found XInputGetAudioDeviceIds.");
                opt_xinput_get_audio_device_ids = Some(::std::mem::transmute(get_audio_device_ids_ptr));
            } else {
                trace!("Could not find XInputGetAudioDeviceIds.");
            }
        }

        // this is safe because no other code can be loading xinput at the same time as us.
        if opt_xinput_enable.is_some()
            && opt_xinput_get_state.is_some()
            && opt_xinput_set_state.is_some()
            && opt_xinput_get_capabilities.is_some()
        {
            debug!("All function pointers loaded successfully.");
            Ok(XInputHandle {
                handle: Arc::new(xinput_handle),
                xinput_enable: opt_xinput_enable.unwrap(),
                xinput_get_state: opt_xinput_get_state.unwrap(),
                xinput_set_state: opt_xinput_set_state.unwrap(),
                xinput_get_capabilities: opt_xinput_get_capabilities.unwrap(),
                opt_xinput_get_keystroke,
                opt_xinput_get_battery_information,
                opt_xinput_get_dsound_audio_device_guids,
                opt_xinput_get_audio_device_ids,
            })
        } else {
            debug!("Could not load the function pointers.");
            Err(XInputLoadingFailure::NoPointers)
        }
    }
}

/// These are all the sorts of problems that can come up when you're using the
/// xinput system.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum XInputUsageError {
    /// The controller ID you gave was 4 or more.
    InvalidControllerID,
    /// Not really an error, this controller is just missing.
    DeviceNotConnected,
    /// There was some sort of unexpected error happened, this is the error code
    /// windows returned.
    UnknownError(u32),
}

/// Error that can be returned by functions that are not guaranteed to be present
/// in earlier XInput versions.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum XInputOptionalFnUsageError {
    /// The controller ID you gave was 4 or more.
    InvalidControllerID,
    /// Not really an error, this controller is just missing.
    DeviceNotConnected,
    /// Function is not present in loaded DLL
    FunctionNotLoaded,
    /// There was some sort of unexpected error happened, this is the error code
    /// windows returned.
    UnknownError(u32),
}

/// This wraps an `XINPUT_STATE` value and provides a more rusty (read-only)
/// interface to the data it contains.
///
/// All three major game companies use different names for most of the buttons,
/// so the docs for each button method list out what each of the major companies
/// call that button. To the driver it's all the same, it's just however you
/// want to think of them.
///
/// If sequential calls to `xinput_get_state` for a given controller slot have
/// the same packet number then the controller state has not changed since the
/// last call. The `PartialEq` and `Eq` implementations for this wrapper type
/// reflect that. The exact value of the packet number is unimportant.
///
/// If you want to do something that the rust wrapper doesn't support, just use
/// the raw field to get at the inner value.
struct XInputState {
    /// The raw value we're wrapping.
    pub(crate) raw: XINPUT_STATE,
}

impl ::std::cmp::PartialEq for XInputState {
    /// Equality for `XInputState` values is based _only_ on the
    /// `dwPacketNumber` of the wrapped `XINPUT_STATE` value. This is entirely
    /// correct for values obtained from the xinput system, but if you make your
    /// own `XInputState` values for some reason you can confuse it.
    fn eq(&self, other: &XInputState) -> bool {
        self.raw.dwPacketNumber == other.raw.dwPacketNumber
    }
}

impl ::std::cmp::Eq for XInputState {}

impl ::std::fmt::Debug for XInputState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "XInputState (_)")
    }
}

impl XInputState {
    /// This helper normalizes a raw stick value using the given deadzone.
    ///
    /// If the raw value's 2d length is less than the deadzone the result will be
    /// `(0.0,0.0)`, otherwise the result is normalized across the range from the
    /// deadzone point to the maximum value.
    ///
    /// The `deadzone` value is clamped to the range 0 to 32,766 (inclusive)
    /// before use. Negative inputs or maximum value inputs make the normalization
    /// just work improperly.
    #[inline]
    pub(crate) fn normalize_raw_stick_value(raw_stick: (i16, i16), deadzone: i16) -> (f64, f64) {
        let deadzone_float = deadzone.max(0).min(i16::MAX - 1) as f64;
        let raw_float = (raw_stick.0 as f64, raw_stick.1 as f64);
        let length = (raw_float.0 * raw_float.0 + raw_float.1 * raw_float.1).sqrt();
        let normalized = (raw_float.0 / length, raw_float.1 / length);
        if length > deadzone_float {
            // clip our value to the expected maximum length.
            let length = length.min(32_767.0);
            let scale = (length - deadzone_float) / (32_767.0 - deadzone_float);
            (normalized.0 * scale, normalized.1 * scale)
        } else {
            (0.0, 0.0)
        }
    }
}

impl XInputHandle {
    /// Enables or disables XInput.
    ///
    /// See the [MSDN documentation for XInputEnable](https://docs.microsoft.com/en-us/windows/desktop/api/xinput/nf-xinput-xinputenable).
    pub(crate) fn enable(&self, enable: bool) -> () {
        unsafe { (self.xinput_enable)(enable as BOOL) };
    }

    /// Polls the controller port given for the current controller state.
    ///
    /// # Notes
    ///
    /// It is a persistent problem (since ~2007?) with xinput that polling for the
    /// data of a controller that isn't connected will cause a long delay. In the
    /// area of 500_000 cpu cycles. That's like 2_000 cache misses in a row.
    ///
    /// Once a controller is detected as not being plugged in you are strongly
    /// advised to not poll for its data again next frame. Instead, you should
    /// probably only poll for one known-missing controller per frame at most.
    ///
    /// Alternately, you can register for your app to get plug and play events and
    /// then wait for one of them to come in before you ever poll for a missing
    /// controller a second time. That's up to you.
    ///
    /// # Errors
    ///
    /// A few things can cause an `Err` value to come back, as explained by the
    /// `XInputUsageError` type.
    ///
    /// Most commonly, a controller will simply not be connected. Most people don't
    /// have all four slots plugged in all the time.
    pub(crate) fn get_state(&self, user_index: u32) -> Result<XInputState, XInputUsageError> {
        if user_index >= 4 {
            Err(XInputUsageError::InvalidControllerID)
        } else {
            let mut output: XINPUT_STATE = unsafe { ::std::mem::zeroed() };
            let return_status = unsafe { (self.xinput_get_state)(user_index, &mut output) };
            match return_status {
                ERROR_SUCCESS => return Ok(XInputState { raw: output }),
                ERROR_DEVICE_NOT_CONNECTED => Err(XInputUsageError::DeviceNotConnected),
                s => {
                    trace!("Unexpected error code: {}", s);
                    Err(XInputUsageError::UnknownError(s))
                }
            }
        }
    }

    /// Allows you to set the rumble speeds of the left and right motors.
    ///
    /// Valid motor speeds are across the whole `u16` range, and the number is the
    /// scale of the motor intensity. In other words, 0 is 0%, and 65,535 is 100%.
    ///
    /// On a 360 controller the left motor is low-frequency and the right motor is
    /// high-frequency. On other controllers running through xinput this might be
    /// the case, or the controller might not even have rumble ability at all. If
    /// rumble is missing from the device you'll still get `Ok` return values, so
    /// treat rumble as an extra, not an essential.
    ///
    /// # Errors
    ///
    /// A few things can cause an `Err` value to come back, as explained by the
    /// `XInputUsageError` type.
    ///
    /// Most commonly, a controller will simply not be connected. Most people don't
    /// have all four slots plugged in all the time.
    pub(crate) fn set_state(
        &self, user_index: u32, left_motor_speed: u16, right_motor_speed: u16,
    ) -> Result<(), XInputUsageError> {
        if user_index >= 4 {
            Err(XInputUsageError::InvalidControllerID)
        } else {
            let mut input = XINPUT_VIBRATION {
                wLeftMotorSpeed: left_motor_speed,
                wRightMotorSpeed: right_motor_speed,
            };
            let return_status = unsafe { (self.xinput_set_state)(user_index, &mut input) };
            match return_status {
                ERROR_SUCCESS => Ok(()),
                ERROR_DEVICE_NOT_CONNECTED => Err(XInputUsageError::DeviceNotConnected),
                s => {
                    trace!("Unexpected error code: {}", s);
                    Err(XInputUsageError::UnknownError(s))
                }
            }
        }
    }

    /// Retrieve a gamepad input event.
    ///
    /// See the [MSDN documentation for XInputGetKeystroke](https://docs.microsoft.com/en-us/windows/desktop/api/xinput/nf-xinput-xinputgetkeystroke).
    pub(crate) fn get_keystroke(
        &self, user_index: u32,
    ) -> Result<Option<XINPUT_KEYSTROKE>, XInputOptionalFnUsageError> {
        if user_index >= 4 {
            Err(XInputOptionalFnUsageError::InvalidControllerID)
        } else if let Some(func) = self.opt_xinput_get_keystroke {
            unsafe {
                let mut keystroke = std::mem::MaybeUninit::<XINPUT_KEYSTROKE>::uninit();
                let return_status = (func)(user_index, 0, keystroke.as_mut_ptr());
                match return_status {
                    ERROR_SUCCESS => Ok(Some(keystroke.assume_init())),
                    ERROR_EMPTY => Ok(None),
                    ERROR_DEVICE_NOT_CONNECTED => Err(XInputOptionalFnUsageError::DeviceNotConnected),
                    s => {
                        trace!("Unexpected error code: {}", s);
                        Err(XInputOptionalFnUsageError::UnknownError(s))
                    }
                }
            }
        } else {
            Err(XInputOptionalFnUsageError::FunctionNotLoaded)
        }
    }
}


type LPTIMECALLBACK = extern "C" fn(timer_id: u32, msg: u32, dw_user: usize, dw1: usize, dw2: usize);

const TIME_ONESHOT: u32 = 0;

// Link to the windows multimedia library.
#[link(name = "winmm")]
extern "system" {
    // set a callback to be triggered after a given ammount of time has passed
    fn timeSetEvent(delay: u32, resolution: u32, lpTimeProc: LPTIMECALLBACK, dw_user: usize, fu_event: u32) -> u32;
}


lazy_static! {
  static ref GLOBAL_XINPUT_HANDLE: Result<XInputHandle, XInputLoadingFailure> =
    XInputHandle::load_default();
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
    connected: u64,
    to_check: u8,
}

impl Hub {
    pub(super) fn new() -> Self {

        Hub {
            connected: 0,
            to_check: 0,
        }
    }

    pub(super) fn enable(flag: bool) {
        if let Ok(ref handle) = *GLOBAL_XINPUT_HANDLE {
            handle.enable(flag);
        }
    }
}

impl Future for Hub {
    type Output = (usize, Event);

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        if let Ok(ref handle) = *GLOBAL_XINPUT_HANDLE {
            let id = self.to_check;
            let mask = 1 << id;
            self.to_check = self.to_check + 1;
            // direct input only allows for 4 controllers
            if self.to_check > 3 {
                self.to_check = 0;
            }
            let was_connected = (self.connected & mask) != 0;

            if let Ok(_) = handle.get_state(id as u32) {
                if !was_connected {
                    // we have a new device!
                    self.connected = self.connected | mask;

                    return Poll::Ready((
                        usize::MAX,
                        Event::Connect(Box::new(crate::Controller(
                            Ctlr::new(id),
                        ))),
                    ));
                }
            } else {
                if was_connected {
                    // a device has been unplugged
                    self.connected = self.connected & !mask;
                }
            }
        }
        register_wake_timeout(100, cx.waker());

        Poll::Pending
    }
}

pub(crate) struct Ctlr {
    device_id: u8,
    pending_events: Vec<Event>,
    last_packet: DWORD,
    joy_x: f64,
    joy_y: f64,
    cam_x: f64,
    cam_y: f64,
    trigger_left: u8,
    trigger_right: u8,
}

impl Ctlr {
    #[allow(unused)]
    fn new(device_id: u8) -> Self {
        Self {
            device_id,
            pending_events: Vec::new(),
            last_packet: 0,
            joy_x: 0.0,
            joy_y: 0.0,
            cam_x: 0.0,
            cam_y: 0.0,
            trigger_left: 0,
            trigger_right: 0
        }
    }

    pub(super) fn id(&self) -> [u16; 4] {
        [0, 0, 0, 0]
    }

    pub(super) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
        if let Some(e) = self.pending_events.pop() {
            return Poll::Ready(e);
        }

        if let Ok(ref handle) = *GLOBAL_XINPUT_HANDLE {
            if let Ok(state) = handle.get_state(self.device_id as u32) {
                if state.raw.dwPacketNumber != self.last_packet {
                    // we have a new packet from the controller
                    self.last_packet = state.raw.dwPacketNumber;

                    let (nx, ny) = XInputState::normalize_raw_stick_value((state.raw.Gamepad.sThumbRX, state.raw.Gamepad.sThumbRY), XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE);
                    if nx != self.joy_x {
                        self.joy_x = nx;
                        self.pending_events.push(Event::JoyX(nx));
                    }
                    if ny != self.joy_y {
                        self.joy_y = ny;
                        self.pending_events.push(Event::JoyY(ny));
                    }

                    let (nx, ny) = XInputState::normalize_raw_stick_value((state.raw.Gamepad.sThumbLX, state.raw.Gamepad.sThumbLY), XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE);
                    if nx != self.cam_x {
                        self.cam_x = nx;
                        self.pending_events.push(Event::CamX(nx));
                    }
                    if ny != self.cam_y {
                        self.cam_y = ny;
                        self.pending_events.push(Event::CamY(ny));
                    }

                    let t = if state.raw.Gamepad.bLeftTrigger > XINPUT_GAMEPAD_TRIGGER_THRESHOLD {
                        state.raw.Gamepad.bLeftTrigger
                    } else {
                        0
                    };
                    if t != self.trigger_left {
                        self.trigger_left = t;
                        self.pending_events.push(Event::TriggerL(t as f64 / 255.0));
                    }

                    let t = if state.raw.Gamepad.bRightTrigger > XINPUT_GAMEPAD_TRIGGER_THRESHOLD {
                        state.raw.Gamepad.bRightTrigger
                    } else {
                        0
                    };
                    if t != self.trigger_right {
                        self.trigger_right = t;
                        self.pending_events.push(Event::TriggerR(t as f64 / 255.0));
                    }
                }

                while let Ok(Some(keystroke)) = handle.get_keystroke(self.device_id as u32) {
                    // ignore key repeat events
                    if keystroke.Flags & XINPUT_KEYSTROKE_REPEAT == 0 {
                        match keystroke.VirtualKey {
                            VK_PAD_START => self.pending_events.push(Event::Next(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_BACK => self.pending_events.push(Event::Prev(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_A => self.pending_events.push(Event::ActionA(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_B => self.pending_events.push(Event::ActionB(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_X => self.pending_events.push(Event::ActionC(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_Y => self.pending_events.push(Event::ActionH(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_LSHOULDER => self.pending_events.push(Event::BumperL(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_RSHOULDER => self.pending_events.push(Event::BumperR(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_LTHUMB_PRESS => self.pending_events.push(Event::JoyPush(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_RTHUMB_PRESS => self.pending_events.push(Event::CamPush(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_DPAD_UP => self.pending_events.push(Event::DpadUp(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_DPAD_DOWN => self.pending_events.push(Event::DpadDown(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_DPAD_LEFT => self.pending_events.push(Event::DpadLeft(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            VK_PAD_DPAD_RIGHT => self.pending_events.push(Event::DpadRight(keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0)),
                            _ => ()
                        }
                    }
                }

                if let Some(event) = self.pending_events.pop() {
                    return Poll::Ready(event);
                }
            } else {
                // the device has gone
                return Poll::Ready(Event::Disconnect);
            }
        } else {
            // XInput has gone away?!?
            return Poll::Ready(Event::Disconnect);
        }

        register_wake_timeout(10, cx.waker());
        Poll::Pending
    }

    pub(super) fn name(&self) -> String {
        String::from("Xinput device")
    }

    pub(super) fn rumble(&mut self, v: f32) {
        self.rumbles(v, v);
    }

    pub(super) fn rumbles(&mut self, l: f32, r: f32) {
        if let Ok(ref handle) = *GLOBAL_XINPUT_HANDLE {
            handle.set_state(self.device_id as u32, (u16::MAX as f32 * l) as u16, (u16::MAX as f32 * r) as u16).unwrap();
        }
    }
}
