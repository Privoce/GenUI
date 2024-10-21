use makepad_widgets::Cx;

pub mod loading;
pub mod tag;

pub fn register(cx: &mut Cx){
    self::loading::register(cx);
    self::tag::register(cx);
}