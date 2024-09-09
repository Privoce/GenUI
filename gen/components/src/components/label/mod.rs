//! # Label component
//! A component that displays text.
//! ## Props
//! - theme: Themes, default `Primary`
//! - color: text color
//! - font_size: font size, default `9.0`
//! - brightness: font brightness, default `1.0` (unused)
//! - curve: font curve, default `0.5` (unused)
//! - line_spacing: line spacing, default `1.5`
//! - top_drop: top drop, default `0.0`
//! - height_factor: height factor, default `0.0`
//! - wrap: text wrap, default `Word`
//! - font_family: font family
//! - visible: is current component visible, default `true`
//! - align: text align, default `Align{x: 0.0, y: 0.0}`
//! - padding: text padding, default `0.0`
//! - text: text content
//! ## Events
//! None
//! ## Example
//! See [label example](https://privoce.github.io/GenUI.github.io/gen/makepad/components/label.html)
mod register;

pub use register::register;

use crate::{
    themes::Themes,
    utils::{get_font_family, ThemeColor},
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
    draw_text: DrawText,
    #[walk]
    walk: Walk,
    #[live]
    align: Align,
    #[live]
    padding: Padding,
    #[live]
    text: ArcStringMut,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let font = get_font_family(&self.font_family, cx);

        self.draw_text.text_style.font = font;

        let mut padding = self.padding;
        padding.top += 2.0;
        self.draw_text.draw_walk(
            cx,
            walk.with_add_padding(padding),
            self.align,
            self.text.as_ref(),
        );

        DrawStep::done()
    }
    /// copy label text
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

impl LiveHook for GLabel {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
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
        self.draw_text.redraw(cx);
    }
}

impl GLabel {
    pub fn area(&self) -> Area {
        self.draw_text.area()
    }
}
