// Stick
// Copyright Daniel Parnell 2021.
// Copyright Jeron Aldaron Lau 2021.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

//! This file's code is based on https://github.com/Lokathor/rusty-xinput

use crate::{Event, Remap};
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;
use std::task::Waker;
use std::task::{Context, Poll};
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::{BOOL, BYTE, DWORD, HMODULE, UINT};
use winapi::shared::ntdef::LPWSTR;
use winapi::shared::winerror::{
    ERROR_DEVICE_NOT_CONNECTED, ERROR_EMPTY, ERROR_SUCCESS,
};
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryW};
use winapi::um::xinput;

type XInputEnableFunc = unsafe extern "system" fn(BOOL);
type XInputGetStateFunc =
    unsafe extern "system" fn(DWORD, *mut xinput::XINPUT_STATE) -> DWORD;
type XInputSetStateFunc =
    unsafe extern "system" fn(DWORD, *mut xinput::XINPUT_VIBRATION) -> DWORD;
type XInputGetCapabilitiesFunc = unsafe extern "system" fn(
    DWORD,
    DWORD,
    *mut xinput::XINPUT_CAPABILITIES,
) -> DWORD;

// Removed in xinput1_4.dll.
type XInputGetDSoundAudioDeviceGuidsFunc =
    unsafe extern "system" fn(DWORD, *mut GUID, *mut GUID) -> DWORD;

// Added in xinput1_3.dll.
type XInputGetKeystrokeFunc =
    unsafe extern "system" fn(DWORD, DWORD, xinput::PXINPUT_KEYSTROKE) -> DWORD;
type XInputGetBatteryInformationFunc = unsafe extern "system" fn(
    DWORD,
    BYTE,
    *mut xinput::XINPUT_BATTERY_INFORMATION,
) -> DWORD;

