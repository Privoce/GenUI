use makepad_widgets::{ActionDefaultRef, DefaultNone, LiveId};

#[derive(Debug, Clone, DefaultNone)]
pub enum GRouterEvent {
    NavTo(LiveId),
    NavBack(LiveId),
    None,
}
