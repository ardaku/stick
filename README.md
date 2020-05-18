# Stick

#### Platform-agnostic asynchronous gamepad library for Rust

[![Build Status](https://api.travis-ci.org/libcala/stick.svg?branch=master)](https://travis-ci.org/libcala/stick)
[![Docs](https://docs.rs/stick/badge.svg)](https://docs.rs/stick)
[![crates.io](https://img.shields.io/crates/v/stick.svg)](https://crates.io/crates/stick)

Stick supports getting controller input, as well as using rumble haptic effects.

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


## Getting Started
Add the following to your `Cargo.toml`.

```toml
[dependencies]
pasts = "0.4"
stick = "0.9"
```

### Example
This example can be used to test joystick input and haptic feedback.

```rust,no_run
use pasts::{CvarExec, prelude::*};
use stick::{Event, Gamepad, Port};

async fn event_loop() {
    let mut port = Port::new();
    let mut gamepads = Vec::<Gamepad>::new();
    'e: loop {
        match [port.fut(), gamepads.select().fut()]
            .select()
            .await
            .1
        {
            (_, Event::Connect(gamepad)) => {
                println!(
                    "Connected p{}, id: {:X}, name: {}",
                    gamepads.len() + 1,
                    gamepad.id(),
                    gamepad.name(),
                );
                gamepads.push(*gamepad);
            }
            (id, Event::Disconnect) => {
                println!("Disconnected p{}", id + 1);
                gamepads.swap_remove(id);
            }
            (id, Event::Quit) => {
                println!("p{} ended the session", id + 1);
                break 'e;
            }
            (id, event) => {
                println!("p{}: {}", id + 1, event);
                match event {
                    Event::Accept(pressed) => {
                        gamepads[id].rumble(if pressed {
                            0.25
                        } else {
                            0.0
                        });
                    }
                    Event::Cancel(pressed) => {
                        gamepads[id].rumble(if pressed {
                            1.0
                        } else {
                            0.0
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    static EXECUTOR: CvarExec = CvarExec::new();

    EXECUTOR.block_on(event_loop())
}
```

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
