mod event;
mod register;

pub use event::*;
pub use register::register;

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down, default_hit_hover_in, default_hit_hover_out, event::UnifiedEvent, event_option, play_animation, ref_animate_state, ref_area, ref_event_option, ref_play_animation, ref_redraw, ref_render, set_scope_path, set_text_and_visible_fn, shader::draw_text::DrawGText, themes::Themes, utils::{get_font_family, set_cursor, ThemeColor, ToBool}
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
/// The `GLabel` component is a customizable label widget with animation and event handling features. It allows for hover, focus, and text styling through various properties, animations, and events.
///
/// ## Animation
/// This component supports animations, particularly for hover and focus states. The default hover and focus animations are defined using the `animator` field:
/// - **hover.off**:  
///   - `draw_text.hover`: changes to `0.0`  
///   - `draw_text.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.on**:  
///   - `draw_text.hover`: changes to `1.0`  
///   - `draw_text.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.focus**:  
///   - `draw_text.hover`: changes to `0.0`  
///   - `draw_text.focus`: changes to `1.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
///
/// Animations are controlled by the `Animator` property, which defines the behavior for transitions between hover and focus states.
///
/// ## Event
/// The `GLabel` component supports the following events:
/// - `HoverIn`: Triggered when a user hovers over the label.
/// - `HoverOut`: Triggered when the user moves the cursor away from the label.
/// - `Focus`: Triggered when the label is clicked or focused.
/// - `FocusLost`: Triggered when the label loses focus.
///
/// These events allow interaction with the label and can be used to trigger actions or further animations in response to user input.
///
/// ## Props
/// | Macro   | Prop                | Description                                      | Type            | Default     |
/// |---------|---------------------|--------------------------------------------------|-----------------|-------------|
/// | live    | stroke_hover_color   | The color of the text stroke when hovered        | `Option<Vec4>`  | `None`      |
/// | live    | stroke_focus_color   | The color of the text stroke when focused        | `Option<Vec4>`  | `None`      |
/// | live    | color                | The base color of the text                       | `Option<Vec4>`  | `None`      |
/// | live    | font_size            | The font size of the label text                  | `f64`           | `9.0`       |
/// | live    | cursor               | The cursor type when hovering over the label     | `Option<MouseCursor>` | `None`      |
/// | live    | line_spacing         | The line spacing between the label text          | `f64`           | `1.5`       |
/// | live    | height_factor        | Factor controlling the height of the text        | `f64`           | `0.0`       |
/// | live    | wrap                 | Text wrapping behavior                          | `TextWrap`      | `TextWrap::Word` |
/// | live    | font_family          | The font family used for the label               | `LiveDependency` | N/A         |
/// | live    | visible              | Whether the label is visible                     | `bool`          | `true`      |
/// | redraw  | draw_text            | Controls the drawing of the label's text         | `DrawGText`     | N/A         |
/// | walk    | walk                 | Defines the positioning of the label             | `Walk`          | N/A         |
/// | live    | align                | Text alignment                                  | `Align`         | N/A         |
/// | live    | padding              | The padding around the label                    | `Padding`       | N/A         |
/// | live    | text                 | The text content of the label                   | `ArcStringMut`  | N/A         |
/// | animator| animator             | Handles animation states for hover and focus    | `Animator`      | N/A         |
/// | rust    | area                 | Represents the area occupied by the label       | `Area`          | N/A         |
/// | live    | event_key            | Controls the event triggering behavior           | `bool`          | `false`     |
/// | live    | grab_key_focus       | Whether the label grabs keyboard focus           | `bool`          | `true`      |
/// | rust    | scope_path           | The path scope for the label                    | `Option<HeapLiveIdPath>` | N/A  |
/// > N/A: Default::default()
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
    #[live(true)]
    pub grab_key_focus: bool,
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
        let _ = self.text.as_ref().is_empty().then(|| {
            let _ = self.set_text(" ");
        });
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
        default_handle_animation!(self, cx, event);
        match event.hits(cx, self.area()) {
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, e.clone());
                UnifiedEvent::hover_in(cx, self.widget_uid(), &scope.path, e);
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, e.clone());
                UnifiedEvent::hover_out(cx, self.widget_uid(), &scope.path, e);
            }
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, self.area(), e);
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
    active_event!{
        active_hover_in: GLabelEvent::HoverIn |e: FingerHoverEvent| => GLabelHoverParam{ e },
        active_hover_out: GLabelEvent::HoverOut |e: FingerHoverEvent| => GLabelHoverParam{ e },
        active_focus: GLabelEvent::Focus |e: FingerDownEvent| => GLabelFocusParam{ e },
        active_focus_lost: GLabelEvent::FocusLost |e: FingerUpEvent| => GLabelFocusLostParam{ e }
    }
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        let color = self.color.get(self.theme, 50);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 25);
        let stroke_focus_color = self.stroke_focus_color.get(self.theme, 100);
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
    ref_render!();
    ref_play_animation!{
        play_hover_on: id!(hover.on),
        play_hover_off: id!(hover.off),
        play_focus_on: id!(hover.focus),
        play_focus_off: id!(hover.off)
    }
    ref_event_option! {
        hover_in  => GLabelHoverParam,
        hover_out => GLabelHoverParam,
        focus => GLabelFocusParam,
        focus_lost => GLabelFocusLostParam
    }
}
