use makepad_widgets::DefaultNone;

#[derive(Clone, Debug, DefaultNone)]
pub enum GShaderEvent {
    Open,
    Close,
    None,
}
