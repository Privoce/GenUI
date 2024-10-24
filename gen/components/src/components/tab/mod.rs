pub mod body;
pub mod button;
pub mod header;
pub mod pane;

use header::GTabHeader;
use makepad_widgets::*;
use pane::GTabPane;

use crate::{shader::draw_view::DrawGView, themes::Themes};

live_design! {
    GTabBase = {{GTab}}{}
}

#[derive(Live, Widget)]
pub struct GTab {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[rust]
    pub items: Vec<String>,
    #[redraw]
    #[live]
    pub draw_tab: DrawGView,
    #[live]
    pub header: GTabHeader,
    #[live]
    pub body: GTabPane,
    #[rust]
    pub selected: usize,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GTab {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_tab.begin(cx, walk, self.layout);

        self.items = self.body.header_items();

        self.header.selected = self.selected;
        self.body.selected = self.selected;
        self.header.set_items(self.items.clone());

        let _ = self.header.draw_walk(cx, scope, self.header.walk);

        let _ = self.body.draw_walk(cx, scope, self.body.walk);

        self.draw_tab.end(cx);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = self
            .header
            .handle_event_actions(cx, event, scope, &mut |e| match e {
                header::GTabHeaderEvent::Selected(index) => {
                    // select_target.replace(index);
                    self.selected = index;
                }
                header::GTabHeaderEvent::Close(index) => {
                    self.body.remove(index);
                }
                _ => {}
            });

        self.body.handle_event(cx, event, scope);
    }
}

impl LiveHook for GTab {
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        // let bg_color = get_color(self.theme, self.background_color, 500);
        // self.draw_tab.apply_over(cx, live!{
        //     background_color: (bg_color),
        // })
    }
}

impl GTab {
    pub fn area(&self) -> Area {
        self.draw_tab.area
    }
    pub fn area_header(&self) -> Area {
        self.header.area()
    }
    pub fn area_body(&self) -> Area {
        self.body.area()
    }
}
