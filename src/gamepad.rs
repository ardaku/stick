use crate::Event;

/// Trait for implementing the "standard gamepad".
pub trait Gamepad {
    pub async fn event() -> Event;
}
