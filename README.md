# Stick
## About
Stick is a cross-platform Rust library for getting joystick, gamepad, or other controller input.

## Features
- Asynchronously get events from multiple joysticks on one thread, joystick state shared via atomics to the other threads.
- Get controller input (Linux)
- Remap controller input (Linux)
- Connect to multiple controllers (Linux)
- CONTROLLER: GameCube controllers (with MAYFLASH adapter)
- CONTROLLER: Flight simulator joystick
- CONTROLLER: XBox controller
- CONTROLLER: PlayStation controller

## Getting Started
```rust
// jstest.rs
use stick::Port;

fn main() {
    // Connect to all devices.
    let mut port = Port::new();

    // Loop showing state of all devices.
    loop {
        // Cycle through all currently plugged in devices.
        let id = if let Some(a) = port.poll() {
            a
        } else {
            continue;
        };

        if let Some(state) = port.get(id) {
            println!("{}: {}", id, state);
        }
    }
}
```

## TODO
- Better haptic (vibration) support
- CONTROLLER: Emulated joystick
- CONTROLLER: Probably some other controllers
- PLATFORM: Windows
- PLATFORM: MacOS
- PLATFORM: Android
- PLATFORM: Nintendo switch

## Links
- [Website](https://aldarobot.plopgrizzly.com/stick)
- [Cargo](https://crates.io/crates/stick)
- [Documentation](https://docs.rs/stick)
- [Change Log](https://aldarobot.plopgrizzly.com/stick/CHANGELOG)
- [Contributors](https://aldarobot.plopgrizzly.com/stick/CONTRIBUTORS)
- [Code of Conduct](https://aldarobot.plopgrizzly.com/stick/CODEOFCONDUCT)
- [Join Cala on Zulip](https://plopgrizzly.zulipchat.com/join/pp13s6clnexk03tvlnrtjvi1)
