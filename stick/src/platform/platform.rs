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
