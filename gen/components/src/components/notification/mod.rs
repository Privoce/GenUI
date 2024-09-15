mod register;

pub use register::register;

use makepad_widgets::*;

use super::card::GCard;

live_design!{
    GNotificationBase = {{GNotification}} {}
}

#[derive(Live, Widget, LiveHook)]
pub struct GNotification{
    #[deref]
    #[live]
    pub card: GCard,
}

impl Widget for GNotification{
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        DrawStep::done()
    }
}

