// use makepad_widgets::*;

// live_design! {
//     GTabHeaderBase = {{GTabHeader}}{}
// }

// #[derive(Live, Widget)]
// pub struct GTabHeader {

//     #[rust] selected_tab: Option<usize>,
//     #[rust] tab_order: Vec<LiveId>,
//     #[live] draw_drag: DrawColor,
//     #[live] pub is_dragged: bool,
//     #[rust] tabs: ComponentMap<LiveId, (Tab, LiveId)>,
//     #[redraw]
//     #[live]
//     pub scroll_bars: ScrollBars,
//     #[layout]
//     pub layout: Layout,
//     #[walk]
//     pub walk: Walk,
//     #[rust]
//     draw_state: DrawStateWrap<()>,
// }

// impl Widget for GTabHeader {
//     fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
//         if self.draw_state.begin(cx, ()) {
//             return DrawStep::make_step();
//         }
//         if let Some(()) = self.draw_state.get() {
//             self.draw_state.end();
//         }
//         DrawStep::done()
//     }
// }

// impl LiveHook for GTabHeader {}

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