use makepad_widgets::{
    font_atlas::CxFontsAtlasRc, Cx, Cx2d, DVec2, Font, LiveDependency, MouseCursor, Rect
};

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

pub trait RectExp {
    /// judget another area is in this area, which usually used in event handle
    fn is_in(&self, rect: &Rect) -> bool;
    /// judget a point is in this area, which usually used in event handle
    fn is_in_pos(&self, pos: &DVec2) -> bool;
}

impl RectExp for Rect {
    fn is_in(&self, rect: &Rect) -> bool {
        // get size and pos to judge
        let self_size = self.size;
        let self_pos = self.pos;
        let rect_size = rect.size;
        let rect_pos = rect.pos;
        if rect_pos.x >= self_pos.x
            && rect_pos.y >= self_pos.y
            && rect_pos.x + rect_size.x <= self_pos.x + self_size.x
            && rect_pos.y + rect_size.y <= self_pos.y + self_size.y
        {
            return true;
        }
        false
    }
    fn is_in_pos(&self, pos: &DVec2) -> bool {
        let self_pos = self.pos;
        let self_size = self.size;
        if pos.x >= self_pos.x
            && pos.y >= self_pos.y
            && pos.x <= self_pos.x + self_size.x
            && pos.y <= self_pos.y + self_size.y
        {
            return true;
        }
        false
    }
}
