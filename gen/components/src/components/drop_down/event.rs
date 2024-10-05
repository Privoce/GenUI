use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent, KeyFocusEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GDropDownEvent {
    Toggle(GDropDownEventParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GDropDownEventParam {
    pub e: GDropDownToggleKind,
    pub opened: bool,
}

#[derive(Debug, Clone, Default)]
pub enum GDropDownToggleKind {
    Clicked(FingerUpEvent),
    Hover(FingerHoverEvent),
    KetFocusLost(KeyFocusEvent),
    #[default]
    Other,
}
