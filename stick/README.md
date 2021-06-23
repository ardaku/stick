# Stick

#### Platform-agnostic asynchronous gamepad, joystick and flightstick library

[![tests](https://github.com/libcala/stick/workflows/tests/badge.svg)](https://github.com/libcala/stick/actions?query=workflow%3Atests)
[![Docs](https://docs.rs/stick/badge.svg)](https://docs.rs/stick)
[![crates.io](https://img.shields.io/crates/v/stick.svg)](https://crates.io/crates/stick)

Stick supports getting controller input from a large variety of gamepads,
joysticks, flightsticks, and other controllers.  Stick also supports left/right
rumble haptic effects.

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
 - Windows

### Planned Platform Support
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
You may enable the following features
 - **sdb**: Enabled by default, the Stick database controller remappings
 - **gcdb**: The SDL game controller database remappings

## Upgrade
You can use the [changelog][3] to facilitate upgrading this crate as a dependency.

## License
Licensed under any of
 - Apache License, Version 2.0, ([LICENSE_APACHE_2_0.txt][7]
   or [https://www.apache.org/licenses/LICENSE-2.0][8])
 - MIT License, ([LICENSE_MIT.txt][9] or [https://mit-license.org/][10])
 - Boost Software License, Version 1.0, ([LICENSE_BOOST_1_0.txt][11]
   or [https://www.boost.org/LICENSE_1_0.txt][12])

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## Help
If you want help using or contributing to this library, feel free to send me an
email at [aldaronlau@gmail.com][13].

[0]: https://docs.rs/stick
[1]: https://crates.io/crates/stick
[2]: https://github.com/libcala/stick/actions?query=workflow%3Atests
[3]: https://github.com/libcala/stick/blob/main/CHANGELOG.md
[4]: https://github.com/libcala/stick/
[5]: https://docs.rs/stick#getting-started
[6]: https://aldaronlau.com/
[7]: https://github.com/libcala/stick/blob/main/LICENSE_APACHE_2_0.txt
[8]: https://www.apache.org/licenses/LICENSE-2.0
[9]: https://github.com/libcala/stick/blob/main/LICENSE_MIT.txt
[10]: https://mit-license.org/
[11]: https://github.com/libcala/whoami/stick/main/LICENSE_BOOST_1_0.txt
[12]: https://www.boost.org/LICENSE_1_0.txt
[13]: mailto:aldaronlau@gmail.com
