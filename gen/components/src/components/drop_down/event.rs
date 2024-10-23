use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GDropDownEvent {
    Changed(GDropDownChangedParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GDropDownChangedParam {
    pub e: GDropDownToggleKind,
    pub opened: bool,
}

#[derive(Debug, Clone, Default)]
pub enum GDropDownToggleKind {
    Click(FingerUpEvent),
    Hover(FingerHoverEvent),
    Press(FingerDownEvent),
    // KetFocusLost(KeyFocusEvent),
    #[default]
    Other,
}
