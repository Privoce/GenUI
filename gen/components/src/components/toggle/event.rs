use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GToggleEvent {
    Clicked(GToggleClickedParam),
    HoverIn(GToggleHoverParam),
    HoverOut(GToggleHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GToggleClickedParam {
    pub selected: bool,
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GToggleHoverParam {
    pub selected: bool,
    pub e: FingerHoverEvent,
}
