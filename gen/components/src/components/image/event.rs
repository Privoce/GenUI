use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GImageEvent {
    Hover(FingerHoverEvent),
    Clicked(FingerUpEvent),
    None,
}
