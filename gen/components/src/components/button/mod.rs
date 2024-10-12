mod event;
mod register;

pub use event::*;
pub use register::register;

use crate::event::FocusType;
use crate::utils::{set_cursor, BoolToF32, ThemeColor};
use crate::{
    animatie_fn, event_option, play_animation, ref_area, ref_event_option, set_event,
    set_scope_path, widget_area,
};
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
                        draw_button: {hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_button: {hover: 1.0}
                    }
                }
            }
            focus = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {focus: 0.0}
                    }
                }

                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {focus: 1.0}
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
    pub focus_color: Option<Vec4>,
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
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GButton {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, hit, focus_area)
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.draw_button.begin(cx, walk, self.layout);

        if self.slot.is_visible() {
            let slot_walk = self.slot.walk(cx);
            let _ = self.slot.draw_walk(cx, scope, slot_walk);
        }

        self.draw_button.end(cx);

        self.set_scope_path(&scope.path);

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
        self.render(cx);
    }
}

impl GButton {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_button
    }
    event_option! {
        hover_in: GButtonEvent::HoverIn => GButtonHoverParam,
        hover_out: GButtonEvent::HoverOut => GButtonHoverParam,
        focus: GButtonEvent::Focus => GButtonFocusParam,
        focus_lost: GButtonEvent::FocusLost => GButtonFocusLostParam,
        clicked: GButtonEvent::Clicked => GButtonClickedParam
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
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
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, focus_area: Area) {
        if self.animation_key {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_button.redraw(cx);
            }
        }
        match hit {
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                self.play_animation(cx, id!(focus.on));
                self.active_focus(cx, e);
            }
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.play_animation(cx, id!(hover.on));
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.play_animation(cx, id!(hover.off));
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.play_animation(cx, id!(focus.off));
                    self.active_focus_lost(cx, e);
                }
            }
            _ => (),
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_button.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }
    fn active_hover_in(&mut self, cx: &mut Cx, e: FingerHoverEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GButtonEvent::HoverIn(GButtonHoverParam { e }),
                );
            });
        }
    }
    fn active_hover_out(&mut self, cx: &mut Cx, e: FingerHoverEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GButtonEvent::HoverOut(GButtonHoverParam { e }),
                );
            });
        }
    }
    fn active_focus(&mut self, cx: &mut Cx, e: FingerDownEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GButtonEvent::Focus(GButtonFocusParam {
                        ty: FocusType::Press,
                        e,
                    }),
                );
            });
        }
    }
    fn active_focus_lost(&mut self, cx: &mut Cx, e: FingerUpEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GButtonEvent::FocusLost(GButtonFocusLostParam { e }),
                );
            });
        }
    }
    fn active_clicked(&mut self, cx: &mut Cx, e: FingerUpEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GButtonEvent::Clicked(GButtonClickedParam { e }),
                );
            });
        }
    }
}

impl GButtonRef {
    ref_event_option! {
        hover_in => GButtonHoverParam,
        hover_out => GButtonHoverParam,
        focus => GButtonFocusParam,
        focus_lost => GButtonFocusLostParam,
        clicked => GButtonClickedParam
    }
    ref_area!();
    // pub fn area(&self) -> Area {
    //     if let Some(btn_ref) = self.borrow() {
    //         return btn_ref.area();
    //     }
    //     Area::Empty
    // }

    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
}

impl GButtonSet {
    set_event! {
        hover_in => GButtonHoverParam,
        hover_out => GButtonHoverParam,
        focus => GButtonFocusParam,
        focus_lost => GButtonFocusLostParam,
        clicked => GButtonClickedParam
    }
}
