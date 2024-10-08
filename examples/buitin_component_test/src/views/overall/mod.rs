use makepad_widgets::Cx;

pub mod all;
pub mod color;

pub fn register(cx: &mut Cx){
    self::all::live_design(cx);
    self::color::live_design(cx);
}