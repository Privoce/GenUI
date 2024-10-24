use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerMoveEvent,
    FingerUpEvent, KeyEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GViewEvent {
    KeyDown(GViewKeyEventParam),
    KeyUp(GViewKeyEventParam),
    Focus(GViewFocusParam),
    Drag(GViewDragParam),
    HoverIn(GViewHoverParam),
    HoverOver(GViewHoverParam),
    HoverOut(GViewHoverParam),
    Clicked(GViewClickedParam),
    FocusLost(GViewFocusLostParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GViewKeyEventParam {
    pub e: KeyEvent,
}

#[derive(Debug, Clone)]
pub struct GViewFocusParam {
    pub e: FingerDownEvent,
}
#[derive(Debug, Clone)]
pub struct GViewHoverParam {
    pub e: FingerHoverEvent,
}
#[derive(Debug, Clone)]
pub struct GViewClickedParam {
    pub e: FingerUpEvent,
}
#[derive(Debug, Clone)]
pub struct GViewFocusLostParam {
    pub e: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct GViewDragParam {
    pub e: FingerMoveEvent,
}
