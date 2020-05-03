# Changelog
All notable changes to `stick` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://jeronlau.tk/semver/).

## [0.8.0] - Unreleased
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
