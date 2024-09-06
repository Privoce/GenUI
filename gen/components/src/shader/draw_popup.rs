use makepad_widgets::*;

use super::{draw_card::DrawCard, manual::Position};

live_design!{
    import makepad_draw::shader::std::*;

    DrawGPopup = {{DrawGPopup}}{}
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawGPopup{
    #[deref]
    pub deref_draw: DrawCard,
    #[live]
    pub position: Position
}