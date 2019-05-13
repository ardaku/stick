# Stick
## About
Stick is a Rust library for getting joystick, gamepad, or other controller input.

## Features
- Get controller input (Linux)
- Remap controller input (Linux)
- Connect to multiple controllers (Linux)
- CONTROLLER: GameCube controllers (with MAYFLASH adapter)
- CONTROLLER: Flight simulator joystick
- CONTROLLER: XBox controller
- CONTROLLER: PlayStation controller

## Getting Started
```rs
// jstest.rs
use stick::Port;

fn main() {
    // Connect to all devices.
    let mut port = Port::new();

    // Loop showing state of all devices.
    loop {
        // Cycle through all currently plugged in devices.
        for i in 0..port.update() {
            let device = port.get(i);
            println!("{}: {}", i, device);
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
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
- [Website](https://jeronaldaron.github.io/stick)
- [Cargo](https://crates.io/crates/stick)
- [Documentation](https://docs.rs/stick)
- [Change Log](https://jeronaldaron.github.io/stick/CHANGELOG)
- [Contributors](https://jeronaldaron.github.io/stick/CONTRIBUTORS)
- [Code of Conduct](https://jeronaldaron.github.io/stick/CODEOFCONDUCT)
- [Join Cala on Zulip](https://plopgrizzly.zulipchat.com/join/pp13s6clnexk03tvlnrtjvi1)
