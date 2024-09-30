use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::router::live_design(cx);
    crate::components::router::page::live_design(cx);
}