use makepad_widgets::*;

use crate::shader::draw_card::DrawCard;

use super::cell::GTableCell;

live_design!{
    GTableColumnBase = {{GTableColumn}}{
        flow: Down,
    }
}

#[derive(Live, Widget)]
pub struct GTableColumn{
    #[redraw]
    #[live]
    pub draw_table_column: DrawCard,
    #[walk]
    pub walk : Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub children: ComponentMap<LiveId, GTableCell>,
}

impl Widget for GTableColumn{
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_table_column.begin(cx, walk, self.layout);

       
        self.draw_table_column.end(cx);

        DrawStep::done()
    }
}

impl LiveHook for GTableColumn {
    
}