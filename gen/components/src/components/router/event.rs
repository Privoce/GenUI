use makepad_widgets::{DefaultNone, HeapLiveIdPath, LiveId};

#[derive(Debug, Clone, DefaultNone)]
pub enum GRouterEvent{
    // NavTo(HeapLiveIdPath),
    NavTo(LiveId),
    NavBack(HeapLiveIdPath),
    None
}