use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent};

use crate::event::FocusType;

#[derive(DefaultNone, Debug, Clone)]
pub enum GLabelEvent {
    HoverIn(GLabelHoverParam),
    HoverOut(GLabelHoverParam),
    Focus(GLabelFocusParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GLabelHoverParam {
    pub e: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct GLabelFocusParam {
    pub ty: FocusType,
    pub e: FingerDownEvent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GLabelState {
    Hover,
    Focus,
    None,
}
