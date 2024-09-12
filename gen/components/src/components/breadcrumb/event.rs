use makepad_widgets::{DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum GBreadCrumbEvent {
    Hover(GBreadCrumbHoverParam),
    Clicked(GBreadCrumbClickedParam),
    ToHome,
    None,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbHoverParam {
    pub index: usize,
    pub item: String,
    pub e: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbClickedParam {
    pub index: usize,
    pub item: String,
    pub e: FingerUpEvent,
}

// ---------------------------------------------------------------------------------------

#[derive(Clone, Debug, DefaultNone)]
pub enum GBreadCrumbItemEvent {
    Hover(GBreadCrumbItemHoverParam),
    Clicked(GBreadCrumbItemClickedParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbItemHoverParam {
    pub item: String,
    pub e: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbItemClickedParam {
    pub item: String,
    pub e: FingerUpEvent,
}
