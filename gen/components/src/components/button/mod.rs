pub mod event;
mod register;

use event::GButtonEvent;
pub use register::register;

use crate::utils::{set_cursor, BoolToF32, ThemeColor};
use crate::{animatie_fn, event_option, ref_event_option, set_event, widget_area};
use crate::{shader::draw_view::DrawGView, themes::Themes};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GButtonBase = {{GButton}}{
        clip_x: false,
        clip_y: false,
        cursor: Hand,
        shadow_offset: vec2(0.0, 2.0),
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_button: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}]}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GButton {
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
    pub cursor: Option<MouseCursor>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // ---------------------------
    #[find]
    #[redraw]
    #[live]
    pub slot: WidgetRef,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    pub draw_button: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GButton {
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
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.draw_button.begin(cx, walk, self.layout);

        let slot_walk = self.slot.walk(cx);
        let _ = self.slot.draw_walk(cx, scope, slot_walk);

        self.draw_button.end(cx);
        DrawStep::done()
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GButton {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        // apply over props to draw_button ----------------------------------------------
        self.draw_button.apply_over(
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
        self.draw_button.redraw(cx);
    }
}

impl GButton {
    widget_area! {
        area, draw_button
    }
    event_option! {
        clicked: GButtonEvent::Clicked => FingerUpEvent,
        pressed: GButtonEvent::Pressed => FingerDownEvent,
        released: GButtonEvent::Released => FingerUpEvent,
        hover: GButtonEvent::Hover => FingerHoverEvent
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        let uid = self.widget_uid();

        if self.animation_key {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_button.redraw(cx);
            }
        }
        match hit {
            Hit::FingerDown(f_down) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                cx.widget_action(uid, &scope.path, GButtonEvent::Pressed(f_down.clone()));
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(h) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GButtonEvent::Hover(h.clone()));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GButtonEvent::Clicked(f_up.clone()));
                    cx.widget_action(uid, &scope.path, GButtonEvent::Released(f_up.clone()));
                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    cx.widget_action(uid, &scope.path, GButtonEvent::Released(f_up.clone()));
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_pressed(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 1.0
            },
        );
    }
}

impl GButtonRef {
    ref_event_option! {
        clicked => FingerUpEvent,
        released => FingerUpEvent,
        pressed => FingerDownEvent,
        hover => FingerHoverEvent
    }

    pub fn area(&self) -> Area {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.area();
        }
        Area::Empty
    }

    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_pressed
    }
}

impl GButtonSet {
    set_event! {
        clicked => FingerUpEvent,
        released => FingerUpEvent,
        pressed => FingerDownEvent,
        hover => FingerHoverEvent
    }
}
