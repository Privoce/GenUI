pub mod header;
pub mod body;
pub mod button;

use makepad_widgets::*;

use crate::shader::draw_card::DrawCard;

live_design!{
    GTabBase = {{GTab}}{}
}

#[derive(Live, Widget)]
pub struct  GTab{
    #[redraw]
    #[live]
    pub draw_tab: DrawCard,
    // #[live]
    // pub header: TabHeader
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GTab{
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        DrawStep::done()
    }
}

impl LiveHook for GTab {
    
}