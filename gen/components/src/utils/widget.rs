use makepad_widgets::{font_atlas::CxFontsAtlasRc, Cx, Cx2d, Font, LiveDependency, MouseCursor};

pub fn get_font_family(font_family: &LiveDependency, cx: &mut Cx2d) -> Font {
    let font_family = font_family.clone();

    let atlas = cx.get_global::<CxFontsAtlasRc>().clone();
    let font_id = Some(
        atlas
            .0
            .borrow_mut()
            .get_font_by_path(cx, font_family.as_str()),
    );
    let font = Font {
        font_id,
        path: font_family,
    };
    font
}

pub fn set_cursor(cx: &mut Cx, cursor: Option<&MouseCursor>) -> () {
    if let Some(cursor) = cursor {
        cx.set_cursor(*cursor);
    } else {
        cx.set_cursor(MouseCursor::default());
    }
}

/// This macro generates the following functions: `text`, `set_text`, `set_text_and_visible`, `is_visible` in Widget trait
#[macro_export]
macro_rules! set_text_and_visible_fn {
    () => {
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
    };
}