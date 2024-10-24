use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::svg::live_design(cx);
}