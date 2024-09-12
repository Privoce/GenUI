use makepad_widgets::DefaultNone;

#[derive(Clone, Debug, DefaultNone)]
pub enum GLoadingEvent {
    Open,
    Close,
    None,
}
