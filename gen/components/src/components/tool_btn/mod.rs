mod event;
mod register;
pub mod types;

pub use event::*;
pub use register::register;

use makepad_widgets::*;
use types::{GOsType, GToolButtonType};

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down, default_hit_hover_in, default_hit_hover_out, event_option, play_animation, ref_area, ref_area_ext, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path, shader::{
        draw_view::DrawGView,
        icon_lib::{base::DrawGIconBase, types::base::Base},
    }, utils::{set_cursor, ThemeColor}, widget_area
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25;
    GToolButtonBase = {{GToolButton}} {
        height: 32.0,
        width: 46.0,
        color: #768390,
        stroke_hover_color: #768390,
        stroke_focus_color: #768390,
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_tool_btn: {hover: 0.0, focus: 0.0},
                        draw_icon: {hover: 0.0, focus: 0.0},
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_tool_btn: {hover: 1.0, focus: 0.0},
                        draw_icon: {hover: 1.0, focus: 0.0},
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_tool_btn: {hover: 0.0, focus: 1.0},
                        draw_icon: {hover: 0.0, focus: 1.0},
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GToolButton {
    // inner icon ------------------------
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    // tool button do not need add control style
    #[redraw]
    #[live]
    pub draw_tool_btn: DrawGView,
    #[redraw]
    #[live]
    pub draw_icon: DrawGIconBase,
    // when os is mac or linux, height and width always = 16.0
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_key: bool,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live]
    pub icon_type: GToolButtonType,
    #[live]
    pub os_type: Option<GOsType>,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GToolButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        let mut icon_walk = Walk::default();
        match self.os_type.as_ref().unwrap() {
            GOsType::Windows | GOsType::Other => {
                icon_walk.width = Size::Fixed(16.0);
                icon_walk.height = Size::Fixed(16.0);
            }
            GOsType::Mac | GOsType::Linux => {
                walk.width = Size::Fixed(16.0);
                walk.height = Size::Fixed(16.0);
                icon_walk.width = Size::Fixed(12.0);
                icon_walk.height = Size::Fixed(12.0);
            }
        }
        let _ = self.draw_tool_btn.begin(cx, walk, self.layout);
        let _ = self.draw_icon.draw_walk(cx, icon_walk);
        let _ = self.draw_tool_btn.end(cx);
        DrawStep::done()
    }

    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.is_visible(){
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible(){
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GToolButton {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }

        self.render(cx);
    }
}

impl GToolButton {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_tool_btn,
        area_icon, draw_icon
    }
    event_option! {
        hover_in: GToolButtonEvent::HoverIn => GToolButtonHoverParam,
        hover_out: GToolButtonEvent::HoverOut => GToolButtonHoverParam,
        focus: GToolButtonEvent::Focus => GToolButtonFocusParam,
        focus_lost: GToolButtonEvent::FocusLost => GToolButtonFocusLostParam,
        clicked: GToolButtonEvent::Clicked => GToolButtonClickedParam
    }
    active_event! {
        active_hover_in: GToolButtonEvent::HoverIn |e: Option<FingerHoverEvent>| => GToolButtonHoverParam {e},
        active_hover_out: GToolButtonEvent::HoverOut |e: Option<FingerHoverEvent>| => GToolButtonHoverParam {e},
        active_focus: GToolButtonEvent::Focus |e: Option<FingerDownEvent>| => GToolButtonFocusParam {e},
        active_focus_lost: GToolButtonEvent::FocusLost |e: Option<FingerUpEvent>| => GToolButtonFocusLostParam {e}
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        if self.event_key {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GToolButtonEvent::Clicked(GToolButtonClickedParam {
                        e,
                        icon_type: self.icon_type,
                    }),
                );
            }
        }
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_icon.redraw(cx);
        self.draw_tool_btn.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        if self.os_type.is_none() {
            self.os_type = Some(GOsType::get());
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.os_type.as_ref().unwrap().bg_color(self.icon_type);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.os_type.as_ref().unwrap().hover_color(self.icon_type);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.os_type.as_ref().unwrap().focus_color(self.icon_type);
        // ------------------ border color ----------------------------------------------
        let (border_color, border_width) = self
            .os_type
            .as_ref()
            .unwrap()
            .border_color_width(self.icon_type);
        let shadow_color = vec4(0.0, 0.0, 0.0, 0.0);
        let border_radius = match self.os_type.as_ref().unwrap() {
            GOsType::Windows | GOsType::Other => 0.0,
            GOsType::Mac | GOsType::Linux => 3.6,
        };
        let stroke_color = match self.os_type.as_ref().unwrap() {
            GOsType::Windows | GOsType::Other => self.color.use_or("#768390"),
            GOsType::Mac | GOsType::Linux => vec4(0.0, 0.0, 0.0, 0.0),
        };
        let stroke_hover_color = self.stroke_hover_color.use_or("#768390");
        let stroke_focus_color = self.stroke_focus_color.use_or("#768390");
        // apply over props to draw_button ----------------------------------------------
        self.draw_tool_btn.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: 1.0,
                border_color: (border_color),
                border_width: (border_width),
                border_radius: (border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                spread_radius: 0.0,
                blur_radius: 10.0
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                color: (stroke_color),
                stroke_width: (self.stroke_width),
                stroke_hover_color: (stroke_hover_color),
                stroke_focus_color: (stroke_focus_color),
            },
        );
        self.draw_icon.apply_type(Base::from(self.icon_type));
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_tool_btn.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        default_handle_animation!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, focus_area, Some(e));
            }
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, Some(e));
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, Some(e));
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, Some(e));
                } else {
                    self.play_animation(cx, id!(hover.off));
                    self.active_focus_lost(cx, Some(e));
                }
            }
            _ => (),
        };
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }
}

impl GToolButtonRef {
    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_area_ext!{
        area_icon
    }
    ref_event_option! {
        hover_in => GToolButtonHoverParam,
        hover_out => GToolButtonHoverParam,
        focus => GToolButtonFocusParam,
        focus_lost => GToolButtonFocusLostParam,
        clicked => GToolButtonClickedParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
}

impl GToolButtonSet {
    set_event! {
        hover_in => GToolButtonHoverParam,
        hover_out => GToolButtonHoverParam,
        focus => GToolButtonFocusParam,
        focus_lost => GToolButtonFocusLostParam,
        clicked => GToolButtonClickedParam
    }
}
