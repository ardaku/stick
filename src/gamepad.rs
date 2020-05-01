use crate::Event;

/// A w3c "Standard Gamepad".
pub struct Gamepad(crate::ffi::Gamepad);

impl Future for &mut Gamepad {
    type Output = Event;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.poll(cx)
    }
}

/// Trait for implementing the "standard gamepad".
pub trait StdGamepad {
    pub async fn event(&mut self) -> Event;
}
