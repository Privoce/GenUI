use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::table::cell::live_design(cx);
    crate::components::table::row::live_design(cx);
    crate::components::table::body::live_design(cx);
    crate::components::table::header::live_design(cx);
    crate::components::table::live_design(cx);
    crate::components::table::virt::live_design(cx);
}