use makepad_widgets::Cx;

pub mod radio;
pub mod checkbox;

pub fn register(cx: &mut Cx){
    self::radio::register(cx);
    self::checkbox::register(cx);
}