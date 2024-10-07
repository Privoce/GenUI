use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GIconEvent {
    Hover(FingerHoverEvent),
    Clicked(FingerUpEvent),
    None,
}
