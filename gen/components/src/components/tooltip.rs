use makepad_widgets::*;

use crate::shader::draw_tool_tip::DrawToolTip;

live_design!{
    GToolTipBase = {{GToolTip}}{}
}

#[derive(Live, LiveHook, Widget)]
pub struct GToolTip{
    #[live]
    #[redraw]
    draw_tooltip: DrawToolTip,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GToolTip {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_tooltip.begin(cx, walk, self.layout);

        self.draw_tooltip.end(cx);
        DrawStep::done()   
    }
}