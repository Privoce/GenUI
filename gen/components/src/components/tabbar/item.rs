use super::event::*;
use crate::{
    components::{label::GLabel, svg::GSvg}, event_option, ref_event_option, shader::draw_view::DrawGView, themes::Themes, utils::{set_cursor, BoolToF32, ThemeColor}, widget_area
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
    pub pressed_color: Option<Vec4>,
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
    pub animation_open: bool,
    #[animator]
    animator: Animator,
    #[live]
    pub selected: bool,
}

impl Widget for GTabbarItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.draw_item.begin(cx, walk, self.layout);
        let icon_walk = self.icon_slot.walk(cx);
        let _ = self.icon_slot.draw_walk(cx, scope, icon_walk);
        let text_walk = self.text_slot.walk(cx);
        // let font = get_font_family(&self.font_family, cx);
        // self.text_slot.draw_text.text_style.font = font;
        let _ = self.text_slot.draw_walk(cx, scope, text_walk);
        let _ = self.draw_item.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animation_open {
            self.animator_handle_event(cx, event);
        }
        match event.hits(cx, self.area()) {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                if !self.selected {
                    self.animation_pressed(cx);
                    // cx.widget_action(uid, &scope.path, GTabbarItemEvent::Pressed(f_down.clone()));
                }
            }
            Hit::FingerHoverIn(h) => {
                let _ = set_cursor(cx, self.cursor.as_ref());

                if !self.selected {
                    self.animation_hover_on(cx);
                    cx.widget_action(uid, &scope.path, GTabbarItemEvent::Hover(GTabbarItemHoverParam{
                        value: self.selected,
                        e: h.clone(),
                    }));
                }
            }
            Hit::FingerHoverOut(_) => {
                if !self.selected {
                    self.animation_hover_off(cx);
                }
            }
            Hit::FingerUp(f_up) => {
                if !self.selected {
                    self.animation_selected(cx);
                    cx.widget_action(uid, &scope.path, GTabbarItemEvent::Clicked(GTabbarItemClickedParam{
                        value: self.selected,
                        e: f_up.clone(),
                    }));
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
    widget_area! {
        area, draw_item,
        area_icon, icon_slot,
        area_text, text_slot
    }
    event_option! {
        clicked: GTabbarItemEvent::Clicked => GTabbarItemClickedParam,
        hover: GTabbarItemEvent::Hover => GTabbarItemHoverParam
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.use_or("#FFFFFF");
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.use_or("#FFFFFF");
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.use_or("#FFFFFF");
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
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius),
                pressed: (selected)
            },
        );

        self.text_slot.draw_text.pressed = selected;
        self.icon_slot.draw_svg.hover = selected;
    }
    pub fn animation_hover_on(&mut self, cx: &mut Cx) -> () {
        self.icon_slot.animate_hover_on(cx);
        self.text_slot.animate_hover_on(cx);
    }
    pub fn animation_hover_off(&mut self, cx: &mut Cx) -> () {
        self.icon_slot.animate_hover_off(cx);
        self.text_slot.animate_hover_off(cx);
    }
    pub fn animation_pressed(&mut self, cx: &mut Cx) -> () {
        self.icon_slot.animate_hover_on(cx);
        self.text_slot.animate_hover_pressed(cx);
    }
    pub fn animation_selected(&mut self, cx: &mut Cx) -> () {
        self.selected = true;
        self.render(cx);
    }
    pub fn animation_unselected(&mut self, cx: &mut Cx) -> () {
        self.selected = false;
        self.render(cx);
    }
    pub fn toggle(&mut self, cx: &mut Cx, selected: bool) ->(){
        self.selected = selected;
        self.render(cx);
    }
}

impl GTabbarItemRef {
    ref_event_option! {
        clicked => GTabbarItemClickedParam,
        hover => GTabbarItemHoverParam
    }
}