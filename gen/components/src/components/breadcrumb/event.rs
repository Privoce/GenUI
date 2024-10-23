use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent};

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
    HoverIn(GBreadCrumbItemHoverParam),
    HoverOut(GBreadCrumbItemHoverParam),
    Clicked(GBreadCrumbItemClickedParam),
    Focus(GBreadCrumbItemFocusParam),
    FocusLost(GBreadCrumbItemFocusLostParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbItemHoverParam {
    pub e: Option<FingerHoverEvent>,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbItemFocusParam {
    pub e: Option<FingerDownEvent>,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbItemFocusLostParam {
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbItemClickedParam {
    pub item: String,
    pub e: Option<FingerUpEvent>,
}
