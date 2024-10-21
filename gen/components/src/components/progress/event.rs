use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum GProgressEvent {
    Changed(GProgressChangedParam),
    HoverIn(GProgressHoverParam),
    HoverOut(GProgressHoverParam),
    BeforeChanged(GProgressBeforeChangedParam),
    FocusLost(GProgressFocusLostParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GProgressHoverParam {
    pub e: Option<FingerHoverEvent>,
}

#[derive(Clone, Debug)]
pub struct GProgressBeforeChangedParam {
    pub e: Option<FingerDownEvent>,
    pub value: f64,
    pub step: f64,
    pub range: [f64; 2],
}

#[derive(Clone, Debug)]
pub struct GProgressFocusLostParam {
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GProgressChangedParam {
    pub e: Option<FingerUpEvent>,
    pub value: f64,
    pub step: f64,
    pub range: [f64; 2],
}
