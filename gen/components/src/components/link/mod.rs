mod register; 
pub mod event;
pub mod types;

use types::LinkType;
use event::{GLinkClickedParam, GLinkEvent};
pub use register::register;

use crate::shader::draw_link::DrawGLink;
use crate::shader::draw_text::DrawGText;
use crate::themes::Themes;
use crate::utils::{get_font_family, set_cursor, BoolToF32, ThemeColor};
use crate::{animatie_fn, event_option, ref_event_option, set_event, set_text_and_visible_fn, widget_area};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

    GLinkBase = {{GLink}}{
        height: Fit,
        width: Fit,
        text_walk: {
            height: Fit,
            width: Fit,
        },
        border_radius: 0.0,
        cursor: Hand,
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_link: {pressed: 0.0, hover: 0.0}
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_link: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_link: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GLink {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(true)]
    pub underline: bool,
    #[live]
    pub underline_color: Option<Vec4>,
    #[live]
    pub underline_hover_color: Option<Vec4>,
    #[live]
    pub underline_pressed_color: Option<Vec4>,
    #[live(1.0)]
    pub underline_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live(false)]
    pub round: bool,
    #[live(true)]
    pub background_visible: bool,
    // text -----------------
    #[live]
    pub text: ArcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_pressed_color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub cursor: Option<MouseCursor>,
    // href -------------------
    #[live]
    pub href: Option<String>,
    #[live]
    pub link_type: LinkType,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // define area -----------------
    #[live]
    pub draw_text: DrawGText,
    #[live]
    pub text_walk: Walk,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    pub draw_link: DrawGLink,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
}


impl Widget for GLink {
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
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;

        let _ = self.draw_link.begin(cx, walk, self.layout);

        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());

        self.draw_link.end(cx);
        DrawStep::done()
    }

    set_text_and_visible_fn!();
}

impl LiveHook for GLink {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        let text_hover_color = self.text_hover_color.get(self.theme, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = self.pressed_color.get(self.theme, 600);
        let text_pressed_color = self.text_pressed_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ underline color ------------------------------------------
        let underline_color = self.underline_color.get(self.theme, 500);
        let underline_hover_color = self.underline_hover_color.get(self.theme, 400);
        let underline_pressed_color = self.underline_pressed_color.get(self.theme, 600);
        // ------------------ font ------------------------------------------------------
        let font_color = self.color.get(self.theme, 500);
        // ------------------ is background_visible -------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ underline -------------------------------------------------
        let underline = self.underline.to_f32();
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
        // apply over props to draw_link ----------------------------------------------
        self.draw_link.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_radius: (self.border_radius),
                pressed_color: (pressed_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
                underline: (underline),
                underline_color: (underline_color),
                underline_width: (self.underline_width),
                underline_hover_color: (underline_hover_color),
                underline_pressed_color: (underline_pressed_color)
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                hover_color: (text_hover_color),
                pressed_color: (text_pressed_color),
                text_style: {
                    font_size: (self.font_size),
                },
            },
        );
        self.draw_link.redraw(cx);
        self.draw_text.redraw(cx);
    }
}

impl GLink {
    widget_area! {
        area, draw_link
    }
    event_option! {
        clicked : GLinkEvent::Clicked => GLinkClickedParam,
        pressed : GLinkEvent::Pressed => FingerDownEvent,
        released : GLinkEvent::Released => FingerUpEvent,
        hover: GLinkEvent::Hover => FingerHoverEvent
    }

    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0
            },
        );
    }
    pub fn animate_pressed(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
                pressed: 1.0
            },
        );
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
        if self.animation_open {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_link.redraw(cx);
            }
        }

        match hit {
            Hit::FingerDown(f_down) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
                cx.widget_action(uid, &scope.path, GLinkEvent::Pressed(f_down.clone()));
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(h) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GLinkEvent::Hover(h.clone()));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GLinkEvent::Clicked(GLinkClickedParam{
                            href: self.href.clone(),
                            ty: self.link_type,
                            e: f_up.clone(),
                        }),
                    );
                    cx.widget_action(uid, &scope.path, GLinkEvent::Released(f_up.clone()));
                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    cx.widget_action(uid, &scope.path, GLinkEvent::Released(f_up.clone()));
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
}

impl GLinkRef {
    ref_event_option! {
        clicked => GLinkClickedParam,
        released => FingerUpEvent,
        pressed => FingerDownEvent,
        hover => FingerHoverEvent
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_pressed
    }
}

impl GLinkSet {
    set_event! {
        clicked => GLinkClickedParam,
        released => FingerUpEvent,
        pressed => FingerDownEvent,
        hover => FingerHoverEvent
    }
}