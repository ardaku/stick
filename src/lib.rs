// Copyright Jeron Aldaron Lau 2017 - 2020.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

//! ## Getting Started
//! Add the following to your *Cargo.toml*:
//!
//! ```toml
//! [dependencies]
//! pasts = "0.6"
//! stick = "0.11"
//! ```
//!
//! ### Example
//! This example demonstrates getting joystick input and sending haptic
//! feedback (copied from *examples/haptic.rs*):
//!
//! ```rust,no_run
//! use pasts::prelude::*;
//! use stick::{Controller, Event};
//!
//! async fn event_loop() {
//!     let mut listener = Controller::listener();
//!     let mut ctlrs = Vec::<Controller>::new();
//!     'e: loop {
//!         match poll![listener, poll!(ctlrs)].await.1 {
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
//!     exec!(event_loop());
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

#[cfg(target_os = "windows")]
#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "windows")]
extern crate winapi;

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
