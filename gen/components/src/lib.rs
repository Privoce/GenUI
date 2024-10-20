use makepad_widgets::Cx;

pub mod components;
pub mod shader;
pub mod themes;
pub mod utils;
pub mod event;
pub mod error;

pub fn live_design(cx: &mut Cx) {
    // components ---------------------------------------------
    crate::components::colors::register(cx);
    crate::components::label::register(cx);
    crate::components::link::register(cx);
    crate::components::view::register(cx);
    crate::components::svg::register(cx);
    crate::components::icon::register(cx);
    crate::components::button::register(cx);
    crate::components::tag::register(cx);
    crate::components::image::register(cx);
    crate::components::breadcrumb::register(cx);
    crate::components::toggle::register(cx);
    crate::components::radio::register(cx);
    crate::components::checkbox::register(cx);
    crate::components::loading::register(cx);
    crate::components::divider::register(cx);
    crate::components::file_upload::register(cx);
    crate::components::progress::register(cx);
    crate::components::collapse::register(cx);
    crate::components::shader::register(cx);
    crate::components::input::register(cx);
    crate::components::popup::register(cx);
    crate::components::drop_down::register(cx);
    crate::components::table::register(cx);
    crate::components::tool_btn::register(cx);
    crate::components::window::register(cx);
    crate::components::select::register(cx);
    crate::components::tabbar::register(cx);
    crate::components::router::register(cx);
    crate::components::menu::register(cx);


    crate::components::tab::header::live_design(cx);
    crate::components::tab::button::live_design(cx);
    crate::components::tab::body::live_design(cx);
    crate::components::tab::pane::live_design(cx);
    crate::components::tab::live_design(cx);
    
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
    crate::shader::draw_view::live_design(cx);
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
    crate::shader::draw_shader::live_design(cx);
    // export all the components
    crate::components::live_design(cx);
}
