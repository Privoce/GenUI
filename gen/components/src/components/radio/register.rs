use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::radio::live_design(cx);
    crate::components::radio::group::live_design(cx);
}