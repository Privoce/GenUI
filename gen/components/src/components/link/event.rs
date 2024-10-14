use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

use super::types::LinkType;

#[derive(Clone, Debug, DefaultNone)]
pub enum GLinkEvent {
    HoverIn(GLinkHoverParam),
    HoverOut(GLinkHoverParam),
    Clicked(GLinkClickedParam),
    Focus(GLinkFocusParam),
    FocusLost(GLinkFocusLostParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GLinkClickedParam {
    pub href: Option<String>,
    pub ty: LinkType,
    pub e: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct GLinkHoverParam {
    pub e: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct GLinkFocusParam {
    pub e: FingerDownEvent,
}

#[derive(Debug, Clone)]
pub struct GLinkFocusLostParam {
    pub e: FingerUpEvent,
}
