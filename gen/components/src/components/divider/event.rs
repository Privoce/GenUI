use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerMoveEvent,
    FingerUpEvent, KeyEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GDividerEvent {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    FingerDown(FingerDownEvent),
    FingerMove(FingerMoveEvent),
    FingerHoverIn(FingerHoverEvent),
    FingerHoverOver(FingerHoverEvent),
    FingerHoverOut(FingerHoverEvent),
    FingerUp(FingerUpEvent),
    None,
}
