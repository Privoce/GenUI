use makepad_widgets::*;

use crate::shader::draw_tool_btn::{DrawGToolButton, GToolButtonType};

live_design! {
    import makepad_draw::shader::std::*;

    GToolButtonBase = {{GToolButton}}{
        
    }
}

#[derive(Live, Widget)]
pub struct GToolButton {
    #[live(GToolButtonType::Default)]
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
        self.draw_tool_button.begin(cx, walk, self.layout);

        self.draw_tool_button.end(cx);
        DrawStep::done()
    }
}

impl LiveHook for GToolButton {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        let color = vec4(1.0, 1.0, 1.0, 1.0);
        self.draw_tool_button.apply_over(cx, live!{
            stroke_color: (color),
        });

        self.draw_tool_button
            .apply_button_type(self.button_type.clone());


        self.draw_tool_button.redraw(cx);
    }
}
