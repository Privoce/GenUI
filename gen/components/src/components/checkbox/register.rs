use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::checkbox::live_design(cx);
    crate::components::checkbox::group::live_design(cx);
}