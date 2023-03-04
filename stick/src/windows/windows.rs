use self::controller::Controller;
use self::xinput::XInputHandle;
use super::{CtlrId, Support};
use crate::Event;
use std::mem::MaybeUninit;
use std::sync::Once;
use std::sync::atomic::{AtomicU8, Ordering};
use std::task::{Context, Poll};
use std::rc::Rc;

mod controller;
mod xinput;

static mut PLATFORM: MaybeUninit<Platform> = MaybeUninit::uninit();
static ONCE: Once = Once::new();
static CONNECTED: AtomicU8 = AtomicU8::new(0);
static READY: AtomicU8 = AtomicU8::new(0);

pub(super) struct Platform(Option<Rc<XInputHandle>>);

impl Support for &Platform {
    fn enable(self) {
		if let Some(ref xinput) = self.0 {
			unsafe { (xinput.xinput_enable)(true as _) };
		}
    }

    fn disable(self) {
		if let Some(ref xinput) = self.0 {
			unsafe { (xinput.xinput_enable)(false as _) };
		}
    }

    fn rumble(self, ctlr: &CtlrId, left: f32, right: f32) {
        todo!()
    }

	fn event(self, ctlr: &CtlrId, cx: &mut Context<'_>) -> Poll<Event> {
		todo!()
	}

	fn connect(self, cx: &mut Context<'_>) -> Poll<(u64, String, CtlrId)> {
		// Early return optimization if timeout hasn't passed yet.
		// FIXME

		// DirectInput only allows for 4 controllers
		let connected = CONNECTED.load(Ordering::Relaxed);
		for id in 0..4 {
			let mask = 1 << id;
			let was_connected = (connected & mask) != 0;
			if let Some(ref xinput) = self.0 {
				if xinput.get_state(id).is_ok() {
					CONNECTED.fetch_or(mask, Ordering::Relaxed);
					if !was_connected {
						// we have a new device!
						return Poll::Ready(CtlrId(id));
					}
				} else {
					// set deviceto unplugged
					CONNECTED.fetch_and(!mask, Ordering::Relaxed);
				}
			}
		}

        xinput::register_wake_timeout(100, cx.waker());

        Poll::Pending
	}
}

pub(super) fn platform() -> &'static Platform {
	ONCE.call_once(|| unsafe {
		PLATFORM = MaybeUninit::new(Platform(if let Ok(xinput) = XInputHandle::load_default() {
			Some(xinput)
		} else {
			None
		}));
	});

	unsafe {
		PLATFORM.assume_init_ref()
	}
}
