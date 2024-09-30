use makepad_widgets::{
    DefaultNone, FingerDownEvent, FingerHoverEvent, FingerMoveEvent, FingerUpEvent, HeapLiveIdPath,
    KeyEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GViewEvent {
    KeyDown(GViewKeyEventParam),
    KeyUp(GViewKeyEventParam),
    FingerDown(GViewFingerDownParam),
    FingerMove(GViewFingerMoveParam),
    FingerHoverIn(GViewFingerHoverParam),
    FingerHoverOver(GViewFingerHoverParam),
    FingerHoverOut(GViewFingerHoverParam),
    FingerUp(GViewFingerUpParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GViewKeyEventParam {
    pub e: KeyEvent,
    pub path: HeapLiveIdPath,
}

#[derive(Debug, Clone)]
pub struct GViewFingerDownParam {
    pub e: FingerDownEvent,
    pub path: HeapLiveIdPath,
}
#[derive(Debug, Clone)]
pub struct GViewFingerMoveParam {
    pub e: FingerMoveEvent,
    pub path: HeapLiveIdPath,
}
#[derive(Debug, Clone)]
pub struct GViewFingerHoverParam {
    pub e: FingerHoverEvent,
    pub path: HeapLiveIdPath,
}
#[derive(Debug, Clone)]
pub struct GViewFingerUpParam {
    pub e: FingerUpEvent,
    pub path: HeapLiveIdPath,
}
