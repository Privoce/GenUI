use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

use crate::event::FocusType;

#[derive(Clone, Debug, DefaultNone)]
pub enum GButtonEvent {
    HoverIn(GButtonHoverParam),
    HoverOut(GButtonHoverParam),
    Clicked(GButtonClickedParam),
    Focus(GButtonFocusParam),
    FocusLost(GButtonFocusLostParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GButtonHoverParam {
    pub e: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct GButtonFocusParam {
    pub ty: FocusType,
    pub e: FingerDownEvent,
}

#[derive(Debug, Clone)]
pub struct GButtonFocusLostParam {
    pub e: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct GButtonClickedParam {
    pub e: FingerUpEvent,
}