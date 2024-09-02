use crate::shader::draw_link::DrawGLink;
use crate::shader::draw_text::DrawGText;
use crate::themes::{get_color, Themes};
use crate::utils::{get_font_family, set_cursor};
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
    #[live(1.0)]
    pub underline_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live(false)]
    pub round: bool,
    #[live(true)]
    transparent: bool,
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
    // href -------------------
    #[live]
    href: Option<String>,
    #[live]
    link_type: LinkType,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // define area -----------------
    #[live]
    draw_text: DrawGText,
    // draw_text: DrawText,
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
    draw_link: DrawGLink,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

#[derive(Copy, Clone, Live, LiveHook, Debug)]
#[live_ignore]
pub enum LinkType {
    #[pick]
    NewTab,
    SameTab,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GLinkEvent {
    Hovered(KeyModifiers),
    /// clicked(key_modifiers, href, link_type)
    Clicked((KeyModifiers, Option<String>, LinkType)),
    Released(KeyModifiers),
    Pressed(KeyModifiers),
    None,
}

impl Widget for GLink {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_link.redraw(cx);
        }
        match event.hits(cx, self.draw_link.area()) {
            Hit::FingerDown(f_down) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.draw_link.area());
                }
                cx.widget_action(uid, &scope.path, GLinkEvent::Pressed(f_down.modifiers));
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(h) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.animator_play(cx, id!(hover.on));
                cx.widget_action(uid, &scope.path, GLinkEvent::Hovered(h.modifiers));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GLinkEvent::Clicked((f_up.modifiers, self.href.clone(), self.link_type)),
                    );
                    cx.widget_action(uid, &scope.path, GLinkEvent::Released(f_up.modifiers));
                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    cx.widget_action(uid, &scope.path, GLinkEvent::Released(f_up.modifiers));
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
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

impl LiveHook for GLink {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        let underline_color = get_color(self.theme, self.underline_color, 500);
        // ------------------ font ------------------------------------------------------
        let font_color = get_color(self.theme, self.color, 500);
        // ------------------ is transparent --------------------------------------------
        let transparent = (self.transparent) as u8 as f32;
        // ------------------ underline -------------------------------------------------
        let underline = (self.underline) as u8 as f32;
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
                transparent: (transparent),
                underline: (underline),
                underline_color: (underline_color),
                underline_width: (self.underline_width),
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                hover_color: (hover_color),
                pressed_color: (pressed_color),
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
    pub fn clicked(&self, actions: &Actions) -> Option<(KeyModifiers, Option<String>, LinkType)> {
        if let GLinkEvent::Clicked(e) = actions.find_widget_action(self.widget_uid()).cast() {
            Some(e)
        } else {
            None
        }
    }
    pub fn pressed(&self, actions: &Actions) -> bool {
        if let GLinkEvent::Pressed(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
    pub fn released(&self, actions: &Actions) -> bool {
        if let GLinkEvent::Released(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
    pub fn hovered(&self, actions: &Actions) -> bool {
        if let GLinkEvent::Hovered(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
}

impl GLinkRef {
    pub fn clicked(&self, actions: &Actions) -> Option<(KeyModifiers, Option<String>, LinkType)> {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.clicked(actions);
        }
        None
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
}

impl GLinkSet {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.iter()
            .any(|btn_ref| btn_ref.clicked(actions).is_some())
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