// Added in xinput1_4.dll.
type XInputGetAudioDeviceIdsFunc = unsafe extern "system" fn(
    DWORD,
    LPWSTR,
    *mut UINT,
    LPWSTR,
    *mut UINT,
) -> DWORD;

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
    opt_xinput_get_dsound_audio_device_guids:
        Option<XInputGetDSoundAudioDeviceGuidsFunc>,
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
fn wide_null<S: AsRef<str>>(
    s: S,
) -> [u16; ::winapi::shared::minwindef::MAX_PATH] {
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
    pub(crate) fn load_default(
    ) -> Result<Arc<XInputHandle>, XInputLoadingFailure> {
        let xinput14 = "xinput1_4.dll";
        let xinput13 = "xinput1_3.dll";
        let xinput12 = "xinput1_2.dll";
        let xinput11 = "xinput1_1.dll";
        let xinput91 = "xinput9_1_0.dll";

        for lib_name in
            [xinput14, xinput13, xinput12, xinput11, xinput91].iter()
        {
            if let Ok(handle) = XInputHandle::load(lib_name) {
                return Ok(Arc::new(handle));
            }
        }

        debug!("Failure: XInput could not be loaded.");
        Err(XInputLoadingFailure::NoDLL)
    }

    /// Attempt to load a specific XInput DLL and get the function pointers.
    pub(crate) fn load<S: AsRef<str>>(
        s: S,
    ) -> Result<XInputHandle, XInputLoadingFailure> {
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
        let get_dsound_audio_device_guids_name =
            b"XInputGetDSoundAudioDeviceGuids\0";

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
            let enable_ptr = GetProcAddress(
                xinput_handle.0,
                enable_name.as_ptr() as *mut i8,
            );
            if !enable_ptr.is_null() {
                trace!("Found XInputEnable.");
                opt_xinput_enable = Some(::std::mem::transmute(enable_ptr));
            } else {
                trace!("Could not find XInputEnable.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_state_ptr = GetProcAddress(
                xinput_handle.0,
                get_state_name.as_ptr() as *mut i8,
            );
            if !get_state_ptr.is_null() {
                trace!("Found XInputGetState.");
                opt_xinput_get_state =
                    Some(::std::mem::transmute(get_state_ptr));
            } else {
                trace!("Could not find XInputGetState.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let set_state_ptr = GetProcAddress(
                xinput_handle.0,
                set_state_name.as_ptr() as *mut i8,
            );
            if !set_state_ptr.is_null() {
                trace!("Found XInputSetState.");
                opt_xinput_set_state =
                    Some(::std::mem::transmute(set_state_ptr));
            } else {
                trace!("Could not find XInputSetState.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_capabilities_ptr = GetProcAddress(
                xinput_handle.0,
                get_capabilities_name.as_ptr() as *mut i8,
            );
            if !get_capabilities_ptr.is_null() {
                trace!("Found XInputGetCapabilities.");
                opt_xinput_get_capabilities =
                    Some(::std::mem::transmute(get_capabilities_ptr));
            } else {
                trace!("Could not find XInputGetCapabilities.");
            }
        }

        // using transmute is so dodgy we'll put that in its own unsafe block.
        unsafe {
            let get_keystroke_ptr = GetProcAddress(
                xinput_handle.0,
                get_keystroke_name.as_ptr() as *mut i8,
            );
            if !get_keystroke_ptr.is_null() {
                trace!("Found XInputGetKeystroke.");
                opt_xinput_get_keystroke =
                    Some(::std::mem::transmute(get_keystroke_ptr));
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
                opt_xinput_get_dsound_audio_device_guids = Some(
                    ::std::mem::transmute(get_dsound_audio_device_guids_ptr),
                );
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
                opt_xinput_get_audio_device_ids =
                    Some(::std::mem::transmute(get_audio_device_ids_ptr));
            } else {
                trace!("Could not find XInputGetAudioDeviceIds.");
            }
        }

        // this is safe because no other code can be loading xinput at the same time as us.
        if let (
            Some(xinput_enable),
            Some(xinput_get_state),
            Some(xinput_set_state),
            Some(xinput_get_capabilities),
        ) = (
            opt_xinput_enable,
            opt_xinput_get_state,
            opt_xinput_set_state,
            opt_xinput_get_capabilities,
        ) {
            debug!("All function pointers loaded successfully.");
            Ok(XInputHandle {
                handle: Arc::new(xinput_handle),
                xinput_enable,
                xinput_get_state,
                xinput_set_state,
                xinput_get_capabilities,
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
    pub(crate) raw: xinput::XINPUT_STATE,
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
    #[inline]
    fn normalize_raw_stick_value(
        (x, y): (i16, i16),
        deadzone: i16,
    ) -> (f64, f64) {
        // Clamp the deadzone value to make the normalization work properly
        let deadzone = deadzone.clamp(0, 32_766);
        // Convert x and y to range -1 to 1, invert Y axis
        let x = (x as f64 + 0.5) * (1.0 / 32_767.5);
        let y = (y as f64 + 0.5) * -(1.0 / 32_767.5);
        // Convert deadzone to range 0 to 1
        let deadzone = deadzone as f64 * (1.0 / 32_767.5);
        // Calculate distance from (0, 0)
        let distance = (x * x + y * y).sqrt();
        // Return 0 unless distance is far enough away from deadzone
        if distance > deadzone {
            (x, y)
        } else {
            (0.0, 0.0)
        }
    }
}

impl XInputHandle {
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
    pub(crate) fn get_state(
        &self,
        user_index: u32,
    ) -> Result<XInputState, XInputUsageError> {
        if user_index >= 4 {
            Err(XInputUsageError::InvalidControllerID)
        } else {
            let mut output: xinput::XINPUT_STATE =
                unsafe { ::std::mem::zeroed() };
            let return_status =
                unsafe { (self.xinput_get_state)(user_index, &mut output) };
            match return_status {
                ERROR_SUCCESS => Ok(XInputState { raw: output }),
                ERROR_DEVICE_NOT_CONNECTED => {
                    Err(XInputUsageError::DeviceNotConnected)
                }
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
        &self,
        user_index: u32,
        left_motor_speed: u16,
        right_motor_speed: u16,
    ) -> Result<(), XInputUsageError> {
        if user_index >= 4 {
            Err(XInputUsageError::InvalidControllerID)
        } else {
            let mut input = xinput::XINPUT_VIBRATION {
                wLeftMotorSpeed: left_motor_speed,
                wRightMotorSpeed: right_motor_speed,
            };
            let return_status =
                unsafe { (self.xinput_set_state)(user_index, &mut input) };
            match return_status {
                ERROR_SUCCESS => Ok(()),
                ERROR_DEVICE_NOT_CONNECTED => {
                    Err(XInputUsageError::DeviceNotConnected)
                }
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
        &self,
        user_index: u32,
    ) -> Result<Option<xinput::XINPUT_KEYSTROKE>, XInputOptionalFnUsageError>
    {
        if user_index >= 4 {
            Err(XInputOptionalFnUsageError::InvalidControllerID)
        } else if let Some(func) = self.opt_xinput_get_keystroke {
            unsafe {
                let mut keystroke =
                    std::mem::MaybeUninit::<xinput::XINPUT_KEYSTROKE>::uninit();
                let return_status =
                    (func)(user_index, 0, keystroke.as_mut_ptr());
                match return_status {
                    ERROR_SUCCESS => Ok(Some(keystroke.assume_init())),
                    ERROR_EMPTY => Ok(None),
                    ERROR_DEVICE_NOT_CONNECTED => {
                        Err(XInputOptionalFnUsageError::DeviceNotConnected)
                    }
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

type LpTimeCallback = extern "C" fn(
    timer_id: u32,
    msg: u32,
    dw_user: usize,
    dw1: usize,
    dw2: usize,
);

const TIME_ONESHOT: u32 = 0;

// Link to the windows multimedia library.
#[link(name = "winmm")]
extern "system" {
    // set a callback to be triggered after a given ammount of time has passed
    fn timeSetEvent(
        delay: u32,
        resolution: u32,
        lpTimeProc: LpTimeCallback,
        dw_user: usize,
        fu_event: u32,
    ) -> u32;
}

extern "C" fn waker_callback(
    _timer_id: u32,
    _msg: u32,
    dw_user: usize,
    _dw1: usize,
    _dw2: usize,
) {
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

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct Controller {
    xinput: Arc<XInputHandle>,
    device_id: u8,
    pending_events: Vec<Event>,
    last_packet: DWORD,
}

impl Controller {
    #[allow(unused)]
    fn new(device_id: u8, xinput: Arc<XInputHandle>) -> Self {
        Self {
            xinput,
            device_id,
            pending_events: Vec::new(),
            last_packet: 0,
        }
    }
}

impl super::Controller for Controller {
    fn id(&self) -> u64 {
        0 // FIXME
    }

    /// Poll for events.
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Event> {
        if let Some(e) = self.pending_events.pop() {
            return Poll::Ready(e);
        }

        if let Ok(state) = self.xinput.get_state(self.device_id as u32) {
            if state.raw.dwPacketNumber != self.last_packet {
                // we have a new packet from the controller
                self.last_packet = state.raw.dwPacketNumber;

                let (nx, ny) = XInputState::normalize_raw_stick_value(
                    (state.raw.Gamepad.sThumbRX, state.raw.Gamepad.sThumbRY),
                    xinput::XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE,
                );

                self.pending_events.push(Event::CamX(nx));
                self.pending_events.push(Event::CamY(ny));

                let (nx, ny) = XInputState::normalize_raw_stick_value(
                    (state.raw.Gamepad.sThumbLX, state.raw.Gamepad.sThumbLY),
                    xinput::XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE,
                );

                self.pending_events.push(Event::JoyX(nx));
                self.pending_events.push(Event::JoyY(ny));

                let t = if state.raw.Gamepad.bLeftTrigger
                    > xinput::XINPUT_GAMEPAD_TRIGGER_THRESHOLD
                {
                    state.raw.Gamepad.bLeftTrigger
                } else {
                    0
                };

                self.pending_events.push(Event::TriggerL(t as f64 / 255.0));

                let t = if state.raw.Gamepad.bRightTrigger
                    > xinput::XINPUT_GAMEPAD_TRIGGER_THRESHOLD
                {
                    state.raw.Gamepad.bRightTrigger
                } else {
                    0
                };

                self.pending_events.push(Event::TriggerR(t as f64 / 255.0));

                while let Ok(Some(keystroke)) =
                    self.xinput.get_keystroke(self.device_id as u32)
                {
                    // Ignore key repeat events
                    if keystroke.Flags & xinput::XINPUT_KEYSTROKE_REPEAT != 0 {
                        continue;
                    }

                    let held =
                        keystroke.Flags & xinput::XINPUT_KEYSTROKE_KEYDOWN != 0;

                    match keystroke.VirtualKey {
                        xinput::VK_PAD_START => {
                            self.pending_events.push(Event::MenuR(held))
                        }
                        xinput::VK_PAD_BACK => {
                            self.pending_events.push(Event::MenuL(held))
                        }
                        xinput::VK_PAD_A => {
                            self.pending_events.push(Event::ActionA(held))
                        }
                        xinput::VK_PAD_B => {
                            self.pending_events.push(Event::ActionB(held))
                        }
                        xinput::VK_PAD_X => {
                            self.pending_events.push(Event::ActionH(held))
                        }
                        xinput::VK_PAD_Y => {
                            self.pending_events.push(Event::ActionV(held))
                        }
                        xinput::VK_PAD_LSHOULDER => {
                            self.pending_events.push(Event::BumperL(held))
                        }
                        xinput::VK_PAD_RSHOULDER => {
                            self.pending_events.push(Event::BumperR(held))
                        }
                        xinput::VK_PAD_LTHUMB_PRESS => {
                            self.pending_events.push(Event::Joy(held))
                        }
                        xinput::VK_PAD_RTHUMB_PRESS => {
                            self.pending_events.push(Event::Cam(held))
                        }
                        xinput::VK_PAD_DPAD_UP => {
                            self.pending_events.push(Event::Up(held))
                        }
                        xinput::VK_PAD_DPAD_DOWN => {
                            self.pending_events.push(Event::Down(held))
                        }
                        xinput::VK_PAD_DPAD_LEFT => {
                            self.pending_events.push(Event::Left(held))
                        }
                        xinput::VK_PAD_DPAD_RIGHT => {
                            self.pending_events.push(Event::Right(held))
                        }
                        _ => (),
                    }
                }

                if let Some(event) = self.pending_events.pop() {
                    return Poll::Ready(event);
                }
            }
        } else {
            // the device has gone
            return Poll::Ready(Event::Disconnect);
        }

        register_wake_timeout(10, cx.waker());
        Poll::Pending
    }

    /// Stereo rumble effect (left is low frequency, right is high frequency).
    fn rumble(&mut self, left: f32, right: f32) {
        self.xinput
            .set_state(
                self.device_id as u32,
                (u16::MAX as f32 * left) as u16,
                (u16::MAX as f32 * right) as u16,
            )
            .unwrap()
    }

    /// Get the name of this controller.
    fn name(&self) -> &str {
        "XInput Controller"
    }
}

pub(crate) struct Listener {
    xinput: Arc<XInputHandle>,
    connected: u64,
    to_check: u8,
    remap: Remap,
}

impl Listener {
    fn new(remap: Remap, xinput: Arc<XInputHandle>) -> Self {
        Self {
            xinput,
            connected: 0,
            to_check: 0,
            remap,
        }
    }
}

impl super::Listener for Listener {
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<crate::Controller> {
        let id = self.to_check;
        let mask = 1 << id;
        self.to_check += 1;
        // direct input only allows for 4 controllers
        if self.to_check > 3 {
            self.to_check = 0;
        }
        let was_connected = (self.connected & mask) != 0;

        if self.xinput.get_state(id as u32).is_ok() {
            if !was_connected {
                // we have a new device!
                self.connected |= mask;

                return Poll::Ready(crate::Controller::new(
                    Box::new(Controller::new(id, self.xinput.clone())),
                    &self.remap,
                ));
            }
        } else if was_connected {
            // a device has been unplugged
            self.connected &= !mask;
        }

        register_wake_timeout(100, cx.waker());

        Poll::Pending
    }
}

struct Global {
    xinput: Arc<XInputHandle>,
}

impl super::Global for Global {
    /// Enable all events (when window comes in focus).
    fn enable(&self) {
        unsafe { (self.xinput.xinput_enable)(true as _) };
    }
    /// Disable all events (when window leaves focus).
    fn disable(&self) {
        unsafe { (self.xinput.xinput_enable)(false as _) };
    }
    /// Create a new listener.
    fn listener(&self, remap: Remap) -> Box<dyn super::Listener> {
        Box::new(Listener::new(remap, self.xinput.clone()))
    }
}

pub(super) fn global() -> Box<dyn super::Global> {
    // Windows implementation may fail.
    if let Ok(xinput) = XInputHandle::load_default() {
        Box::new(Global { xinput })
    } else {
        Box::new(super::FakeGlobal)
    }
}
