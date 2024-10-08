use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GMenuItemEvent {
    Clicked(GMenuItemClickedParam),
    Hovered(GMenuItemHoveredParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GMenuItemClickedParam {
    pub e: FingerUpEvent,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub struct GMenuItemHoveredParam {
    pub selected: bool,
    pub e: FingerHoverEvent,
}
