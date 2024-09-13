use makepad_widgets::*;

#[derive(Clone, Copy)]
pub enum DrawCollapseState {
    DrawHeader,
    DrawBody,
}

#[derive(Live, LiveHook, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum IconPosition {
    #[pick]
    Left,
    Right,
}
