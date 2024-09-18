use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::window::live_design(cx);
}