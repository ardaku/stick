# Changelog
All notable changes to `stick` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.12.0] - Unreleased
### Added
 - You can now import your own mappings at runtime **WIP**
 - Support For WASM **WIP**
 - Default evdev id guessing **WIP**

### Changed
 - Database now stored as data rather than control flow in resulting binary
   (lowering bloat) - **WIP**
 - Use faster-compiling dependency than serde for controller mappings **WIP**

### Removed
 - Controllers from sdl controller db (it assumes everything is a gamepad, which
   not all controllers are - stick will only include semantically correct
   mappings from now on).

### Fixed
 - Cleaned up database (fixing inconsistencies).

## [0.11.1] - 2020-12-13
### Fixed
 - Dummy implementation not compiling (started using GitHub Actions so hopefully
   this won't happen again).

## [0.11.0] - 2020-11-25
### Added
 - `Controller::listener()` to replace `Hub`.

### Changed
 - `Pad` renamed to `Controller`.

### Removed
 - `Hub` type, use `Controller::listener()` instead.

### Fixed
 - The controller database is now sorted (thanks to
   [iRaiko](https://github.com/iRaiko)!)

## [0.10.2] - 2020-09-20
### Fixed
 - Incorrect mapping for Xbox One wired controller

### Contributors
Thanks to everyone who contributed to make this version of stick possible!

 - [AldaronLau](https://github.com/AldaronLau)
 - [jamesmcm](https://github.com/jamesmcm)

## [0.10.1] - 2020-06-26
### Added
 - Support for XBox and Steam controller Grip Buttons / Paddles:
   - `Event::PaddleRight(bool)`
   - `Event::PaddleLeft(bool)`
   - `Event::PaddleRightPinky(bool)`
   - `Event::PaddleLeftPinky(bool)`

## [0.10.0] - 2020-06-09
### Added
 - Support for all of the gamepads and joysticks in SDL\_gamecontrollerdb by
   creating a database of gamepads (using a TOML format), as well as others that
   weren't in the database (pads stick supported before, plus the Thrustmaster
   Warthog).
 - `Event` variants:
   - `ActionC(bool)`
   - `JoyZ(f64)`
   - `CamZ(f64)`
   - `AutopilotToggle(bool)`
   - `LandingGearSilence(bool)`
   - `PovUp(bool)`
   - `PovDown(bool)`
   - `PovLeft(bool)`
   - `PovRight(bool)`
   - `MicUp(bool)`
   - `MicDown(bool)`
   - `MicLeft(bool)`
   - `MicRight(bool)`
   - `MicPush(bool)`
   - `Slew(f64)`
   - `Throttle(f64)`
   - `ThrottleL(f64)`
   - `ThrottleR(f64)`
   - `ThrottleButtonL(bool)`
   - `EngineFuelFlowL(bool)`
   - `EngineFuelFlowR(bool)`
   - `Eac(bool)`
   - `RadarAltimeter(bool)`
   - `Apu(bool)`
   - `AutopilotPath(bool)`
   - `AutopilotAlt(bool)`
   - `FlapsUp(bool)`
   - `FlapsDown(bool)`
   - `EngineLIgnition(bool)`
   - `EngineLMotor(bool)`
   - `EngineRIgnition(bool)`
   - `EngineRMotor(bool)`
   - `PinkyForward(bool)`
   - `PinkyBackward(bool)`
   - `SpeedbrakeForward(bool)`
   - `SpeedbrakeBackward(bool)`
   - `BoatForward(bool)`
   - `BoatBackward(bool)`
   - `ChinaForward(bool)`
   - `ChinaBackward(bool)`
   - `Dpi(bool)`
   - `MouseX(f64)`
   - `MouseY(f64)`
   - `MousePush(bool)`
   - `WheelX(f64)`
   - `WheelY(f64)`
   - `WheelPush(bool)`

### Changed
 - Renamed `Port` to `Hub`
 - Renamed `Gamepad` to `Pad`
 - `id()` now returns a `[u16; 4]` instead of a `u32` for gamepad ID.
 - Renamed `Event` variants:
   - `Accept(bool)` -> `ActionA(bool)`
   - `Cancel(bool)` -> `ActionB(bool)`
   - `Common(bool)` -> `ActionH(bool)`
   - `Action(bool)` -> `ActionV(bool)`
   - `Up(bool)` -> `DpadUp(bool)`
   - `Down(bool)` -> `DpadDown(bool)`
   - `Left(bool)` -> `DpadLeft(bool)`
   - `Right(bool)` -> `DpadRight(bool)`
   - `Back(bool)` -> `Prev(bool)`
   - `Forward(bool)` -> `Next(bool)`
   - `L(bool)` -> `BumperL(bool)`
   - `R(bool)` -> `BumperR(bool)`
   - `Lt(f32)` -> `TriggerL(f64)`
   - `Rt(f32)` -> `TriggerR(f64)`
   - `MotionH(f32)` -> `JoyX(f64)`
   - `MotionV(f32)` -> `JoyY(f64)`
   - `CameraH(f32)` -> `CamX(f64)`
   - `CameraV(f32)` -> `CamY(f64)`
   - `MotionButton(bool)` -> `JoyPush(bool)`
   - `CameraButton(bool)` -> `CamPush(bool)`
   - `ExtL(u8, bool)` -> `Action(u16, bool)`
   - `ExtR(u8, bool)` -> `Action(u16, bool)`
   - `Quit` -> `Home(bool)`
 - `Event` is now marked `#[non_exhaustive]`, so you will alway have to match
   for `_`.

### Fixed
 - Randomly crashing
 - All clippy lints
 - No longer does `dbg!()` prints constantly.

### Contributors
Thanks to everyone who contributed to make this version of stick possible!

- [AldaronLau](https://github.com/AldaronLau)
- [theunkn0wn1](https://github.com/theunkn0wn1)

## [0.9.0] - 2020-05-18
### Added
- Support for a lot of different kinds of joysticks.
- `ExtL` and `ExtR` variants of `Event` enum for buttons that aren't on the W3C
  standard gamepad.

### Changed
- Update documentation/examples to use pasts 0.4
- Remove unneeded `(&mut )` in example/documentation.
- `Lz` and `Rz` variants on `Event` are renamed to `Lt` and `Rt`

### Contributions
Thanks to everyone who contributed to make this version of stick possible!

 - [AldaronLau](https://github.com/AldaronLau)
 - [Chronophylos](https://github.com/Chronophylos)

 - Thanks to everyone who helped test joysticks at
   [https://github.com/libcala/stick/issues/5](https://github.com/libcala/stick/issues/5)!

## [0.8.0] - 2020-05-03
### Added
- Async/await support
- Haptic support with `.rumble()` on `Gamepad`.
- Linux implementation of `.name()` on `Gamepad`.
- `Event` enum.

### Changed
- Renamed `Device` to `Gamepad`.
- Back to an event-based API with `Event`.

### Removed
- `Btn` enum, use `Event` instead.

### Fixed
- Panic on drop (joystick disconnected)

### Contributors
Thanks to everyone who contributed to make this version of stick possible!

- [AldaronLau](https://github.com/AldaronLau)
- [jannic](https://github.com/jannic)

## [0.7.1] - 2019-07-18
### Fixed
- Not compiling for 32-bit architecture.

## [0.7.0] - 2019-05-22
### Added
- `poll()` method to block until an event is received from any controller.
- Asynchronous support.
- Multi-threaded support.
- `count()` to get the number of plugged in controllers.
- Implementation of default for `Port`.
- `CONTROLLER_MAX` constant, set to 64 controllers.

### Removed
- Pan separate from camera Y.
- Mods

### Changed
- `update()` is now renamed to `poll()` and is asynchronous.  It's now recommended to put your input on it's own thread and call `poll` which blocks.
- There's now a limit of 64 joysticks, because it makes multi-threaded joystick support easier and faster.
- Joystick Ids are now `u8` instead of `u16`.

### Fixed
- L & R triggers without buttons requiring mods to be treated as buttons.

## [0.6.0] - 2019-05-13
### Added
- `Device.lrt()` function to get left & right trigger values.

### Fixed
- Can only extract `Device.joy()` values once.

## [0.5.0] - 2019-05-12
### Added
- Full support for 4 joysticks
- New API with `Port`, `Device` and `Btn`
- API to detect whether or not joystick features are supported (not complete).

### Removed
- `ControllerManager` & `Input`

### Changed
- Input is now received through function calls like `joy()` instead of the `Input` Enum

## [0.4.1] - 2018-08-05
### Fixed
- Crash on specific hardware running Linux.

## [0.4.0] - 2018-05-23
### Added
- Fake Windows support.

### Removed
- `Button` - Now incorporated into `Input`.

## [0.3.0] - 2018-02-03
### Added
- Added `ControllerManager` to simplify API

### Removed
- Removed `Throttle` struct and `Joystick` struct

## [0.2.0] - 2018-01-27
### Added
- Remapping

### Changed
- Use evdev instead of js0

## [0.1.0] - 2018-01-01
### Added
- Linux Support
