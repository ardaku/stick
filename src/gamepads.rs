use std::task::Context;
use std::future::Future;

use crate::StdGamepad;
use crate::ffi::NativeGamepads;

/// All connected joysticks / gamepads / controllers.
///
/// The controllers are remapped mostly according to
/// [the w3c Gamepad specification](https://w3c.github.io/gamepad/#remapping).
/// 
/// Sometimes, positional button mapping is not what you want for the four main
/// buttons, but rather Accept/Cancel Toggle/Action.  Accept/Cancel are always
/// in the bottom right because it's the easiest to reach, but which is the
/// bottom and which is right vary by controller.  It generally makes the most
/// sense for Action to be the left button, and Toggle to be the top button.
pub struct Gamepads(NativeGamepad);

impl Gamepads {
    /// Initiate a connection with the plugged-in gamepads.
    pub fn new(emulated: &[Box<&dyn StdGamepad>]) -> Self {
        Gamepads(NativeGamepad::new())
    }
}

impl Future for Gamepads {
    type Output = &mut StdGamepad;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.poll(cx)
    }
}
