// Copyright © 2017-2022 The Stick Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

#![allow(unsafe_code)]

use lookit::It;

// Choose platform driver implementation.
#[allow(unused_attributes)]
#[cfg_attr(target_os = "linux", path = "../linux/linux.rs")]
#[path = "unsupported.rs"]
mod driver;

// Import the device type from the target platform.
pub(crate) use driver::Controller as PlatformController;

// Single required method for each platform.
pub(crate) fn connect(it: It) -> Option<(u64, String, PlatformController)> {
    driver::connect(it)
}

pub(crate) trait Support {
    fn enable(self);
    fn disable(self);
    fn rumble(self, controller: &mut PlatformController, left: f32, right: f32);
    fn event(self, device: &mut PlatformController) -> Option<crate::Event>;
}

pub(crate) fn platform() -> impl Support {
    driver::platform()
}
