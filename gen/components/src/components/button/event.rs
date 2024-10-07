use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GButtonEvent {
    Hover(FingerHoverEvent),
    Clicked(FingerUpEvent),
    Released(FingerUpEvent),
    Pressed(FingerDownEvent),
    None,
}
