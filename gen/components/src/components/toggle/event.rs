use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GToggleEvent {
    Clicked(GToggleClickedParam),
    Hover(GToggleHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GToggleClickedParam {
    pub value: bool,
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GToggleHoverParam {
    pub value: bool,
    pub e: FingerHoverEvent,
}
