use makepad_widgets::{
    DefaultNone, FingerDownEvent, FingerHoverEvent, FingerMoveEvent, FingerUpEvent, HeapLiveIdPath,
    KeyEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GCardEvent {
    KeyDown(GCardKeyEventParam),
    KeyUp(GCardKeyEventParam),
    FingerDown(GCardFingerDownParam),
    FingerMove(GCardFingerMoveParam),
    FingerHoverIn(GCardFingerHoverParam),
    FingerHoverOver(GCardFingerHoverParam),
    FingerHoverOut(GCardFingerHoverParam),
    FingerUp(GCardFingerUpParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GCardKeyEventParam {
    pub e: KeyEvent,
    pub path: HeapLiveIdPath,
}

#[derive(Debug, Clone)]
pub struct GCardFingerDownParam {
    pub e: FingerDownEvent,
    pub path: HeapLiveIdPath,
}
#[derive(Debug, Clone)]
pub struct GCardFingerMoveParam {
    pub e: FingerMoveEvent,
    pub path: HeapLiveIdPath,
}
#[derive(Debug, Clone)]
pub struct GCardFingerHoverParam {
    pub e: FingerHoverEvent,
    pub path: HeapLiveIdPath,
}
#[derive(Debug, Clone)]
pub struct GCardFingerUpParam {
    pub e: FingerUpEvent,
    pub path: HeapLiveIdPath,
}
