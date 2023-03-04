//! File I/O

use std::io::{BufReader};
use std::fs::File;
use lookit::It;
use std::os::unix::io::{AsRawFd};
use smelling_salts::linux::{Device, Watcher};

use crate::Event;

pub(crate) struct Controller {
    pub(crate) device: Device,
    pub(crate) queued: Option<Event>,
    pub(crate) stream: BufReader<File>,
    pub(crate) abs_ranges: [super::evdev::AbsRange; super::evdev::ABS_MAX],
    pub(crate) rumble: i16,
}

pub(crate) fn connect(it: It) -> Option<(u64, String, Controller)> {
	// Some controllers may not have haptic force feedback while others might
	// ONLY have haptic force feedback and no controls.
    let file = it.file_open() // Try Read & Write first
        .or_else(|it| it.file_open_r()) // Then Readonly second
        .or_else(|it| it.file_open_w()) // Then Writeonly third
        .ok()?;
    let device = file.as_raw_fd();
    dbg!(device);
    let stream = BufReader::new(file);
    let abs_ranges = super::evdev::AbsRange::query(device);
    let watcher = Watcher::new().input();

    // Cache some information about the controller.
    let id = super::evdev::hardware_id(device);
    let name = super::evdev::hardware_name(device);
    let rumble = super::haptic::joystick_haptic(device, -1, 0.0, 0.0);

    // Return controller information.
    Some((
        id,
        name,
        Controller {
            queued: None,
            stream: stream,
            device: Device::new(device, watcher, true),
            abs_ranges,
            rumble,
        }
    ))
}
