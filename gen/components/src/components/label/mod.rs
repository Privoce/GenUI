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
    themes::Themes,
    utils::{get_font_family, set_cursor, ThemeColor}, widget_area,
};
use makepad_widgets::*;
use shader::draw_text::TextWrap;
live_design! {
    GLabelBase = {{GLabel}}{}
}

/// ## GLabel component
/// A component that displays text.
/// This component has no events. If you need add event on this component, you should wrap a card outside
#[derive(Live, Widget)]
pub struct GLabel {
    #[live]
    pub theme: Themes,
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
    pub draw_text: DrawText,
    #[walk]
    pub walk: Walk,
    #[live]
    pub align: Align,
    #[live]
    pub padding: Padding,
    #[live]
    pub text: ArcStringMut,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;

        let padding = self.padding;

        self.draw_text.draw_walk(
            cx,
            walk.with_add_padding(padding),
            self.align,
            self.text.as_ref(),
        );
        DrawStep::done()
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GLabel {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible{
            return;
        }

        let color = self.color.get(self.theme, 800);

        self.draw_text.apply_over(
            cx,
            live! {
                color: (color),
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
        set_cursor(cx, self.cursor.as_ref());
        self.draw_text.redraw(cx);
    }
}

impl GLabel {
    widget_area! {
        area, draw_text
    }
}
