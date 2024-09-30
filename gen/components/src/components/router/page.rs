use makepad_widgets::*;

use crate::components::view::GView;

live_design!{
    GPageBase = {{GPage}}{}
}

#[derive(Live, Widget)]
pub struct GPage{
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for GPage {
    
}

impl Widget for GPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
}