use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, HeapLiveIdPath};

/// # UnifiedEvent
/// A unified event that should be transmitted throughout the entire system,
/// currently used to supplement event bubble transmission
#[derive(DefaultNone, Clone, Debug)]
pub enum UnifiedEvent {
    HoverIn(HoverInParam),
    HoverOut(HoverOutParam),
    None,
}

/// # HoverInParam
#[derive(Debug, Clone)]
pub struct HoverInParam {
    pub e: FingerHoverEvent,
    pub path: HeapLiveIdPath,
}

#[derive(Debug, Clone)]
pub struct HoverOutParam {
    pub path: HeapLiveIdPath,
}

/// # FocusEvent Type
/// The type of focus event
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FocusType {
    #[default]
    Press,
    // Selecting,
    // Expanding,
    Dragging,
}
