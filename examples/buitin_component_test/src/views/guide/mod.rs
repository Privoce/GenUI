use makepad_widgets::Cx;

pub mod start;
pub mod install;
pub mod quickstart;

pub fn register(cx: &mut Cx){
    self::start::live_design(cx);
    self::install::live_design(cx);
    self::quickstart::live_design(cx);
}