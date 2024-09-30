use makepad_widgets::*;

use crate::components::{icon::GIconWidgetExt, view::GView};

use super::event::GRouterEvent;

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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        if self.gicon(id!(back_wrap.back)).clicked(&actions).is_some(){
            dbg!("back");
            cx.widget_action(self.widget_uid(), &scope.path, GRouterEvent::NavBack(scope.path.clone()));
        }
    }
}