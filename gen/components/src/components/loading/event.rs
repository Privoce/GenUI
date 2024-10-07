use makepad_widgets::{ActionDefaultRef, DefaultNone};

#[derive(Clone, Debug, DefaultNone)]
pub enum GLoadingEvent {
    Open,
    Close,
    None,
}
