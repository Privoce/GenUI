use makepad_widgets::Cx;

pub mod window;
pub mod tool_btn;
pub mod tabbar;
pub mod menu;
pub mod breadcrumb;
pub mod router;

pub fn register(cx: &mut Cx){
    self::tool_btn::register(cx);
    self::window::register(cx);
    self::tabbar::register(cx);
    self::menu::register(cx);
    self::breadcrumb::register(cx);
    self::router::register(cx);
    
}