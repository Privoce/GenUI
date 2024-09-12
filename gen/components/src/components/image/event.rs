use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GImageEvent {
    Hover(FingerHoverEvent),
    Clicked(FingerUpEvent),
    None,
}