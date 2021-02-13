# Stick
Folders in this repository:
 - `stick/`: The Stick crate.
 - `stick_db/`: The Stick Public Domain database of controllers (not limited
   to game controllers).
<!-- - `fmt/`: Formatting for the Stick database.
 - `map/`: Code to import the SDL Game Controller ZLib database of controllers. -->
 - `gcdb/`: Git Submodule to grab optional SDL mappings from.
 - `xtask/`: Scripts run with `cargo-xtask`.

## Xtask
Stick uses the [xtask](https://github.com/matklad/cargo-xtask) repository model
to have various scripts written in Rust process controller data, also allowing
serde to not be a direct dependency.

### Available Commands
 - `cargo xtask`, `cargo xtask --help` - Print help
 - `cargo xtask codegen` - Generate the embeddable bytecode databases.
 - `cargo xtask fmt` - Auto-format and sort the TOML files in `stick_db/`.

## Bytecode Format
Stick reads the bytecode format that the library user inputs into the library.
There's the databases that can be enabled/disabled with the `stickdb` and `gcdb`
features.  All numbers are little endian.

 - `version: 0x00`: Version of stick format.
 - `len: u24`: Number of controllers.
 - `filter: [(b64, u16); len]`: Controller filter.
   - `wildcard: b4`: Mask for which parts of the ID are significant.
   - `bus: b12`: The hardware type. (bit 0 mask)
   - `vendor: b16`: Who made the controller. (bit 1 mask)
   - `product: b16`: Which controller is it. (bit 2 mask)
   - `version: b16`: Which revision is it. (bit 3 mask)
   - `jump: u16`: How many bytes does the controller spec need (must be %2==0).
 - `layout: u16`: Controller Layout
   - `0x0000`: Unknown Names
   - `0x0010`: W3C Standard Gamepad / Nintendo Names
   - `0x0011`: GameCube (Subset of 0x01 buttons)
   - `0x0020`: W3C Standard Gamepad / NSEW Names
   - `0x0030`: W3C Standard Gamepad / XBox Names
   - `0x0040`: W3C Standard Gamepad / PlayStation Names
   - `0x0050`: Thrustmaster Warthog Names
   - `0x0060`: Simple Flightstick Names
 - `buttons: [(…)]`: List of buttons
   - `reserved`: NonZeroU4: RESERVED FOR FUTURE USE (must be 1)
   - `btn_id: u12`: Platform-specific button ID
   - `event: u16`: A pre-defined stick Event
 - `axes: [(…)]`: List of axes
   - `reserved: NonZeroU8`: RESERVED FOR FUTURE USE (must be 1)
   - `axis_id: u8`: Platform-specific axis ID
   - `event: u16`: A pre-defined stick Event
   - `deadzone: u16`: Normalized 0-1 deadzone area.
   - `maximum: u16`: The maximum value returned (clamped, opposite of deadzone).
 - `trigger: [(…)]`: List of triggers
   - `invert: NonZeroU8`: Whether or not the trigger is inverted (0 and 1 flip)
     - `0x80`: Not inverted
     - `0x81`: Inverted
     - `_`: RESERVED FOR FUTURE USE
   - `axis_id: u8`: Platform-specific axis ID
   - `event: u16`: A pre-defined stick Event
   - `deadzone: u16`: Normalized 0-1 deadzone area.
   - `maximum: u16`: The maximum value returned (clamped, opposite of deadzone).
 - `wheel: [(…)]`:
   - `reserved: NonZeroU8`: RESERVED FOR FUTURE USE (must be 1)
   - `axis_id: u8`: Platform-specific axis ID
   - `event: u16`: A pre-defined stick Event
 - `three_way: [(…)]`:
   - `reserved: NonZeroU8`: RESERVED FOR FUTURE USE (must be 1)
   - `axis_id: u8`: Platform-specific axis ID
   - `negative: u16`: A pre-defined stick Event
   - `positive: u16`: A pre-defined stick Event
 - `type: b8`: Joystick Type
   - `0x00`: Gamepad Controller (Modern gaming controller)
   - `0x01`: Joystick Controller (Old style gaming controller)
   - `0x02`: Flightstick Controller (Flight simulation type controller)
   - `0x03`: Remote (Wii-Style Controller)
   - `0x04`~`0xFF`: RESERVED FOR FUTURE USE
 - `name: Text`: UTF-8 string name of joystick (jump value to determine length).
 - `pad: Opt<0b00>`: Optional null termination to align to 16-bit words.

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
