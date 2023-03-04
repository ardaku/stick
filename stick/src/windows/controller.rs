use std::rc::Rc;
use crate::Event;
use winapi::shared::minwindef::DWORD;
use super::XInputHandle;
use crate::ctlr::lookit::It;

pub(crate) struct Controller {
    pub(crate) xinput: Rc<XInputHandle>,
    pub(crate) device_id: u8,
    pub(crate) pending_events: Vec<Event>,
    pub(crate) last_packet: DWORD,
}

impl Controller {
    #[allow(unused)]
    fn new(device_id: u8, xinput: Rc<XInputHandle>) -> Self {
        Self {
            xinput,
            device_id,
            pending_events: Vec::new(),
            last_packet: 0,
        }
    }

	/// Stereo rumble effect (left is low frequency, right is high frequency).
	pub(super) fn rumble(&mut self, left: f32, right: f32) {
		self.xinput
			.set_state(
				self.device_id as u32,
				(u16::MAX as f32 * left) as u16,
				(u16::MAX as f32 * right) as u16,
			)
			.unwrap()
	}
}

pub(crate) fn connect(it: It) -> Option<(u64, String, Controller)> {
	let name = "XInput Controller";
	let controller = Controller::new(it.id(), todo!());
	Some((0, name.to_string(), controller))
}
