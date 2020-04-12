use crate::Gamepad;
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
pub struct Gamepads {
    
}

impl Gamepads {
    /// Initiate a connection with the plugged-in gamepads.
    pub fn new(emulated: &[Box<&dyn Gamepad>]) -> Self {
        Gamepads {
            
        }
    }
}
