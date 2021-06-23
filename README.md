# Stick
Folders in this repository:
 - `stick/`: The Stick crate.
 - `sdb/`: The Stick Public Domain database of controllers (not limited
   to game controllers).
 - `gcdb/`: Git Submodule to grab optional SDL mappings from.
 - `xtask/`: Scripts run with `cargo-xtask`.

## Xtask
Stick uses the [xtask](https://github.com/matklad/cargo-xtask) repository model
to have various scripts written in Rust process controller data, also allowing
serde to not be a direct dependency.

### Available Commands
 - `cargo xtask`, `cargo xtask --help` - Print help
 - `cargo xtask sdb` - Generate the embeddable bytecode databases.

## TOML Format
File names are 64-bit hexadecimal values with leading zeros followed by `.toml`
within a folder referring to the platform.

```toml
name = "My Controller Name"
type = "xbox"

[remap]
TriggerR = {} # Ignore events
TriggerL = {}
HatUp = Up # Map hat to dpad when you're not using a flightstick
HatDown = Down
HatLeft = Left
HatRight = Right
CamX = { event = "CamX", deadzone = 0.075 } # Adjust deadzones
CamY = { event = "CamY", deadzone = 0.075 }
JoyZ = { event = "TriggerR", max = 1024 } # Set higher-precision axis ranges (usually 255)
CamZ = { event = "TriggerL", max = 1024 }
```

### `type`
Type can be any of the following:
 - `xbox` - An Xbox Gamepad (W3 Standard Gamepad Compliant)
 - `flight` - A Flightstick
 - `playstation` - A PlayStation Gamepad (W3 Standard Gamepad Compliant)
 - `nintendo` - A Nintendo Gamepad (W3 Standard Gamepad Compliant)

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

[7]: https://github.com/libcala/stick/blob/main/LICENSE_APACHE_2_0.txt
[8]: https://www.apache.org/licenses/LICENSE-2.0
[9]: https://github.com/libcala/stick/blob/main/LICENSE_MIT.txt
[10]: https://mit-license.org/
[11]: https://github.com/libcala/stick/blob/main/LICENSE_BOOST_1_0.txt
[12]: https://www.boost.org/LICENSE_1_0.txt
[13]: mailto:aldaronlau@gmail.com
