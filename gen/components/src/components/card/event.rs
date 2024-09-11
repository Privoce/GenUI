use makepad_widgets::{
    DefaultNone, FingerDownEvent, FingerHoverEvent, FingerMoveEvent, FingerUpEvent, KeyEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GCardEvent {
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
