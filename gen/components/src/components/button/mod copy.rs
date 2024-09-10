mod register;

pub use register::register;

use crate::utils::{get_font_family, set_cursor, ThemeColor};
use crate::{shader::draw_card::DrawCard, themes::Themes};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GButtonBase = {{GButton}}{
        height: Fit,
        width: Fit,
        text_walk: {
            height: Fit,
            width: Fit,
        }
        cursor: Hand,
        draw_text: {
            instance hover: 0.0,
            instance pressed: 0.0,

            fn get_color(self) -> vec4 {
                let hover_color = self.color - vec4(0.0, 0.0, 0.0, 0.1);
                let pressed_color = self.color - vec4(0.0, 0.0, 0.0, 0.2);

                return mix(
                    mix(
                        self.color,
                        hover_color,
                        self.hover
                    ),
                    pressed_color,
                    self.pressed
                )
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {pressed: 0.0, hover: 0.0}
                        // draw_icon: {pressed: 0.0, hover: 0.0}
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_button: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        // draw_icon: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}]}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        // draw_icon: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
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
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub text_pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live(false)]
    pub round: bool,
    // text -----------------
    #[live]
    pub text: ArcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub cursor: Option<MouseCursor>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // define area -----------------
    #[live]
    draw_text: DrawText,
    #[live]
    text_walk: Walk,
    #[live(true)]
    grab_key_focus: bool,
    // animator -----------------
    #[animator]
    animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    draw_button: DrawCard,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GButtonEvent {
    Hovered(KeyModifiers),
    Clicked(KeyModifiers),
    Released(KeyModifiers),
    Pressed(KeyModifiers),
    None,
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
            self.draw_button.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let focus_area = self.draw_button.area();
        let hit = event.hits(cx, self.draw_button.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;

        let _ = self.draw_button.begin(cx, walk, self.layout);

        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());

        self.draw_button.end(cx);
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GButton {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ font ------------------------------------------------------
        let font_color = self.color.get(self.theme, 100);
        // ------------------ round -----------------------------------------------------
        if self.round {
            self.border_radius = match self.walk.height {
                Size::Fixed(h) => (h * 0.25) as f32,
                Size::Fit => {
                    ((self.draw_text.text_style.font_size
                        + self.layout.padding.top
                        + self.layout.padding.bottom)
                        * 0.25) as f32
                }
                _ => panic!("round only support fixed and fit"),
            };
        }
        // apply over props to draw_button ----------------------------------------------
        // ⚠️⚠️⚠️ attention! ⚠️⚠️⚠️ self.apply_over may cause a stack overflow
        // self.apply_over(
        //     cx,
        //     live! {
        //         // show_bg: true,
        //         draw_text: {
        //             color: (font_color),
        //             text_style: {
        //                 font_size: (self.font_size),
        //             },
        //         }
        //     },
        // );
        // self.redraw(cx);
        self.draw_button.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                text_style: {
                    font_size: (self.font_size),
                },
            },
        );
        self.draw_button.redraw(cx);
        self.draw_text.redraw(cx);
    }
}

impl GButton {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let GButtonEvent::Clicked(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
    pub fn pressed(&self, actions: &Actions) -> bool {
        if let GButtonEvent::Pressed(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
    pub fn released(&self, actions: &Actions) -> bool {
        if let GButtonEvent::Released(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
    pub fn hovered(&self, actions: &Actions) -> bool {
        if let GButtonEvent::Hovered(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
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

        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_button.redraw(cx);
        }
        match hit {
            Hit::FingerDown(f_down) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                cx.widget_action(uid, &scope.path, GButtonEvent::Pressed(f_down.modifiers));
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(h) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GButtonEvent::Hovered(h.modifiers));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GButtonEvent::Clicked(f_up.modifiers));
                    cx.widget_action(uid, &scope.path, GButtonEvent::Released(f_up.modifiers));
                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    cx.widget_action(uid, &scope.path, GButtonEvent::Released(f_up.modifiers));
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
}

impl GButtonRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.clicked(actions);
        }
        false
    }
    pub fn released(&self, actions: &Actions) -> bool {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.released(actions);
        }
        false
    }
    pub fn pressed(&self, actions: &Actions) -> bool {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.pressed(actions);
        }
        false
    }
    pub fn hovered(&self, actions: &Actions) -> bool {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.hovered(actions);
        }
        false
    }
    pub fn area(&self) -> Area {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.draw_button.area();
        }
        Area::Empty
    }
}

impl GButtonSet {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.clicked(actions))
    }
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.pressed(actions))
    }
    pub fn released(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.released(actions))
    }
    pub fn hovered(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.hovered(actions))
    }
}
