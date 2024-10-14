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
