use makepad_widgets::*;

use super::{draw_view::DrawGView, manual::Position};

live_design! {
    import makepad_draw::shader::std::*;

    DrawGPopup = {{DrawGPopup}}{}
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGPopup {
    #[deref]
    pub deref_draw: DrawGView,
    #[live]
    pub position: Position,
    #[live(0.6)]
    pub opactiy: f32,
    #[live(0.4)]
    pub proportion: f32,
    /// The angle offset of the popup, usually used in tooltips to get the angle center when painting
    #[live]
    pub angle_offset: f32,
}
