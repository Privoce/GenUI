use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GTagEvent {
    Clicked(GTagClickedParam),
    Hover(GTagHoverParam),
    Close(String),
    None,
}

#[derive(Clone, Debug)]
pub struct GTagClickedParam {
    /// tag text
    pub text: String,
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GTagHoverParam {
    pub text: String,
    pub e: FingerHoverEvent,
}
