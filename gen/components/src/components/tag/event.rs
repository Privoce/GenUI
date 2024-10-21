use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Debug, Clone, DefaultNone)]
pub enum GTagEvent {
    Clicked(GTagClickedParam),
    HoverIn(GTagHoverParam),
    HoverOut(GTagHoverParam),
    Closed(GTagClosedParam),
    Focus(GTagFocusParam),
    FocusLost(GTagFocusLostParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GTagClickedParam {
    /// tag text
    pub text: String,
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GTagHoverParam {
    pub e: Option<FingerHoverEvent>,
}

#[derive(Clone, Debug)]
pub struct GTagClosedParam {
    pub text: String,
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GTagFocusParam {
    pub e: Option<FingerDownEvent>,
}

#[derive(Clone, Debug)]
pub struct GTagFocusLostParam {
    pub e: Option<FingerUpEvent>,
}
