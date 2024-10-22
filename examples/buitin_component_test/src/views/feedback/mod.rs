use makepad_widgets::Cx;
pub mod state;
pub mod popup;
pub mod tool_tip;
pub mod dialog;
pub mod drawer;

pub fn register(cx: &mut Cx){
    self::state::register(cx);
    self::popup::register(cx);
    self::tool_tip::register(cx);
    self::dialog::register(cx);
    self::drawer::register(cx);
}