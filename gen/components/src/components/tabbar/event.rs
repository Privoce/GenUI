use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GTabbarItemEvent {
    Hover(GTabbarItemHoverParam),
    Clicked(GTabbarItemClickedParam),
    // Pressed(FingerDownEvent),
    None,
}

#[derive(Clone, Debug)]
pub struct GTabbarItemHoverParam {
    pub value: bool,
    pub e: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct GTabbarItemClickedParam {
    pub value: bool,
    pub e: FingerUpEvent,
}

// -----------------------------------------------------------------------------------

#[derive(Clone, Debug, DefaultNone)]
pub enum GTabbarEvent {
    Changed(GTabbarEventParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GTabbarEventParam {
    pub selected: usize,
    pub e: FingerUpEvent,
}
