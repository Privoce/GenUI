use makepad_widgets::Cx;

pub mod radio;
pub mod checkbox;
pub mod toggle;
pub mod progress;
pub mod upload;
pub mod input;
pub mod select;

pub fn register(cx: &mut Cx){
    self::radio::register(cx);
    self::checkbox::register(cx);
    self::toggle::register(cx);
    self::progress::register(cx);
    self::upload::register(cx);
    self::input::register(cx);
}