use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Debug, Clone, DefaultNone)]
pub enum GSvgEvent {
    Clicked(GSvgClickedParam),
    HoverIn(GSvgHoverParam),
    HoverOut(GSvgHoverParam),
    Focus(GSvgFocusParam),
    FocusLost(GSvgFocusLostParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GSvgHoverParam {
    pub e: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct GSvgClickedParam {
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GSvgFocusParam {
    pub e: FingerDownEvent,
}

#[derive(Clone, Debug)]
pub struct GSvgFocusLostParam {
    pub e: FingerUpEvent,
}
