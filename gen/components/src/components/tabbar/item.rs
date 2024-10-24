use super::event::*;
use crate::{
    animatie_fn, check_event_scope,
    components::{label::GLabel, svg::GSvg},
    default_handle_animation, event_option, play_animation, ref_area, ref_area_ext,
    ref_event_option, ref_redraw, ref_render, set_event, set_scope_path,
    shader::draw_view::DrawGView,
    themes::Themes,
    utils::{set_cursor, BoolToF32, ThemeColor},
    widget_area,
};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25;
    GTabbarItemBase = {{GTabbarItem}}{
        height: 36.0,
        width: Fill,
        flow: Down,
        background_visible: false,
        align: {
            x: 0.5,
            y: 0.5
        },
        cursor: Hand,
        spacing: 2.0,
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        icon_slot: {draw_svg: {hover: 0.0, focus: 0.0}},
                        text_slot: {draw_text: {hover: 0.0, focus: 0.0}},
                        draw_item: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        icon_slot: {draw_svg: {hover: 1.0, focus: 0.0}},
                        text_slot: {draw_text: {hover: 1.0, focus: 0.0}},
                        draw_item: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        icon_slot: {draw_svg: {focus: 1.0, hover: 0.0}},
                        text_slot: {draw_text: {focus: 1.0, hover: 0.0}},
                        draw_item: {focus: 1.0, hover: 0.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GTabbarItem {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub icon: LiveDependency,
    #[redraw]
    #[live]
    pub draw_item: DrawGView,
    #[redraw]
    #[live]
    #[find]
    pub icon_slot: GSvg,
    #[redraw]
    #[live]
    #[find]
    pub text_slot: GLabel,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub grab_key_focus: bool,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    animator: Animator,
    #[live]
    pub selected: bool,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GTabbarItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.is_visible() {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);

        let _ = self.draw_item.begin(cx, walk, self.layout);
        if self.icon_slot.is_visible() {
            let icon_walk = self.icon_slot.walk(cx);
            let _ = self.icon_slot.draw_walk(cx, scope, icon_walk);
        }
        if self.text_slot.is_visible() {
            let text_walk = self.text_slot.walk(cx);
            let _ = self.text_slot.draw_walk(cx, scope, text_walk);
        }
        let _ = self.draw_item.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        default_handle_animation!(self, cx, event);

        match event.hits(cx, self.area()) {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                if !self.selected {
                    // self.play_animation(cx, id!(hover.focus));
                    self.animate_focus_on(cx); 
                }
            }
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                if !self.selected {
                    self.play_animation(cx, id!(hover.on));
                    self.active_hover_in(cx, e);
                }
            }
            Hit::FingerHoverOut(_) => {
                if !self.selected {
                    self.play_animation(cx, id!(hover.off));
                }
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if !self.selected {
                        self.selected(cx);
                        self.active_clicked(cx, e);
                    }
                }
            }
            _ => (),
        }
    }
}

impl LiveHook for GTabbarItem {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}

impl GTabbarItem {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_item,
        area_icon, icon_slot,
        area_text, text_slot
    }
    event_option! {
        clicked: GTabbarItemEvent::Clicked => GTabbarItemClickedParam,
        hover: GTabbarItemEvent::Hover => GTabbarItemHoverParam
    }
    check_event_scope!();
    pub fn active_hover_in(&mut self, cx: &mut Cx, e: FingerHoverEvent) -> () {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GTabbarItemEvent::Hover(GTabbarItemHoverParam {
                    value: self.selected,
                    e,
                }),
            );
        });
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, e: FingerUpEvent) -> () {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GTabbarItemEvent::Clicked(GTabbarItemClickedParam {
                    value: self.selected,
                    e,
                    id: path.last(),
                }),
            );
        });
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_item.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.icon_slot.draw_svg.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.text_slot.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.use_or("#FFFFFF");
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.use_or("#FFFFFF");
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.use_or("#FFFFFF");
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.use_or("#FFFFFF");
        let shadow_color = self.shadow_color.use_or("#FFFFFF");
        let background_visible = self.background_visible.to_f32();
        let selected = self.selected.to_f32();
        self.draw_item.apply_over(
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
                focus: (selected)
            },
        );

        self.text_slot.draw_text.apply_over(
            cx,
            live! {
                focus: (selected),
            },
        );
        self.icon_slot.draw_svg.apply_over(
            cx,
            live! {
                focus: (selected),
            },
        );
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_item.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.icon_slot.draw_svg.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.text_slot.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_item.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.icon_slot.draw_svg.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.text_slot.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_item.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        self.icon_slot.draw_svg.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        self.text_slot.draw_text.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_item.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        self.icon_slot.draw_svg.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        self.text_slot.draw_text.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
    }
    pub fn selected(&mut self, cx: &mut Cx) -> () {
        self.toggle(cx, true);
    }
    pub fn unselected(&mut self, cx: &mut Cx) -> () {
        self.toggle(cx, false);
    }
    pub fn toggle(&mut self, cx: &mut Cx, selected: bool) -> () {
        self.selected = selected;
        self.render(cx);
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.icon_slot.redraw(cx);
        self.text_slot.redraw(cx);
        self.draw_item.redraw(cx);
    }
}

impl GTabbarItemRef {
    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_area_ext! {
        area_icon,
        area_text
    }
    ref_event_option! {
        clicked => GTabbarItemClickedParam,
        hover => GTabbarItemHoverParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    pub fn selected(&self, cx: &mut Cx) -> () {
        self.borrow_mut().map(|mut x| {
            x.selected(cx);
        });
    }
    pub fn unselected(&self, cx: &mut Cx) -> () {
        self.borrow_mut().map(|mut x| {
            x.unselected(cx);
        });
    }
    pub fn toggle(&self, cx: &mut Cx, selected: bool) -> () {
        self.borrow_mut().map(|mut x| {
            x.toggle(cx, selected);
        });
    }
}

impl GTabbarItemSet {
    set_event! {
        clicked => GTabbarItemClickedParam,
        hover => GTabbarItemHoverParam
    }
}
