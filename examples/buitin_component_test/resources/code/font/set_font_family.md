```rust
#[derive(Live, Widget)]
pub struct MyWidget {
    #[redraw]
    #[live]
    draw_text: DrawGText,
}

impl Widget for GLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // ...
        let font = get_font_family(&self.font_family, cx);
        self.draw_text.text_style.font = font;
        // ...
        DrawStep::done()
    }
}
```