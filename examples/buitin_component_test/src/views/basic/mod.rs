use makepad_widgets::Cx;

pub mod label;
pub mod view;
pub mod button;
pub mod svg;
pub mod icon;
pub mod image;
pub mod divider;
pub mod link;

pub fn register(cx: &mut Cx){
    // self::label::live_design(cx);
    // self::view::live_design(cx);
    // self::button::live_design(cx);
    // self::svg::live_design(cx);
    // self::icon::live_design(cx);
    // self::image::live_design(cx);
    // self::divider::live_design(cx);
    // self::link::live_design(cx);
    self::label::register(cx);
}