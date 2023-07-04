/// Window grab focus, re-enable events if they were disabled.
pub fn focus() {
    crate::raw::GLOBAL.with(|g| g.enable());
}

/// Window ungrab focus, disable events.
pub fn unfocus() {
    crate::raw::GLOBAL.with(|g| g.disable());
}
