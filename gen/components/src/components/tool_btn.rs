use makepad_widgets::*;

use crate::shader::draw_tool_btn::{DrawGToolButton, GToolButtonType};

live_design! {
    GToolButtonBase = {{GToolButton}}{}
}

#[derive(Live, Widget)]
pub struct GToolButton {
    #[live]
    pub button_type: GToolButtonType,
    #[redraw]
    #[live]
    pub draw_tool_button: DrawGToolButton,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GToolButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_tool_button.draw_walk(cx, walk);
        DrawStep::done()
    }
}

impl LiveHook for GToolButton {
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.draw_tool_button
            .apply_button_type(self.button_type.clone());
    }
}
