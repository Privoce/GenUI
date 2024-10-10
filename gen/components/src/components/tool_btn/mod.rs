pub mod event;
mod register;
pub mod types;

use event::*;
pub use register::register;

use makepad_widgets::*;
use types::{GOsType, GToolButtonType};

use crate::{
    animatie_fn, event_option, ref_event_option, set_event,
    shader::{
        draw_view::DrawGView,
        icon_lib::{base::DrawGIconBase, types::base::Base},
    },
    utils::{set_cursor, ThemeColor},
    widget_area,
};

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25;
    GToolButtonBase = {{GToolButton}} {
        height: 32.0,
        width: 46.0,
        stroke_color: #768390,
        stroke_hover_color: #768390,
        stroke_pressed_color: #768390,
        draw_icon: {
            instance pressed: 0.0,
            instance stroke_pressed_color: vec4,
            fn stroke_color(self) -> vec4 {
                return mix(
                    mix(
                        self.stroke_color,
                        self.stroke_pressed_color,
                        self.pressed
                    ),
                    self.stroke_hover_color,
                    self.hover
                );
            }
        }
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_icon: {hover: 0.0, pressed: 0.0},
                        draw_tool_btn: {hover: 0.0, pressed: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_icon: {hover: 1.0, pressed: 0.0},
                        draw_tool_btn: {hover: 1.0, pressed: 0.0}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_icon: {hover: 0.0, pressed: 1.0},
                        draw_tool_btn: {hover: 0.0, pressed: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GToolButton {
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live(1.0)]
    pub stroke_width: f32,
    #[live]
    pub stroke_pressed_color: Option<Vec4>,
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
}

impl Widget for GToolButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let mut icon_walk = Walk::default();
        match self.os_type.as_ref().unwrap() {
            GOsType::Windows | GOsType::Other => {
                icon_walk.width = Size::Fixed(16.0);
                icon_walk.height = Size::Fixed(16.0);
            }
            GOsType::Mac | GOsType::Linux => {
                walk.width = Size::Fixed(18.0);
                walk.height = Size::Fixed(18.0);
                icon_walk.width = Size::Fixed(10.0);
                icon_walk.height = Size::Fixed(10.0);
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
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
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
        if self.os_type.is_none() {
            self.os_type = Some(GOsType::get());
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.os_type.as_ref().unwrap().bg_color(self.icon_type);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.os_type.as_ref().unwrap().hover_color(self.icon_type);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.os_type.as_ref().unwrap().pressed_color(self.icon_type);
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
            GOsType::Windows | GOsType::Other => self.stroke_hover_color.use_or("#768390"),
            GOsType::Mac | GOsType::Linux => vec4(0.0, 0.0, 0.0, 0.0),
        };
        let stroke_hover_color = self.stroke_hover_color.use_or("#768390");
        let stroke_pressed_color = self.stroke_pressed_color.use_or("#768390");
        // apply over props to draw_button ----------------------------------------------
        self.draw_tool_btn.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: 1.0,
                border_color: (border_color),
                border_width: (border_width),
                border_radius: (border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                spread_radius: 0.0,
                blur_radius: 10.0
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                stroke_color: (stroke_color),
                stroke_width: (self.stroke_width),
                stroke_hover_color: (stroke_hover_color),
                stroke_pressed_color: (stroke_pressed_color),
            },
        );
        self.draw_icon.apply_type(Base::from(self.icon_type));
        self.draw_icon.redraw(cx);
        self.draw_tool_btn.redraw(cx);
    }
}

impl GToolButton {
    widget_area! {
        area, draw_tool_btn
    }
    event_option! {
        clicked: GToolButtonEvent::Clicked => GToolButtonClickParam,
        pressed: GToolButtonEvent::Pressed => FingerDownEvent,
        hover: GToolButtonEvent::Hover => FingerHoverEvent
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        if self.animation_key {
            self.animator_handle_event(cx, event);
        }
        let uid = self.widget_uid();

        match hit {
            Hit::FingerDown(fe) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                cx.widget_action(uid, &scope.path, GToolButtonEvent::Pressed(fe.clone()));
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(f_in) => {
                set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GToolButtonEvent::Hover(f_in.clone()));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(uid, &scope.path, GToolButtonEvent::Clicked(GToolButtonClickParam{
                        e: fe.clone(),
                        mode: self.icon_type,
                    }));
                    if fe.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    // cx.widget_action(uid, &scope.path, ButtonAction::Released(fe.modifiers));
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        };
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_pressed(&mut self, cx: &mut Cx) -> () {
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 1.0
            },
        );
    }
}

impl GToolButtonRef {
    ref_event_option! {
        clicked => GToolButtonClickParam,
        pressed => FingerDownEvent,
        hover => FingerHoverEvent
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_pressed
    }
}

impl GToolButtonSet {
    set_event! {
        clicked => GToolButtonClickParam,
        pressed => FingerDownEvent,
        hover => FingerHoverEvent
    }
}
