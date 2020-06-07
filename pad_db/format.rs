#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Button {
    pub(super) code: u16,
    pub(super) event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Axis {
    pub(super) code: u8,
    pub(super) event: String,
    pub(super) max: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Trigger {
    pub(super) code: u8,
    pub(super) event: String,
    pub(super) max: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ThreeWay {
    pub(super) code: u8,
    pub(super) neg: String,
    pub(super) pos: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Wheel {
    pub(super) code: u8,
    pub(super) event: String,
}

/// A mapping for a specific pad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct PadMapping {
    // Name of the controller.
    pub(super) name: String,
    // Type of the controller.
    pub(super) r#type: String,
    // Buttons are simple on or off
    pub(super) button: Option<Vec<Button>>,
    // Signed axes are "continuous" from min to max value
    pub(super) axis: Option<Vec<Axis>>,
    // Triggers (Unsigned Axes) are "continuous" from 0 to 255
    pub(super) trigger: Option<Vec<Trigger>>,
    // Three-Way switches are either on positive, on negative, or off
    pub(super) three_way: Option<Vec<ThreeWay>>,
    // Signed relative axes are "continuous" from min to max value
    pub(super) wheel: Option<Vec<Wheel>>,
}
