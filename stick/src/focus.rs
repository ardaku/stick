use crate::platform::Support;

/// Window grab focus, re-enable events if they were disabled.
pub fn focus() {
    crate::platform::platform().enable();
}

/// Window ungrab focus, disable events.
pub fn unfocus() {
    crate::platform::platform().disable();
}
