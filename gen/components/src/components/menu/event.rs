use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent, LiveId};

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

// --------------------------------------------------------------------------------

#[derive(Debug, Clone, DefaultNone)]
pub enum GSubMenuEvent {
    Changed(GSubMenuChangedParam),
    None,
}

#[derive(Debug, Clone)]
pub struct GSubMenuChangedParam {
    /// The index of the selected item.
    pub selected: Option<Vec<usize>>,
    pub selected_id: LiveId,
    pub e: FingerUpEvent,
}

// --------------------------------------------------------------------------------
#[derive(Debug, Clone, DefaultNone)]
pub enum GMenuEvent {
    Changed(GMenuChangedParam),
    None,
}
#[derive(Debug, Clone)]
pub struct GMenuChangedParam {
    /// The index of the selected item.
    pub selected: Option<Vec<usize>>,
    pub selected_id: LiveId,
    pub e: FingerUpEvent,
}
