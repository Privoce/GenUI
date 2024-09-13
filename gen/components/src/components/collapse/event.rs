use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GCollapseEvent {
    Hover(FingerHoverEvent),
    Opened(FingerUpEvent),
    Closed(FingerUpEvent),
    None,
}
