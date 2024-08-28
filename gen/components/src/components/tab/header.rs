use makepad_widgets::*;

use crate::shader::draw_card::DrawCard;

use super::button::{GTabButton, GTabButtonRef, GTabButtonWidgetRefExt};

live_design! {
    GTabHeaderBase = {{GTabHeader}}{}
}

#[derive(Live, Widget)]
pub struct GTabHeader {
    #[rust]
    selected_tab: Option<usize>,
    #[rust]
    tab_order: Vec<LiveId>,
    #[live]
    draw_drag: DrawColor,
    #[live]
    pub is_dragged: bool,
    #[live]
    pub item: Option<LivePtr>,
    #[live]
    pub items: Vec<String>,
    #[rust]
    children: ComponentMap<LiveId, GTabButtonRef>,
    #[redraw]
    #[live]
    pub scroll_bars: ScrollBars,
    // pub draw_tab_header: DrawCard,

    // #[live]
    // pub scroll_bars: Option<LivePtr>,
    // #[rust]
    // pub scroll_bars_obj: Option<Box<ScrollBars>>,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[rust]
    pub view_area: Area,
}

impl Widget for GTabHeader {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
        //     scroll_bars.begin_nav_area(cx);
        //     scroll_bars.get_scroll_pos()
        // } else {
        //     self.layout.scroll
        // };
        // self.draw_tab_header.begin(cx, walk, self.layout);

        // // render the tab_buttons depend on
        // for (index, data) in self.items.iter().enumerate() {
        //     let target = self.children.get_or_insert(cx, LiveId(index as u64), |cx| {
        //         WidgetRef::new_from_ptr(cx, self.item).as_gtab_button()
        //     });

        //     target.set_text(data);
        //     target.draw_all(cx, &mut Scope::empty());
        // }
        // let area = self.area();
        // if let Some(scroll_bars) = &mut self.scroll_bars_obj {
        //     scroll_bars.draw_scroll_bars(cx);
        // }
        // self.draw_tab_header.end(cx);
        // if let Some(scroll_bars) = &mut self.scroll_bars_obj {
        //     scroll_bars.set_area(area);
        //     scroll_bars.end_nav_area(cx);
        // }

        self.scroll_bars.begin(
            cx,
            walk,
            Layout::flow_right(),
        );

        // render the tab_buttons depend on
        for (index, data) in self.items.iter().enumerate() {
            let target = self.children.get_or_insert(cx, LiveId(index as u64), |cx| {
                WidgetRef::new_from_ptr(cx, self.item).as_gtab_button()
            });

            target.set_text(data);
            target.draw_all(cx, &mut Scope::empty());
        }
        self.scroll_bars.end(cx);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if self.scroll_bars.handle_event(cx, event).len() > 0 {
            self.view_area.redraw(cx);
        };
    }
}

impl LiveHook for GTabHeader {
    // fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
    //     self.draw_tab_header.apply_over(cx, live!{
    //         background_color: (vec4(1.0, 1.0, 1.0, 1.0)),
    //     });
    //     self.draw_tab_header.redraw(cx);
    // }
}

impl GTabHeader {
    // pub fn area(&self) -> Area{
    //     self.draw_tab_header.area
    // }
}

// impl GTabHeader {
//     pub fn begin(&mut self, cx: &mut Cx2d, selected_tab: Option<usize>, walk:Walk) {
//         self.selected_tab = selected_tab;
//         //if selected_tab.is_some(){
//         //    self.selected_tab_id = None
//         // }
//         self.scroll_bars.begin(cx, walk, Layout::flow_right());
//         self.tab_order.clear();
//     }

//     pub fn end(&mut self, cx: &mut Cx2d) {
//         if self.is_dragged {
//             self.draw_drag.draw_walk(
//                 cx,
//                 Walk {
//                     width: Size::Fill,
//                     height: Size::Fill,
//                     ..Walk::default()
//                 },
//             );
//         }
//         self.tabs.retain_visible();
//         self.draw_fill.draw_walk(cx, Walk::size(Size::Fill, Size::Fill));
//         self.scroll_bars.end(cx);
//     }
// }
