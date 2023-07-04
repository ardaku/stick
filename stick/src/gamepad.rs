// FIXME: Use in crate.

/// An event from a `Gamepad`.
enum GamepadEvent {
    /// Main / Home / Mode / XBox Button / PS3 Button
    Home(bool),
    /// Select / Back / Minus / Menu Button
    Menu(bool),
    /// Start / Forward / Plus / Play Button
    Play(bool),
    /// D-Pad Up
    Up(bool),
    /// D-Pad Down
    Down(bool),
    /// D-Pad Left
    Left(bool),
    /// D-Pad Right
    Right(bool),
    /// The primary face action button (Circle, A, 1)
    A(bool),
    /// The secondary face action button (Cross, B, 2)
    B(bool),
    /// The topmost face action button (Triangle, may be either X or Y, 3 or 4)
    Top(bool),
    /// The remaining face action button (Square, may be either X or Y, 3 or 4)
    Use(bool),
    /// Left bumper button
    BumperL(bool),
    /// Right bumper button
    BumperR(bool),
    /// The camera joystick push button
    Cam(bool),
    /// The direction joystick push button
    Dir(bool),
    /// Camera joystick X
    CamX(f32),
    /// Camera joystick Y
    CamY(f32),
    /// Direction joystick X
    DirX(f32),
    /// Direction joystick Y
    DirY(f32),
    /// Left trigger
    TriggerL(f32),
    /// Right trigger
    TriggerR(f32),
    /// Extended Gamepad: Top grip button on the left
    PaddleL(bool),
    /// Extended Gamepad: Top grip button on the right
    PaddleR(bool),
    /// Extended Gamepad: Lower grip button on the left
    GripL(bool),
    /// Extended Gamepad: Lower grip button on the right
    GripR(bool),
}
