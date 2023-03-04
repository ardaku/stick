#![allow(unsafe_code)]

use std::task::{Context, Poll};

use crate::Event;

// Choose platform driver implementation.
#[allow(unused_attributes)]
#[cfg_attr(target_os = "linux", path = "../linux/linux.rs")]
#[cfg_attr(target_os = "windows", path = "../windows/windows.rs")]
#[path = "unsupported.rs"]
mod driver;

/// Controller ID.
pub(crate) struct CtlrId(u32);

/// Required platform support trait.
pub(crate) trait Support {
    /// Window gained focus, start receiving events.
    fn enable(self);
    /// Window lost focus, stop receiving events.
    fn disable(self);
    /// Set left and right rumble value for controller.
    fn rumble(self, ctlr: &CtlrId, left: f32, right: f32);
    /// Attempt getting a new event from a connected controller.
    fn event(self, ctlr: &CtlrId, cx: &mut Context<'_>) -> Poll<Event>;
    /// Attempt connecting to a new controller.
    fn connect(self, cx: &mut Context<'_>) -> Poll<(u64, String, CtlrId)>;
}

/// Get platform support implementation.
///
/// Each platform must implement this function to work.
pub(crate) fn platform() -> impl Support {
    driver::platform()
}
