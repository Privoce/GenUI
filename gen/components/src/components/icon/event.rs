use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GIconEvent {
    HoverIn(GIconHoverParam),
    HoverOut(GIconHoverParam),
    Focus(GIconFocusParam),
    Clicked(GIconClickedParam),
    FocusLost(GIconFocusLostParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GIconHoverParam {
    pub e: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct GIconFocusParam { 
    pub e: FingerDownEvent,
}

#[derive(Clone, Debug)]
pub struct GIconClickedParam {
    pub e: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct GIconFocusLostParam {
    pub e: FingerUpEvent,
}