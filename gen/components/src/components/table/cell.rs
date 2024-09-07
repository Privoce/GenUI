use makepad_widgets::*;

use crate::{components::card::Card, themes::get_color};

live_design! {
    GTableCellBase = {{GTableCell}}{
        transparent: true,
        border_radius: 0.0,
        border_width: 0.0
    }
}

#[derive(Live, Widget)]
pub struct GTableCell {
    #[deref]
    pub deref_widget: Card,
}

impl LiveHook for GTableCell {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        let border_radius = self.border_radius;
        let border_width = self.border_width;
        self.draw_card.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (border_width),
                border_radius: (border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
            },
        );
        self.draw_card.redraw(cx);
    }
}

impl Widget for GTableCell {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref_widget.handle_event(cx, event, scope)
    }
}
