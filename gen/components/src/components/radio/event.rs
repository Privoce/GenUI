use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

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