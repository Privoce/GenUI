use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::shader::live_design(cx);
}