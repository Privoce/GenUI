use makepad_widgets::Cx;

pub mod components;
pub mod themes;
pub mod macros;
pub mod shader;
pub mod utils;

pub fn live_design(cx: &mut Cx) {
    // components ---------------------------------------------
    crate::components::label::live_design(cx);
    crate::components::button::live_design(cx);
    crate::components::card::live_design(cx);
    crate::components::link::live_design(cx);
    crate::components::icon::live_design(cx);
    crate::components::radio::live_design(cx);
    crate::components::checkbox::live_design(cx);
    crate::components::image::live_design(cx);
    crate::components::input::live_design(cx);
    crate::components::divider::live_design(cx);
    crate::components::shader::live_design(cx);
    crate::components::popup::live_design(cx);
    crate::components::drop_down::live_design(cx);
    crate::components::toggle::live_design(cx);
    crate::components::progress::live_design(cx);
    crate::components::loading::live_design(cx);
    crate::components::badge::live_design(cx);
    // shader -------------------------------------------------
    crate::shader::draw_button::live_design(cx);
    crate::shader::draw_card::live_design(cx);
    crate::shader::draw_link::live_design(cx);
    crate::shader::draw_text::live_design(cx);
    crate::shader::draw_radio::live_design(cx);
    crate::shader::draw_check_box::live_design(cx);
    crate::shader::draw_icon::live_design(cx);
    crate::shader::draw_divider::live_design(cx);
    crate::shader::draw_toggle::live_design(cx);
    crate::shader::draw_progress::live_design(cx);
    crate::shader::draw_loading::live_design(cx);
    crate::shader::draw_icon_pixel::live_design(cx);
    // export all the components
    crate::components::live_design(cx);
}