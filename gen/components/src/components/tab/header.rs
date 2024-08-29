use makepad_widgets::*;

use super::button::{GTabButtonEvent, GTabButtonRef, GTabButtonWidgetRefExt};

live_design! {
    GTabHeaderBase = {{GTabHeader}}{}
}

#[derive(Live, Widget)]
pub struct GTabHeader {
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
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[rust]
    pub view_area: Area,
}

impl Widget for GTabHeader {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.scroll_bars.begin(
            cx,
            walk.with_add_padding(Padding {
                left: 4.0,
                top: 2.0,
                right: 4.0,
                bottom: 2.0,
            }),
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
        let t_map = self.children.clone();
        let mut target_selected = None;
        let mut target_remove = None;
        for (id, tab_btn_ref) in t_map.iter() {
            tab_btn_ref
                .as_origin_mut()
                .unwrap()
                .handle_event_actions(cx, event, &mut |_cx, e| match e {
                    GTabButtonEvent::Selected => {
                        target_selected = Some(id);
                    }
                    GTabButtonEvent::Close => {
                        target_remove = Some(id.0);
                    }
                    _ => {}
                });
        }
        // remove the tab ---------------------------------------------------------------------------------------------------
        if let Some(remove) = target_remove {
            self.items.remove(remove as usize);
            self.redraw(cx);
        }
        // render select tab ------------------------------------------------------------------------------------------------
        if let Some(selected) = target_selected {
            for (id, tab_btn_ref) in self.children.iter_mut() {
                if id == selected {
                    tab_btn_ref.as_origin_mut().unwrap().selected = true;
                } else {
                    tab_btn_ref.as_origin_mut().unwrap().selected = false;
                }
                tab_btn_ref.as_origin_mut().unwrap().render(cx);
            }
        }
    }
}

impl LiveHook for GTabHeader {

}

impl GTabHeader {
    // pub fn area(&self) -> Area{
    //     self.draw_tab_header.area
    // }
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
    }
}

