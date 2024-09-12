use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GCheckboxEvent {
    Clicked(GCheckboxClickedParam),
    Hover(GCheckboxHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GCheckboxClickedParam {
    pub value: bool,
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GCheckboxHoverParam {
    pub value: bool,
    pub e: FingerHoverEvent,
}
