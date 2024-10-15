mod event;
mod register;
pub mod types;

pub use event::*;
pub use register::register;
use types::LinkType;

use crate::shader::draw_link::DrawGLink;
use crate::shader::draw_text::DrawGText;
use crate::themes::Themes;
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::open_browser;
use crate::utils::{get_font_family, set_cursor, BoolToF32, ThemeColor};
use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down,
    default_hit_hover_in, default_hit_hover_out, event_option, play_animation, ref_area,
    ref_event_option, ref_redraw, ref_render, set_event, set_scope_path, set_text_and_visible_fn,
    widget_area,
};
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
                        draw_link: {focus: 0.0, hover: 0.0}
                        draw_text: {focus: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_link: {focus: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {focus: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_link: {focus: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {focus: [{time: 0.0, value: 1.0}], hover: 1.0,}
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
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(true)]
    pub underline_visible: bool,
    #[live]
    pub underline_color: Option<Vec4>,
    #[live]
    pub underline_hover_color: Option<Vec4>,
    #[live]
    pub underline_focus_color: Option<Vec4>,
    #[live(1.0)]
    pub underline_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live(false)]
    pub round: bool,
    #[live(false)]
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
    pub text_focus_color: Option<Vec4>,
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
    pub animation_key: bool,
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
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GLink {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
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

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let _ = self.set_scope_path(&scope.path);
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
        self.render(cx);
    }
}

impl GLink {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_link,
        area_text, draw_text
    }
    active_event! {
        active_hover_in: GLinkEvent::HoverIn |e: FingerHoverEvent| => GLinkHoverParam { e },
        active_hover_out: GLinkEvent::HoverOut |e: FingerHoverEvent| => GLinkHoverParam { e },
        active_focus: GLinkEvent::Focus |e: FingerDownEvent| => GLinkFocusParam { e },
        active_focus_lost: GLinkEvent::FocusLost |e: FingerUpEvent| => GLinkFocusLostParam { e }
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, e: FingerUpEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GLinkEvent::Clicked(GLinkClickedParam {
                        href: self.href.clone(),
                        ty: self.link_type,
                        e,
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: GLinkEvent::HoverIn => GLinkHoverParam,
        hover_out: GLinkEvent::HoverOut => GLinkHoverParam,
        focus: GLinkEvent::Focus => GLinkFocusParam,
        focus_lost: GLinkEvent::FocusLost => GLinkFocusLostParam,
        clicked: GLinkEvent::Clicked => GLinkClickedParam
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_link.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
        self.draw_link.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) {
        // backgroud visible is true, means link act as a button, text color should be plain
        let (
            background_color,
            hover_color,
            focus_color,
            text_color,
            text_hover_color,
            text_focus_color,
            underline_color,
            underline_hover_color,
            underline_focus_color,
        ) = if self.background_visible {
            (
                self.background_color.get(self.theme, 500),
                self.hover_color.get(self.theme, 400),
                self.focus_color.get(self.theme, 600),
                self.color.get(self.theme, 50),
                self.text_hover_color.get(self.theme, 25),
                self.text_focus_color.get(self.theme, 100),
                self.underline_color.get(self.theme, 50),
                self.underline_hover_color.get(self.theme, 25),
                self.underline_focus_color.get(self.theme, 100),
            )
        } else {
            (
                self.background_color.get(self.theme, 500),
                self.hover_color.get(self.theme, 400),
                self.focus_color.get(self.theme, 600),
                self.color.get(self.theme, 500),
                self.text_hover_color.get(self.theme, 400),
                self.text_focus_color.get(self.theme, 600),
                self.underline_color.get(self.theme, 500),
                self.underline_hover_color.get(self.theme, 400),
                self.underline_focus_color.get(self.theme, 600),
            )
        };
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ is background_visible -------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ underline -------------------------------------------------
        let underline_visible = self.underline_visible.to_f32();
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
                background_color: (background_color),
                border_color: (border_color),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
                underline_visible: (underline_visible),
                underline_color: (underline_color),
                underline_width: (self.underline_width),
                underline_hover_color: (underline_hover_color),
                underline_focus_color: (underline_focus_color),
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (text_color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
                text_style: {
                    font_size: (self.font_size),
                },
            },
        );
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                focus: 0.0
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
        default_handle_animation!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                // if self.grab_key_focus {
                //     cx.set_key_focus(focus_area);
                // }
                // cx.widget_action(uid, &scope.path, GLinkEvent::Pressed(f_down.clone()));
                // self.animator_play(cx, id!(hover.focus));
                default_hit_finger_down!(self, cx, focus_area, e);
            }
            Hit::FingerHoverIn(e) => {
                // let _ = set_cursor(cx, self.cursor.as_ref());
                // self.animator_play(cx, id!(hover.on));
                // cx.widget_action(uid, &scope.path, GLinkEvent::Hover(h.clone()));
                default_hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                // self.animator_play(cx, id!(hover.off));
                default_hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }

                    let _ = self.href.as_ref().map(|x| {
                        #[cfg(not(target_arch = "wasm32"))]
                        open_browser(&x)
                    });

                    cx.widget_action(
                        uid,
                        &scope.path,
                        GLinkEvent::Clicked(GLinkClickedParam {
                            href: self.href.clone(),
                            ty: self.link_type,
                            e,
                        }),
                    );
                } else {
                    self.animator_play(cx, id!(hover.off));
                    cx.widget_action(
                        self.widget_uid(),
                        self.scope_path.as_ref().unwrap(),
                        GLinkEvent::FocusLost(GLinkFocusLostParam { e }),
                    );
                }
            }
            _ => (),
        }
    }
}

impl GLinkRef {
    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_event_option! {
        hover_in => GLinkHoverParam,
        hover_out => GLinkHoverParam,
        focus => GLinkFocusParam,
        focus_lost => GLinkFocusLostParam,
        clicked => GLinkClickedParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
}

impl GLinkSet {
    set_event! {
        hover_in => GLinkHoverParam,
        hover_out => GLinkHoverParam,
        focus => GLinkFocusParam,
        focus_lost => GLinkFocusLostParam,
        clicked => GLinkClickedParam
    }
}
