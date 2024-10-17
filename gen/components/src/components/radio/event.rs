use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GRadioEvent {
    Clicked(GRadioClickedParam),
    Hover(GRadioHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GRadioClickedParam {
    pub value: Option<String>,
    pub selected: bool,
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GRadioHoverParam {
    pub value: Option<String>,
    pub selected: bool,
    pub e: FingerHoverEvent,
}

// -------------------------------------------------------------------------

#[derive(Debug, Clone, DefaultNone)]
pub enum GRadioGroupEvent {
    Changed(GRadioGroupEventParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GRadioGroupEventParam {
    /// The index of the selected radio.
    pub selected: usize,
    /// The value of the selected radio.
    pub value: Option<String>,
    pub e: FingerUpEvent,
}
