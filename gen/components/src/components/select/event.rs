use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GSelectItemEvent {
    Clicked(GSelectItemClickedParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GSelectItemClickedParam {
    pub selected: bool,
    pub text: String,
    pub value: String,
    pub e: FingerUpEvent,
}
// -------------------------------------------------------------------------
#[derive(Debug, Clone, DefaultNone)]
pub enum GSelectOptionsEvent {
    Changed(GSelectOptionsChangedParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GSelectOptionsChangedParam {
    pub selected: bool,
    pub text: String,
    pub value: String,
    pub selected_id: usize,
    pub e: FingerUpEvent,
}

// -------------------------------------------------------------------------
#[derive(Debug, Clone, DefaultNone)]
pub enum GSelectEvent {
    Changed(GSelectOptionsChangedParam),
    None,
}
