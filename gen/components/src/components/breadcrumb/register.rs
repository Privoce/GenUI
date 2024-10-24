use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::breadcrumb::item::live_design(cx);
    crate::components::breadcrumb::live_design(cx);
}