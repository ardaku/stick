// button.rs -- Stick
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use std::fmt;

/// A Button for a Controller
#[derive(PartialEq, Copy, Clone)]
pub enum Button {
	/// Accept (A Button / Left Top Button - Missle / Circle)
	Accept,
	/// Cancel (B Button / Side Button / Cross)
	Cancel,
	/// Execute (X Button / Trigger / Triangle)
	Execute,
	/// Action (Y Button / Right Top Button / Square)
	Action,
	/// Left Button (0: L Trigger, 1: LZ / L Bumper).  0 is
	/// farthest away from user, incrementing as buttons get closer.
	L(u8),
	/// Right Button (0: R Trigger, 1: Z / RZ / R Bumper). 0 is
	/// farthest away from user, incrementing as buttons get closer.
	R(u8),
	/// Pause Menu (Start Button)
	Menu,
	/// Show Controls (Guide on XBox, Select on PlayStation).  Use as
	/// alternative for Menu -> "Controls".
	Controls,
	/// Exit This Screen (Back on XBox).  Use as alternative for
	/// Menu -> "Quit" or Cancel, depending on situation.
	Exit,
	/// HAT/DPAD Up Button
	Up,
	/// HAT/DPAD Down Button
	Down,
	/// Hat/D-Pad left button
	Left,
	/// Hat/D-Pad right button.
	Right,
	/// Movement stick Push
	MoveStick,
	/// Camera stick Push
	CamStick,
	/// Unknown Button
	Unknown,
}

impl fmt::Display for Button {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Button::Accept => write!(f, "Accept"),
			Button::Cancel => write!(f, "Cancel"),
			Button::Execute => write!(f, "Execute"),
			Button::Action => write!(f, "Action"),
			Button::L(a) => write!(f, "L-{}", a),
			Button::R(a) => write!(f, "R-{}", a),
			Button::Menu => write!(f, "Menu"),
			Button::Controls => write!(f, "Controls"),
			Button::Exit => write!(f, "Exit"),
			Button::Up => write!(f, "Up"),
			Button::Down => write!(f, "Down"),
			Button::Left => write!(f, "Left"),
			Button::Right => write!(f, "Right"),
			Button::MoveStick => write!(f, "Movement Stick Push"),
			Button::CamStick => write!(f, "Camera Stick Push"),
			Button::Unknown => write!(f, "Unknown"),
		}
	}
}
