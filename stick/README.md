# Stick

#### Platform-agnostic asynchronous gamepad/joystick library for Rust

[![tests](https://github.com/libcala/stick/workflows/tests/badge.svg)](https://github.com/libcala/stick/actions?query=workflow%3Atests)
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
 - Dual licensed with the Boost license (permission to use without attribution
   in the binary's UI) - making it great for game development.
 - Not game-specific, doesn't depend on a "standard gamepad" model (which
   doesn't work due to the variety of controllers in existence) - therefore can
   also be used in robotics, control centers, advanced flight simulations, etc.
 - Support more types of gamepads/joysticks than gilrs, and (WIP) unified
   support across platforms.

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
[changelog](https://github.com/libcala/stick/blob/main/CHANGELOG.md)
to facilitate upgrading this crate as a dependency.

## License
Licensed under either of
 - Apache License, Version 2.0
   ([LICENSE_APACHE_2_0.txt](https://github.com/libcala/stick/blob/main/LICENSE_APACHE_2_0.txt) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
 - Boost License, Version 1.0
   ([LICENSE_BOOST_1_0.txt](https://github.com/libcala/stick/blob/main/LICENSE_BOOST_1_0.txt) or
   [https://www.boost.org/LICENSE_1_0.txt](https://www.boost.org/LICENSE_1_0.txt))

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Anyone is more than welcome to contribute!  Don't be shy about getting involved,
whether with a question, idea, bug report, bug fix, feature request, feature
implementation, or other enhancement.  Other projects have strict contributing
guidelines, but this project accepts any and all formats for pull requests and
issues.  For ongoing code contributions, if you wish to ensure your code is
used, open a draft PR so that I know not to write the same code.  If a feature
needs to be bumped in importance, I may merge an unfinished draft PR into it's
own branch and finish it (after a week's deadline for the person who openned
it).  Contributors will always be notified in this situation, and given a choice
to merge early.

All pull request contributors will have their username added in the contributors
section of the release notes of the next version after the merge, with a message
thanking them.  I always make time to fix bugs, so usually a patched version of
the library will be out a few days after a report.  Features requests will not
complete as fast.  If you have any questions, design critques, or want me to
find you something to work on based on your skill level, you can email me at
[jeronlau@plopgrizzly.com](mailto:jeronlau@plopgrizzly.com).  Otherwise,
[here's a link to the issues on GitHub](https://github.com/libcala/stick/issues),
and, as always, make sure to read and follow the
[Code of Conduct](https://github.com/libcala/stick/blob/main/CODE_OF_CONDUCT.md).
