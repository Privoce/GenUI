use makepad_widgets::{DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GTabbarItemEvent {
    Hover(FingerHoverEvent),
    Clicked(FingerUpEvent),
    Pressed(FingerDownEvent),
    None,
}
