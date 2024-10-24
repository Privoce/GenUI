use makepad_widgets::*;

use crate::shader::draw_view::DrawGView;

use super::row::GTableRowRef;

live_design! {
    GVTableBodyBase = {{GVTableBody}}{
        padding: 0.0,
        margin: 0.0,
        flow: Down,
    }
}

#[derive(Live, Widget)]
pub struct GVTableBody {
    #[live(true)]
    pub visible: bool,
    #[redraw]
    #[live]
    pub draw_table_body: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub children: ComponentMap<LiveId, GTableRowRef>,
}

impl Widget for GVTableBody {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_table_body.begin(cx, walk, self.layout);
        for (_index, (_id, child)) in self.children.iter().enumerate() {
            if child.is_visible() {
                let child_walk = child.walk(cx);
                let _ = child.draw_walk(cx, scope, child_walk);
            }
        }
        self.draw_table_body.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for (_index, (_id, child)) in self.children.iter().enumerate() {
            if child.is_visible() {
                child.handle_event(cx, event, scope);
            }
        }
    }
}

impl LiveHook for GVTableBody {}

impl GVTableBody {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_table_body.redraw(cx);
        for (_, row) in self.children.iter() {
            row.borrow_mut().map(|mut row| {
                row.redraw(cx);
            });
        }
    }
}
