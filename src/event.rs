/// An event on the "Standard Gamepad" from w3c shown below.
///
/// ![Standard Gamepad](https://w3c.github.io/gamepad/standard_gamepad.svg)
pub enum Event {
    /// Bottom right cluster (A / Circle / Return / Right Click).
    Accept(bool),
    /// Bottom right cluster (B / X / Shift).
    Cancel(bool),
    /// Leftmost button in right cluster (Y / X / Square / Left Click).
    Common(bool),
    /// Topmost button in right cluster (X / Y / Triangle / Space).
    Action(bool),

    /// Up arrow / D-pad
    Up(bool),
    /// Down arrow / D-pad
    Down(bool),
    /// Left arrow / D-pad
    Left(bool),
    /// Right arrow / D-pad
    Right(bool),

    /// Back / Select Button (Escape)
    Back(bool),
    /// Forward / Start Button (Tab)
    Forward(bool),

    /// Near L - "Inventory" (E)
    L(bool),
    /// Near R - "Use" (R)
    R(bool),

    /// Far L Throttle - "Sneak" (Ctrl)
    Lz(i8),
    /// Far R Throttle - "Precision Action" (Alt)
    Rz(i8),

    /// Right Joystick (A / D)
    MotionH(i8),
    /// Left Joystick (W / S)
    MotionV(i8),
    /// Left Joystick (Mouse X Position)
    CameraH(i8),
    /// Right Joystick (Mouse Y Position)
    CameraV(i8),

    /// Left Joystick Button (Middle Click)
    MotionButton(bool),
    /// Right Joystick Button (F)
    CameraButton(bool),

    /// Home button (Target platform application close)
    Exit,
}
