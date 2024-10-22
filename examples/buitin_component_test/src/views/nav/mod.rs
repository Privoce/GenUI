use makepad_widgets::Cx;

pub mod window;
pub mod tool_btn;

pub fn register(cx: &mut Cx){
    self::tool_btn::register(cx);
    self::window::register(cx);
}