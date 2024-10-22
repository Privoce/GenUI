use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

use super::types::GToolButtonType;

#[derive(Clone, Debug, DefaultNone)]
pub enum GToolButtonEvent {
    HoverIn(GToolButtonHoverParam),
    HoverOut(GToolButtonHoverParam),
    Clicked(GToolButtonClickedParam),
    Focus(GToolButtonFocusParam),
    FocusLost(GToolButtonFocusLostParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GToolButtonClickedParam {
    pub e: Option<FingerUpEvent>,
    pub icon_type: GToolButtonType,
}

#[derive(Debug, Clone)]
pub struct GToolButtonHoverParam {
    pub e: Option<FingerHoverEvent>,
}

#[derive(Debug, Clone)]
pub struct GToolButtonFocusParam {
    pub e: Option<FingerDownEvent>,
}

#[derive(Debug, Clone)]
pub struct GToolButtonFocusLostParam {
    pub e: Option<FingerUpEvent>,
}
