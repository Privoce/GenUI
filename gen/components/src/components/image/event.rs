use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GImageEvent {
    HoverIn(GImageHoverParam),
    HoverOut(GImageHoverParam),
    Clicked(GImageClickedParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GImageHoverParam {
    pub e: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct GImageClickedParam {
    pub e: FingerUpEvent,
}
