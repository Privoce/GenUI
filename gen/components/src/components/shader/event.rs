use makepad_widgets::{ActionDefaultRef, DefaultNone};

#[derive(Clone, Debug, DefaultNone)]
pub enum GShaderEvent {
    Open,
    Close,
    None,
}
