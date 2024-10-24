use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(DefaultNone, Debug, Clone)]
pub enum GLabelEvent {
    HoverIn(GLabelHoverParam),
    HoverOut(GLabelHoverParam),
    Focus(GLabelFocusParam),
    FocusLost(GLabelFocusLostParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GLabelHoverParam {
    pub e: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct GLabelFocusParam {
    pub e: FingerDownEvent,
}

#[derive(Debug, Clone)]
pub struct GLabelFocusLostParam {
    pub e: FingerUpEvent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GLabelState {
    Hover,
    Focus,
    None,
}
