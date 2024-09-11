use makepad_widgets::*;

use crate::components::card::GCard;

live_design! {
    GTableHeaderBase = {{GTableHeader}}{
        background_visible: true,
        border_width: 0.0,
        border_radius: 0.0,
        padding: 0.0,
        margin: 0.0,
    }
}

#[derive(Live, Widget)]
pub struct GTableHeader {
    #[live]
    #[deref]
    pub deref_widget: GCard,
}

impl Widget for GTableHeader {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref_widget.handle_event(cx, event, scope)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTableHeader {}

impl GTableHeader {
    pub fn default_walk(&self) -> Walk {
        Walk {
            height: Size::Fit,
            width: Size::Fill,
            margin: Margin::default(),
            abs_pos: None,
        }
    }
}
