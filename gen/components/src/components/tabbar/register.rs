use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::tabbar::live_design(cx);
    crate::components::tabbar::item::live_design(cx);
}