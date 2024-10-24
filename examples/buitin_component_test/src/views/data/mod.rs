use makepad_widgets::Cx;

pub mod loading;
pub mod tag;
pub mod splitter;
pub mod collapse;

pub fn register(cx: &mut Cx){
    self::loading::register(cx);
    self::tag::register(cx);
    self::splitter::register(cx);
    self::collapse::register(cx);
}