use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::menu::menu_item::live_design(cx);
}