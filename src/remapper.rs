use super::Input;

/// A structure to remap an `Input` to a different `Input`.
pub struct Remapper {
	pub(crate) id: u32,
	pub(crate) remapper: fn((usize, Input)) -> (usize, Input),
}

impl Remapper {
	/// Create a new remapping.  `id` is which joystick type should remap an
	/// input, 0 for all.  `remapper` is the function to do the remapping.
	pub fn new(id: u32, remapper: fn((usize, Input)) -> (usize, Input))
		-> Self
	{
		Self { id, remapper }
	}
}
