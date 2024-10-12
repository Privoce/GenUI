use makepad_widgets::*;

use crate::{components::view::GView, widget_area};

live_design! {
    GTableCellBase = {{GTableCell}}{
        background_visible: true,
        border_radius: 0.0,
        border_width: 0.0,
        background_color: #F9FAFB,
        hover_color: #F9FAFB,
        focus_color: #F9FAFB,
        border_color: #EAECF0,
        shadow_color: #FFFFFF00
    }
}

#[derive(Live, Widget)]
pub struct GTableCell {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for GTableCell {}

impl Widget for GTableCell {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl GTableCell {
    widget_area! {
        area, draw_view
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
}
