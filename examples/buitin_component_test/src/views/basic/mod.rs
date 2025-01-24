use makepad_widgets::Cx;

pub mod label;
pub mod view;
pub mod button;
pub mod svg;
pub mod icon;
pub mod image;
pub mod divider;
pub mod link;
pub mod scroll;
pub mod shader;

pub fn register(cx: &mut Cx){
    self::svg::register(cx);
    self::label::register(cx);
    self::button::register(cx);
    self::view::register(cx);
    self::image::register(cx);
    self::icon::register(cx);
    self::divider::register(cx);
    self::link::register(cx);
    self::scroll::register(cx);
    self::shader::register(cx);
}