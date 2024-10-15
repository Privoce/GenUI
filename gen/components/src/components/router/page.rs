use makepad_widgets::*;

use crate::{
    components::{icon::GIconWidgetExt, view::GView},
    utils::LiveIdExp,
};

use super::{event::GRouterEvent, GRouter};

live_design! {
    GPageBase = {{GPage}}{}
}

#[derive(Live, Widget)]
pub struct GPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for GPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for GPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        for action in &actions {
            if let Some(action) = action.as_widget_action() {
                match action.cast::<GRouterEvent>() {
                    GRouterEvent::NavTo(path) => {
                        GRouter::nav_to_path(cx, self.widget_uid(), scope, path.as_slice());
                    }
                    GRouterEvent::NavBack(_) => {
                        GRouter::nav_back(cx, self.widget_uid(), scope);
                    }
                    GRouterEvent::None => (),
                }
            }
        }

        if self.gicon(id!(back_wrap.back)).clicked(&actions).is_some() {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                GRouterEvent::NavBack(scope.path.clone().last()),
            );
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl GPage {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) {
        self.deref_widget.render(cx);
    }
}


impl GPageRef {
    pub fn set_visible_and_redraw(&mut self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }
}