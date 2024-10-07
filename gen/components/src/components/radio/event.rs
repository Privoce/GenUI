use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GRadioEvent {
    Clicked(GRadioClickedParam),
    Hover(GRadioHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GRadioClickedParam {
    pub value: bool,
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GRadioHoverParam {
    pub value: bool,
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
    pub selected: usize,
    pub e: FingerUpEvent,
}
