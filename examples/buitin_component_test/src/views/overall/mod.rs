use makepad_widgets::Cx;

pub mod all;
pub mod color;
pub mod font;

pub fn register(cx: &mut Cx){
    self::all::live_design(cx);
    self::color::live_design(cx);
    self::font::live_design(cx);
}