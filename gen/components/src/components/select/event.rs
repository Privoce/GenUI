use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GSelectItemEvent {
    Clicked(GSelectItemClickedParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GSelectItemClickedParam {
    pub selected: bool,
    pub e: FingerUpEvent,
}
// -------------------------------------------------------------------------
