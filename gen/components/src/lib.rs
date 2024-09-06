use makepad_widgets::Cx;

pub mod components;
pub mod macros;
pub mod shader;
pub mod themes;
pub mod utils;

pub fn live_design(cx: &mut Cx) {
    // components ---------------------------------------------
    crate::components::label::live_design(cx);
    crate::components::button::live_design(cx);
    crate::components::card::live_design(cx);
    crate::components::link::live_design(cx);
    crate::components::svg::live_design(cx);
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
    crate::components::breadcrumb::live_design(cx);
    crate::components::breadcrumb::item::live_design(cx);
    crate::components::tab::header::live_design(cx);
    crate::components::tab::button::live_design(cx);
    crate::components::tab::body::live_design(cx);
    crate::components::tab::pane::live_design(cx);
    crate::components::tab::live_design(cx);
    crate::components::file_upload::live_design(cx);
    crate::components::collapse::live_design(cx);
    // shader -------------------------------------------------
    crate::shader::icon_lib::live_design(cx);
    crate::shader::icon_lib::base::live_design(cx);
    crate::shader::icon_lib::arrow::live_design(cx);
    crate::shader::icon_lib::code::live_design(cx);
    crate::shader::icon_lib::emoji::live_design(cx);
    crate::shader::icon_lib::fs::live_design(cx);
    crate::shader::icon_lib::person::live_design(cx);
    crate::shader::icon_lib::relation::live_design(cx);
    crate::shader::icon_lib::state::live_design(cx);
    crate::shader::icon_lib::time::live_design(cx);
    crate::shader::icon_lib::tool::live_design(cx);
    crate::shader::icon_lib::ui::live_design(cx);
    crate::shader::draw_button::live_design(cx);
    crate::shader::draw_card::live_design(cx);
    crate::shader::draw_link::live_design(cx);
    crate::shader::draw_text::live_design(cx);
    crate::shader::draw_radio::live_design(cx);
    crate::shader::draw_check_box::live_design(cx);
    crate::shader::draw_svg::live_design(cx);
    crate::shader::draw_divider::live_design(cx);
    crate::shader::draw_toggle::live_design(cx);
    crate::shader::draw_progress::live_design(cx);
    crate::shader::draw_loading::live_design(cx);
    crate::shader::draw_icon_pixel::live_design(cx);
    crate::shader::draw_split::live_design(cx);
    crate::shader::draw_tab::live_design(cx);
    crate::shader::draw_tab_pane::live_design(cx);
    crate::shader::draw_popup::live_design(cx);
    // export all the components
    crate::components::live_design(cx);
}
