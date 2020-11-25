# Stick

#### Platform-agnostic asynchronous gamepad/joystick library for Rust

[![Build Status](https://api.travis-ci.org/libcala/stick.svg?branch=master)](https://travis-ci.org/libcala/stick)
[![Docs](https://docs.rs/stick/badge.svg)](https://docs.rs/stick)
[![crates.io](https://img.shields.io/crates/v/stick.svg)](https://crates.io/crates/stick)

Stick supports getting controller input from a large variety of gamepads,
joysticks, and other controllers, as well as rumble haptic effects.

### Why Does Stick Exist?
The main reason is that I hadn't heard of gilrs when I started stick back in
2017 when gilrs was only a year old and had less than 500 all-time downloads.
Now, I think there are many other reasons for stick to exist despite gilrs:

 - Executor-agnostic `async/.await` for gamepads, joysticks, etc (I recommend
   using the `pasts` crate for a simple single-threaded executor).
 - Low-level hotplugging support (you assign the gamepad ID's)
 - Meaningful Event Names (`ActionA` and `ActionB` instead of `South` and
   `East`)
 - Minimal dependencies
 - Dual licensed with the Zlib license (permission to use without attribution in
   the binary's UI) - making it great for game development.
 - Not game-specific, doesn't depend on a "standard gamepad" model (which
   doesn't work due to the variety of controllers in existence) - therefore can
   also be used in robotics, control centers, advanced flight simulations, etc.
 - Support more gamepads/joysticks than gilrs, and (WIP) unified support across
   platforms.

### Platform Support
- Linux

### Planned Platform Support
- Windows
- MacOS
- BSD
- Redox
- Fuchsia
- Android
- iOS
- Web Assembly
- Nintendo Switch (And other game consoles)
- Others

## Table of Contents
- [Getting Started](#getting-started)
   - [Example](#example)
   - [API](#api)
   - [Features](#features)
- [Upgrade](#upgrade)
- [License](#license)
   - [Contribution](#contribution)

### API
API documentation can be found on [docs.rs](https://docs.rs/stick).

### Features
There are no optional features.

## Upgrade
You can use the
[changelog](https://github.com/libcala/stick/blob/master/CHANGELOG.md)
to facilitate upgrading this crate as a dependency.

## License
Licensed under either of
 - Apache License, Version 2.0,
   ([LICENSE-APACHE](https://github.com/libcala/stick/blob/master/LICENSE-APACHE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
 - Zlib License,
   ([LICENSE-ZLIB](https://github.com/libcala/stick/blob/master/LICENSE-ZLIB) or
   [https://opensource.org/licenses/Zlib](https://opensource.org/licenses/Zlib))

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Contributors are always welcome (thank you for being interested!), whether it
be a bug report, bug fix, feature request, feature implementation or whatever.
Don't be shy about getting involved.  I always make time to fix bugs, so usually
a patched version of the library will be out a few days after a report.
Features requests will not complete as fast.  If you have any questions, design
critques, or want me to find you something to work on based on your skill level,
you can email me at [jeronlau@plopgrizzly.com](mailto:jeronlau@plopgrizzly.com).
Otherwise,
[here's a link to the issues on GitHub](https://github.com/libcala/stick/issues).
Before contributing, check out the
[contribution guidelines](https://github.com/libcala/stick/blob/master/CONTRIBUTING.md),
and, as always, make sure to follow the
[code of conduct](https://github.com/libcala/stick/blob/master/CODE_OF_CONDUCT.md).
