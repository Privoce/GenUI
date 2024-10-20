use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GCheckBoxEvent {
    Clicked(GCheckBoxClickedParam),
    HoverIn(GCheckBoxHoverParam),
    HoverOut(GCheckBoxHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GCheckBoxClickedParam {
    pub value: Option<String>,
    pub selected: bool,
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GCheckBoxHoverParam {
    pub e: Option<FingerHoverEvent>,
}

// -------------------------------------------------------------------------

#[derive(Debug, Clone, DefaultNone)]
pub enum GCheckBoxGroupEvent {
    Changed(GCheckBoxGroupEventParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GCheckBoxGroupEventParam {
    /// The index of the selected checkboxs.
    /// checkbox_group can have multiple selected checkboxs.
    pub selected: Vec<usize>,
    /// The value of the selected checkboxs.
    pub values: Vec<Option<String>>,
    pub e: Option<FingerUpEvent>,
}
