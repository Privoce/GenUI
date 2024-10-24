use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GBreadCrumbEvent {
    HoverIn(GBreadCrumbHoverParam),
    HoverOut(GBreadCrumbHoverParam),
    Focus(GBreadCrumbFocusParam),
    FocusLost(GBreadCrumbFocusLostParam),
    Changed(GBreadCrumbChangedParam),
    Home(GBreadCrumbHomeParam),
    None,
}

#[derive(Clone, Debug)]
pub enum GBreadCrumbItemKind {
    Item {
        text: String,
        index: usize,
    },
    /// Home Icon
    Icon,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbHoverParam {
    pub kind: GBreadCrumbItemKind,
    pub e: Option<FingerHoverEvent>,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbFocusParam {
    pub kind: GBreadCrumbItemKind,
    pub e: Option<FingerDownEvent>,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbFocusLostParam {
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbHomeParam {
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbChangedParam {
    pub index: usize,
    pub text: String,
    pub e: Option<FingerUpEvent>,
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
    pub text: String,
    pub e: Option<FingerUpEvent>,
}
