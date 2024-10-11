use makepad_widgets::*;

use crate::{
    shader::draw_view::DrawGView,
    themes::{hex_to_vec4, Themes},
};

use super::button::{GTabButtonEvent, GTabButtonRef, GTabButtonWidgetRefExt};

live_design! {
    import makepad_draw::shader::std::*;

    GTabHeaderBase = {{GTabHeader}}{
        draw_header: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.rect(
                    self.border_width,
                    self.rect_size.y - self.border_width - 3.0,
                    self.rect_size.x - self.border_width * 2.0,
                    3.0
                );
                sdf.fill(self.get_color())
                sdf.box(
                    self.border_width,
                     self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    max(1.0, self.border_radius)
                )
                if self.background_visible == 0.0 {
                   sdf.fill_keep(self.get_color())
                }

                sdf.stroke(self.get_border_color(), self.border_width)
                return sdf.result;
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GTabHeader {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    draw_drag: DrawColor,
    #[redraw]
    #[live]
    draw_header: DrawGView,
    #[live]
    pub selected: usize,
    #[live]
    pub is_dragged: bool,
    #[live]
    pub item: Option<LivePtr>,
    #[live]
    pub items: Vec<String>,
    #[rust]
    pub children: ComponentMap<LiveId, GTabButtonRef>,
    #[live]
    pub scroll_bars: ScrollBars,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[rust]
    pub view_area: Area,
    #[live(true)]
    pub event_key: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GTabHeaderEvent {
    Selected(usize),
    Close(usize),
    None,
}

impl Widget for GTabHeader {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_header.begin(cx, walk, self.layout);
        self.scroll_bars.begin(
            cx,
            walk.with_add_padding(Padding {
                left: 4.0,
                top: 2.0,
                right: 4.0,
                bottom: 0.0,
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
        self.render_selected(cx);

        self.scroll_bars.end(cx);
        self.draw_header.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.scroll_bars.handle_event(cx, event, scope).len() > 0 {
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
            self.selected = selected.0 as usize;
            self.render_selected(cx);
        }
    }
}

impl LiveHook for GTabHeader {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        let bg_color = hex_to_vec4("#EAECF0");
        self.draw_header.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: 1.0
            },
        )
    }
}

impl GTabHeader {
    pub fn handle_event_actions(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        f: &mut dyn FnMut(GTabHeaderEvent),
    ) {
        if self.scroll_bars.handle_event(cx, event, scope).len() > 0 {
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
                        self.selected = id.0 as usize;
                        f(GTabHeaderEvent::Selected(id.0 as usize));
                    }
                    GTabButtonEvent::Close => {
                        target_remove = Some(id.0);
                        f(GTabHeaderEvent::Close(id.0 as usize));
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
            self.selected = selected.0 as usize;
            self.render_selected(cx);
        }
    }
    pub fn area(&self) -> Area {
        self.scroll_bars.area()
    }
    pub fn render_selected(&mut self, cx: &mut Cx) {
        for (id, tab_btn_ref) in self.children.iter_mut() {
            if id.0 == self.selected as u64 {
                tab_btn_ref.as_origin_mut().unwrap().selected = true;
            } else {
                tab_btn_ref.as_origin_mut().unwrap().selected = false;
            }
            tab_btn_ref.as_origin_mut().unwrap().render(cx);
        }
    }
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
    }
}
