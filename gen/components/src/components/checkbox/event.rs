use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GCheckBoxEvent {
    Clicked(GCheckBoxClickedParam),
    Hover(GCheckBoxHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GCheckBoxClickedParam {
    pub value: bool,
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GCheckBoxHoverParam {
    pub value: bool,
    pub e: FingerHoverEvent,
}
