use makepad_widgets::*;

use crate::{
    components::{label::GLabel, svg::GSvg, view::GView},
    event_option,
    shader::draw_view::DrawGView,
    themes::Themes,
    utils::{set_cursor, BoolToF32, ThemeColor},
    widget_area,
};

use super::event::{GMenuItemClickedParam, GMenuItemEvent, GMenuItemHoveredParam};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25;
    GMenuItemBase = {{GMenuItem}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_menu_item: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_menu_item: {hover: [{time: 0.0, value: 1.0}], focus: 0.0}
                    }
                }

                focus = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_menu_item: {focus: [{time: 0.0, value: 1.0}], hover: 0.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GMenuItem {
    #[live(Themes::Dark)]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub visible: bool,
    #[redraw]
    #[live]
    pub draw_menu_item: DrawGView,
    #[find]
    #[live]
    #[redraw]
    pub icon_slot: GSvg,
    #[find]
    #[live]
    #[redraw]
    pub text_slot: GLabel,
    #[find]
    #[live]
    #[redraw]
    pub right: GView,
    #[layout]
    pub layout: Layout,
    #[walk]
    pub walk: Walk,
    #[live(false)]
    pub selected: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GMenuItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.is_visible() {
            return DrawStep::done();
        }

        self.draw_menu_item.begin(cx, walk, self.layout);
        if self.icon_slot.is_visible() {
            let icon_walk = self.icon_slot.walk(cx);
            let _ = self.icon_slot.draw_walk(cx, scope, icon_walk);
        }
        if self.text_slot.is_visible() {
            let text_walk = self.text_slot.walk(cx);
            let _ = self.text_slot.draw_walk(cx, scope, text_walk);
        }
        if self.right.is_visible() {
            let right_walk = self.right.walk(cx);
            let _ = self.right.draw_walk(cx, scope, right_walk);
        }

        // dbg!(&self.selected);
        self.draw_menu_item.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area()) {
            Hit::FingerDown(_) => {
                self.animator_play(cx, id!(hover.focus));
            }
            Hit::FingerHoverIn(_) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerHoverOver(f_h) => {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    GMenuItemEvent::Hovered(GMenuItemHoveredParam {
                        selected: self.selected,
                        e: f_h,
                    }),
                );
            }
            Hit::FingerUp(f_up) => {
                self.selected = !self.selected;
                if self.selected {
                    self.animator_play(cx, id!(hover.focus));
                } else {
                    self.animator_play(cx, id!(hover.off));
                }
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    GMenuItemEvent::Clicked(GMenuItemClickedParam {
                        e: f_up,
                        selected: self.selected,
                    }),
                );
            }
            _ => (),
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GMenuItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.is_visible() {
            return;
        }
        self.render(cx);
        self.redraw(cx);
    }
}

impl GMenuItem {
    widget_area! {
        area, draw_menu_item,
        area_icon, icon_slot,
        area_text, text_slot,
        area_right, right
    }
    event_option! {
        clicked: GMenuItemEvent::Clicked => GMenuItemClickedParam,
        hovered: GMenuItemEvent::Hovered => GMenuItemHoveredParam
    }
    pub fn toggle(&mut self, cx: &mut Cx, selected: bool) {
        self.selected = selected;
        self.draw_menu_item.focus = self.selected.to_f32();
        if self.selected {
            self.animator_play(cx, id!(hover.focus));
        } else {
            self.animator_play(cx, id!(hover.off));
        }
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_menu_item.redraw(cx);
        if self.icon_slot.is_visible() {
            self.icon_slot.redraw(cx);
        }
        if self.text_slot.is_visible() {
            self.text_slot.redraw(cx);
        }
        if self.right.is_visible() {
            self.right.redraw(cx);
        }
    }
    pub fn clear_selected(&mut self, cx: &mut Cx) {
        self.selected = false;
        self.render(cx);
        // self.draw_menu_item.focus = self.selected.to_f32();
        // self.animator_play(cx, id!(hover.off));
        // self.redraw(cx);
    }
    // pub fn set_selected(&mut self, cx: &mut Cx, selected: bool){
    //     self.selected = selected;
    //     self.draw_menu_item.focus = self.selected.to_f32();
    //     // if self.selected {
    //     //     self.animator_play(cx, id!(hover.focus));
    //     // } else {
    //     //     self.animator_play(cx, id!(hover.off));
    //     // }
    //     self.render(cx);
    // }
    pub fn render(&mut self, cx: &mut Cx) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 300);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        self.draw_menu_item.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius),
                focus: (self.selected.to_f32())
            },
        );
    }
}
