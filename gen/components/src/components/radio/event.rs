use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GRadioEvent {
    Clicked(GRadioClickedParam),
    HoverIn(GRadioHoverParam),
    HoverOut(GRadioHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GRadioClickedParam {
    pub value: Option<String>,
    pub selected: bool,
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GRadioHoverParam {
    pub e: Option<FingerHoverEvent>,
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
    pub e: Option<FingerUpEvent>,
}
