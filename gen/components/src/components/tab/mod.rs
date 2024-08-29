pub mod body;
pub mod button;
pub mod header;
pub mod pane;

use body::{GTabBody, GTabBodyRef};
use header::GTabHeader;
use makepad_widgets::*;
use pane::GTabPane;

use crate::{
    shader::draw_card::DrawCard,
    themes::{get_color, Themes},
};

use super::card::Card;

live_design! {
    GTabBase = {{GTab}}{}
}

#[derive(Live, Widget)]
pub struct GTab {
    
    #[redraw]
    #[live]
    pub draw_tab: DrawCard,
    #[live]
    pub header: GTabHeader,
    #[live]
    pub body: GTabPane,
    #[live(0_usize)]
    pub selected: usize,
    // #[rust]
    // pub bodys: ComponentMap<LiveId, GTabBodyRef>,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}

impl Widget for GTab {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_tab.begin(cx, walk, self.layout);

        let mut header_items = vec![];
        let _ = self.body.children.iter().for_each(|(_id, body_ref)|{
            header_items.push(body_ref.text())
        });
        // dbg!(&header_items);
        self.header.set_items(header_items);
       
        let _ = self.header.draw_walk(cx, scope, self.header.walk);
        
        let _ = self.body.draw_walk(cx, scope, self.body.walk);

        self.draw_tab.end(cx);

        DrawStep::done()
    }
}

impl LiveHook for GTab {

}
