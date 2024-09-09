pub mod body;
pub mod cell;
pub mod column;
pub mod footer;
pub mod header;
pub mod row;
mod register;

pub use register::register;
use body::GTableBody;
use header::{GTableHeader, GTableHeaderRef};
use makepad_widgets::*;

use crate::shader::draw_card::DrawCard;

live_design! {
    GTableBase = {{GTable}}{
        // transparent: true,
        // border_width: 0.0,
        // border_radius: 0.0,
        flow: Down,
        padding: 0.0,
        margin: 0.0,
        header_walk: {
            height: Fit,
            width: Fill,
            margin: 0.0,
        },
        body_walk: {
            height: Fit,
            width: Fill,
            margin: 0.0,
        }
    }
}

#[derive(Live, Widget)]
pub struct GTable {
    #[live(true)]
    pub visible: bool,
    #[redraw]
    #[live]
    pub draw_table: DrawCard,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    #[redraw]
    #[find]
    pub header: GTableHeader,
    #[live]
    #[redraw]
    #[find]
    pub body: GTableBody,
    #[live]
    pub header_walk: Walk,
    #[live]
    pub body_walk: Walk,
}

impl Widget for GTable {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_table.begin(cx, walk, self.layout);
        let _ = self.header.draw_walk(cx, scope, self.header_walk);
        let _ = self.body.draw_walk(cx, scope, self.body_walk);

        self.draw_table.end(cx);
        DrawStep::done()
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTable {
    
}