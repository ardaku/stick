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
//! pasts = "0.7"
//! stick = "0.12"
//! ```
//!
//! ### Example
//! This example demonstrates getting joystick input and sending haptic
//! feedback (copied from *examples/haptic.rs*):
//!
//! ```rust,no_run
//! use stick::{Controller, Event};
//! use pasts::{race, wait};
//! 
//! async fn event_loop() {
//!     let mut listener = Controller::listener();
//!     let mut ctlrs = Vec::<Controller>::new();
//!     'e: loop {
//!         let event = wait![(&mut listener).await, race!(ctlrs)];
//!         match event {
//!             (_, Event::Connect(new)) => {
//!                 println!(
//!                     "Connected p{}, id: {:04X}_{:04X}_{:04X}_{:04X}, name: {}",
//!                     ctlrs.len() + 1,
//!                     new.id()[0],
//!                     new.id()[1],
//!                     new.id()[2],
//!                     new.id()[3],
//!                     new.name(),
//!                 );
//!                 ctlrs.push(*new);
//!             }
//!             (id, Event::Disconnect) => {
//!                 println!("Disconnected p{}", id + 1);
//!                 ctlrs.swap_remove(id);
//!             }
//!             (id, Event::Home(true)) => {
//!                 println!("p{} ended the session", id + 1);
//!                 break 'e;
//!             }
//!             (id, event) => {
//!                 println!("p{}: {}", id + 1, event);
//!                 match event {
//!                     Event::ActionA(pressed) => {
//!                         ctlrs[id].rumble(if pressed { 1.0 } else { 0.0 });
//!                     }
//!                     Event::ActionB(pressed) => {
//!                         ctlrs[id].rumble(if pressed { 0.3 } else { 0.0 });
//!                     }
//!                     _ => {}
//!                 }
//!             }
//!         }
//!     }
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

mod ctlr;
mod event;

#[cfg_attr(target_arch = "wasm32", path = "ffi/wasm32.rs")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    cfg_attr(target_os = "linux", path = "ffi/linux.rs"),
    cfg_attr(target_os = "android", path = "ffi/android.rs"),
    cfg_attr(target_os = "macos", path = "ffi/macos.rs"),
    cfg_attr(target_os = "ios", path = "ffi/ios.rs"),
    cfg_attr(target_os = "windows", path = "ffi/windows.rs"),
    cfg_attr(
        any(
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "bitrig",
            target_os = "openbsd",
            target_os = "netbsd"
        ),
        path = "ffi/bsd.rs"
    ),
    cfg_attr(target_os = "fuchsia", path = "ffi/fuchsia.rs"),
    cfg_attr(target_os = "redox", path = "ffi/redox.rs"),
    cfg_attr(target_os = "none", path = "ffi/none.rs"),
    cfg_attr(target_os = "dummy", path = "ffi/dummy.rs")
)]
#[allow(unsafe_code)]
mod ffi;

pub use ctlr::Controller;
pub use event::Event;
