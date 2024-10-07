use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

use super::types::GToolButtonType;

#[derive(Clone, Debug, DefaultNone)]
pub enum GToolButtonEvent {
    Hover(FingerHoverEvent),
    Clicked(GToolButtonClickParam),
    Pressed(FingerDownEvent),
    None,
}

#[derive(Clone, Debug)]
pub struct GToolButtonClickParam {
    pub e: FingerUpEvent,
    pub mode: GToolButtonType,
}
