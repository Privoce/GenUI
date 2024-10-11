use makepad_widgets::*;

use crate::shader::draw_view::DrawGView;

use super::cell::GTableCell;

live_design! {
    GTableColumnBase = {{GTableColumn}}{
        flow: Down,
    }
}

#[derive(Live, Widget)]
pub struct GTableColumn {
    #[live]
    pub ptr: String,
    #[live]
    pub text: String,
    #[rust]
    pub data: Vec<String>,
    #[redraw]
    #[live]
    pub draw_table_column: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub children: ComponentMap<LiveId, GTableCell>,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GTableColumn {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_table_column.begin(cx, walk, self.layout);

        self.draw_table_column.end(cx);

        DrawStep::done()
    }
}

impl LiveHook for GTableColumn {}
