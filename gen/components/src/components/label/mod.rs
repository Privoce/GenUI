mod event;
mod register;

pub use event::*;
pub use register::register;

use crate::{
    animatie_fn,
    event::UnifiedEvent,
    event_option, play_animation, ref_animate_state, ref_area, ref_event_option, ref_redraw,
    set_scope_path, set_text_and_visible_fn,
    shader::draw_text::DrawGText,
    themes::Themes,
    utils::{get_font_family, set_cursor, ThemeColor, ToBool},
};
use makepad_widgets::*;
use shader::draw_text::TextWrap;
live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25;
    GLabelBase = {{GLabel}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_text: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_text: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_text: {hover: 0.0, focus: 1.0}
                    }
                }
            }
        }
    }
}

/// # GLabel component
/// A component that displays text.
/// ## Aniamtion
/// When you need to enable the default animation, you need to set animation_key to true
///
/// The default animation color is related to the passed theme
///
/// Label animations are divided into two types: Hover and Focus (Press)
/// - Hover: stroke_hover_color
/// - Press: stroke_focus_color
/// ## Event
/// When an event needs to be started, event_key needs to be set to true. See [`GLabelEvent`]
#[derive(Live, Widget)]
pub struct GLabel {
    #[live]
    pub theme: Themes,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    #[live]
    pub color: Option<Vec4>,
    #[live(9.0)]
    pub font_size: f64,
    // #[live(1.0)]
    // pub brightness: f32,
    // #[live(0.5)]
    // pub curve: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(1.5)]
    pub line_spacing: f64,
    // #[live(0.0)]
    // pub top_drop: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live(true)]
    pub visible: bool,
    // deref ---------------------
    #[redraw]
    #[live]
    pub draw_text: DrawGText,
    #[walk]
    pub walk: Walk,
    #[live]
    pub align: Align,
    #[live]
    pub padding: Padding,
    #[live]
    pub text: ArcStringMut,
    // animator -----------------
    #[live(false)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    #[rust]
    pub area: Area,
    #[live(false)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let padding = self.padding;
        let walk = walk.with_add_padding(padding);
        cx.begin_turtle(walk, Layout::default());
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;
        self.draw_text
            .draw_walk(cx, walk, self.align, self.text.as_ref());
        cx.end_turtle_with_area(&mut self.area);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        if self.animation_key {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_text.redraw(cx);
            }
        }
        match event.hits(cx, self.area()) {
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.play_animation(cx, id!(hover.on));
                self.active_hover_in(cx, e.clone());
                UnifiedEvent::hover_in(cx, self.widget_uid(), &scope.path, e);
            }
            Hit::FingerHoverOut(e) => {
                self.play_animation(cx, id!(hover.off));
                self.active_hover_out(cx, e.clone());
                UnifiedEvent::hover_out(cx, self.widget_uid(), &scope.path, e);
            }
            Hit::FingerDown(e) => {
                self.play_animation(cx, id!(hover.focus));
                self.active_focus(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                } else {
                    // focus lost
                    self.active_focus_lost(cx, e);
                    self.play_animation(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GLabel {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }

        self.render(cx);
    }
}

impl GLabel {
    set_scope_path!();
    play_animation!();
    fn active_hover_in(&mut self, cx: &mut Cx, e: FingerHoverEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GLabelEvent::HoverIn(GLabelHoverParam { e }),
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
                    GLabelEvent::HoverOut(GLabelHoverParam { e }),
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
                    GLabelEvent::Focus(GLabelFocusParam { e }),
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
                    GLabelEvent::FocusLost(GLabelFocusLostParam { e }),
                );
            });
        }
    }
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        let color = self.color.get(self.theme, 100);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 50);
        let stroke_focus_color = self.stroke_focus_color.get(self.theme, 200);
        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
                stroke_hover_color: (stroke_hover_color),
                stroke_focus_color: (stroke_focus_color),
                text_style: {
                    // brightness: (self.brightness),
                    // curve: (self.curve),
                    line_spacing: (self.line_spacing),
                    // top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
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
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
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
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }
    pub fn animate_state(&self) -> GLabelState {
        if self.draw_text.focus.to_bool() {
            return GLabelState::Focus;
        } else if self.draw_text.hover.to_bool() {
            return GLabelState::Hover;
        } else {
            return GLabelState::None;
        }
    }
    event_option! {
        hover_in: GLabelEvent::HoverIn => GLabelHoverParam,
        hover_out: GLabelEvent::HoverOut => GLabelHoverParam,
        focus: GLabelEvent::Focus => GLabelFocusParam,
        focus_lost: GLabelEvent::FocusLost => GLabelFocusLostParam
    }
}

impl GLabelRef {
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    ref_area!();
    ref_animate_state!();
    ref_redraw!();
    ref_event_option! {
        hover_in  => GLabelHoverParam,
        hover_out => GLabelHoverParam,
        focus => GLabelFocusParam,
        focus_lost => GLabelFocusLostParam
    }
}
