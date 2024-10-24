use makepad_widgets::*;

use crate::{
    shader::draw_view::DrawGView,
    themes::Themes,
    utils::{BoolToF32, ThemeColor},
};

use super::{item::GSelectItem, GSelectItemEvent, GSelectOptionsChangedParam, GSelectOptionsEvent};

live_design! {
    GSelectOptionsBase = {{GSelectOptions}}{

    }
}

#[derive(Live, LiveRegister)]
pub struct GSelectOptions {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.8)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live]
    pub draw_options: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub children: ComponentMap<LiveId, GSelectItem>,
    #[live]
    pub draw_list: DrawList2d,
    #[live]
    pub item: Option<LivePtr>,
    // scroll ---------------------
    #[live]
    pub scroll_bars: Option<LivePtr>,
    #[rust]
    pub scroll_bars_obj: Option<Box<ScrollBars>>,
}

impl LiveHook for GSelectOptions {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.use_or("#ffffff");
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.use_or("#ffffff");
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.use_or("#ffffff");
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        self.draw_options.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        self.draw_options.redraw(cx);
    }
}

impl GSelectOptions {
    pub fn area(&self) -> Area {
        self.draw_options.area()
    }

    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_options.area().clipped_rect(cx).contains(pos)
    }
    /// ## Begin to draw popup
    /// this method is used to begin drawing the popup
    pub fn begin(&mut self, cx: &mut Cx2d, theme: Themes) {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.begin_nav_area(cx);
            scroll_bars.get_scroll_pos()
        } else {
            self.layout.scroll
        };
        self.draw_options
            .begin(cx, self.walk, self.layout.with_scroll(scroll));
        self.theme = theme;
    }
    pub fn end_container(&mut self, cx: &mut Cx2d) {
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.draw_scroll_bars(cx);
        }
        // before end do apply
        self.draw_options.end(cx);
        let area = self.area();

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.set_area(area);
            scroll_bars.end_nav_area(cx);
        }
    }
    /// ## End to draw popup
    pub fn end(&mut self, cx: &mut Cx2d, _scope: &mut Scope, shift_area: Area, shift: DVec2) {
        // before this do end container!
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_list.redraw(cx);
        // self.draw_options.redraw(cx);
    }
    pub fn draw_option(&mut self, cx: &mut Cx2d, item_id: LiveId, text: &str, value: &str) {
        let target = self
            .children
            .get_or_insert(cx, item_id, |cx| GSelectItem::new_from_ptr(cx, self.item));
        target.draw_item(cx, text, value, self.theme);
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        dispatch_action: &mut dyn FnMut(&mut Cx, GSelectOptionsEvent),
    ) {
        let mut actions = Vec::new();
        for (item_id, node) in self.children.iter_mut() {
            node.handle_event_with(cx, event, sweep_area, &mut |_, e| {
                actions.push((*item_id, e))
            });
        }

        for (node_id, action) in actions {
            match action {
                GSelectItemEvent::Clicked(param) => {
                    // if is item clicked, do options event change
                    if param.selected {
                        for (_index, (id, item)) in self.children.iter_mut().enumerate() {
                            if id.0 != node_id.0 {
                                item.selected = false;
                                item.animator_play(cx, id!(select.off));
                            } else {
                                item.selected = true;
                                item.animator_play(cx, id!(select.on));
                            }
                        }
                    }
                    dispatch_action(
                        cx,
                        GSelectOptionsEvent::Changed(GSelectOptionsChangedParam {
                            selected: param.selected,
                            text: param.text,
                            value: param.value,
                            selected_id: node_id.0 as usize,
                            e: param.e,
                        }),
                    );
                }
                _ => (),
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, &mut Scope::empty(), &mut Vec::new());
        }
    }
}
