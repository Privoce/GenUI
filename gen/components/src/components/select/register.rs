use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::select::live_design(cx);
    crate::components::select::item::live_design(cx);
    crate::components::select::options::live_design(cx);
}
