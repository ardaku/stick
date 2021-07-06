// Stick
// Copyright © 2017-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

//! ## Getting Started
//! Add the following to your *Cargo.toml*:
//!
//! ```toml
//! [dependencies]
//! pasts = "0.8"
//! stick = "0.12"
//! ```
//!
//! ### Example
//! This example demonstrates getting joystick input and sending haptic
//! feedback (copied from *examples/haptic.rs*):
//!
//! ```rust,no_run
//! use pasts::Loop;
//! use std::task::Poll::{self, Pending, Ready};
//! use stick::{Controller, Event, Listener};
//!
//! type Exit = usize;
//!
//! struct State {
//!     listener: Listener,
//!     controllers: Vec<Controller>,
//!     rumble: (f32, f32),
//! }
//!
//! impl State {
//!     fn connect(&mut self, controller: Controller) -> Poll<Exit> {
//!         println!(
//!             "Connected p{}, id: {:016X}, name: {}",
//!             self.controllers.len() + 1,
//!             controller.id(),
//!             controller.name(),
//!         );
//!         self.controllers.push(controller);
//!         Pending
//!     }
//!
//!     fn event(&mut self, id: usize, event: Event) -> Poll<Exit> {
//!         let player = id + 1;
//!         println!("p{}: {}", player, event);
//!         match event {
//!             Event::Disconnect => {
//!                 self.controllers.swap_remove(id);
//!             }
//!             Event::MenuR(true) => return Ready(player),
//!             Event::ActionA(pressed) => {
//!                 self.controllers[id].rumble(f32::from(u8::from(pressed)));
//!             }
//!             Event::ActionB(pressed) => {
//!                 self.controllers[id].rumble(0.5 * f32::from(u8::from(pressed)));
//!             }
//!             Event::BumperL(pressed) => {
//!                 self.rumble.0 = f32::from(u8::from(pressed));
//!                 self.controllers[id].rumble(self.rumble);
//!             }
//!             Event::BumperR(pressed) => {
//!                 self.rumble.1 = f32::from(u8::from(pressed));
//!                 self.controllers[id].rumble(self.rumble);
//!             }
//!             _ => {}
//!         }
//!         Pending
//!     }
//! }
//!
//! async fn event_loop() {
//!     let mut state = State {
//!         listener: Listener::default(),
//!         controllers: Vec::new(),
//!         rumble: (0.0, 0.0),
//!     };
//!
//!     let player_id = Loop::new(&mut state)
//!         .when(|s| &mut s.listener, State::connect)
//!         .poll(|s| &mut s.controllers, State::event)
//!         .await;
//!
//!     println!("p{} ended the session", player_id);
//! }
//!
//! fn main() {
//!     pasts::block_on(event_loop());
//! }
//! ```

#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg",
    html_root_url = "https://docs.rs/stick"
)]
#![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

#[cfg(target_os = "windows")]
#[macro_use]
extern crate log;

mod ctlr;
mod event;
mod focus;
mod listener;
mod raw;

pub use ctlr::{Controller, Remap};
pub use event::Event;
pub use focus::{focus, unfocus};
pub use listener::Listener;
