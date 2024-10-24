mod register;

pub use register::register;

use makepad_widgets::*;

use super::view::GView;

live_design!{
    GNotificationBase = {{GNotification}} {}
}

#[derive(Live, Widget, LiveHook)]
pub struct GNotification{
    #[deref]
    #[live]
    pub view: GView,
}

impl Widget for GNotification{
    fn draw_walk(&mut self, _cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        DrawStep::done()
    }
}

