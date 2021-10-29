// Copyright © 2017-2021 The Stick Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

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
