//! # Label component
//! A component that displays text.
//! ## Props
//! |decorate|name|type|description|
//! |--|--|--|--|
//! |live|color|`Vec4`|The color of the label.|
//! |live|font_size|`f64`|The size of the font used in the label.|
//! |live|brightness(unused)|`f32`|The brightness level of the text.|
//! |live|curve(unused)|`f32`|The curve factor of the text.|
//! |live|line_spacing|`f64`|The line spacing of the text.|
//! |live|top_drop|`f64`|The top drop of the text.|
//! |live|height_factor|`f64`|The height factor of the text.|
//! |live|wrap|`TextWrap`|The text wrapping mode.|
//! |live|font_family|`LiveDependency`|The font family of the text.|
//! |live|visible|`bool`|Whether the label is visible.|
//! |deref|draw_text|`DrawText`|The `DrawText` component used for drawing the text.|
//! |walk|height|`Size`|The height of the label|
//! |walk|width|`Size`|The width of the label|
//! |live|align|`Align`|The alignment of the text.|
//! |live|padding|`Padding`|The padding around the text. default `0.0`|
//! |live|text|`ArcStringMut`|The content of the label.|
//! ## Events
//! None
//! ## Example
//! See [label example](https://privoce.github.io/GenUI.github.io/gen/makepad/components/label.html)
mod register;

pub use register::register;

use crate::{
    set_text_and_visible_fn,
    shader::draw_text::DrawGText,
    themes::Themes,
    utils::{get_font_family, set_cursor, ThemeColor},
    widget_area,
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
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

/// ## GLabel component
/// A component that displays text.
/// This component has no events. If you need add event on this component, you should wrap a view outside
#[derive(Live, Widget)]
pub struct GLabel {
    #[live]
    pub theme: Themes,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_pressed_color: Option<Vec4>,
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
    #[live(0.0)]
    pub top_drop: f64,
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
    pub animation_open: bool,
    #[animator]
    animator: Animator,
    #[rust]
    area: Area,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
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
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if self.animation_open {
            let uid = self.widget_uid();
            if self.animator_handle_event(cx, event).must_redraw() {
                self.draw_text.redraw(cx);
            }
            match event.hits_with_capture_overload(cx, self.area, true) {
                Hit::FingerHoverIn(_) => {
                    self.animator_play(cx, id!(hover.on));
                }
                Hit::FingerHoverOut(_) => {
                    self.animator_play(cx, id!(hover.off));
                }
                Hit::FingerDown(_) => {
                    self.animator_play(cx, id!(hover.pressed));
                }
                Hit::FingerUp(f_up) => {
                    if f_up.is_over {
                        if f_up.device.has_hovers() {
                            self.animator_play(cx, id!(hover.on));
                        } else {
                            self.animator_play(cx, id!(hover.off));
                        }
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                }
                _ => (),
            }
        }
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GLabel {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }

        let color = self.color.get(self.theme, 800);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 800);
        let stroke_pressed_color = self.stroke_pressed_color.get(self.theme, 800);
        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
                hover_color: (stroke_hover_color),
                pressed_color: (stroke_pressed_color),
                text_style: {
                    // brightness: (self.brightness),
                    // curve: (self.curve),
                    line_spacing: (self.line_spacing),
                    top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
        self.draw_text.redraw(cx);
    }
}

impl GLabel {
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
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
    pub fn animate_hover_pressed(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 1.0
            },
        );
    }
}
