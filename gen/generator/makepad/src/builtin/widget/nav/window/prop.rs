use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;
use gen_utils::error::Error;
use toml_edit::Item;

use crate::builtin::prop::{
    value_bool, DVec2, Flow, GOsType, Layout, Prop, Size, Walk, WindowSize,
};
use crate::builtin::widget::basic::ViewProps;
use crate::{from_gen_props, props_to_tokens};

// ------------ wait to implement ------------------------------------
// pub window: WindowHandle,
// pub pass: Pass,
// pub cursor_draw_list: DrawList2d,
// pub draw_cursor: DrawQuad,
// pub nav_control: NavControl,
// -------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum Props {
    OsType(GOsType),
    DerefWidget(ViewProps),
    ShowTitle(bool),
    ShowIcon(bool),
    LastMousePos(DVec2),
    MouseCursorSize(DVec2),
    HideCaptionOnFullscreen(bool),
    EventKey(bool),
    WindowSize(WindowSize),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        let name = value.0.name.to_string();
        match name.as_str() {
            "os_type" => Ok(Props::OsType(GOsType::try_from(&value.1)?)),
            "show_title" => Ok(Props::ShowTitle(value.1.as_bool()?)),
            "show_icon" => Ok(Props::ShowIcon(value.1.as_bool()?)),
            "last_mouse_pos" => Ok(Props::LastMousePos(DVec2::try_from(&value.1)?)),
            "mouse_cursor_size" => Ok(Props::MouseCursorSize(DVec2::try_from(&value.1)?)),
            "hide_caption_on_fullscreen" => Ok(Props::HideCaptionOnFullscreen(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            _ => {
                // handle deref widget
                if let Ok(p) = ViewProps::try_from(value) {
                    return Ok(Props::DerefWidget(p));
                } else {
                    return Err(err_from_to!(
                        "GenUI Props" => &format!("Makepad Window Prop, Invalid Prop: {}", name)
                    ).to_runtime("Makepad Compiler"));
                }
            }
        }
    }
}

pub fn default_window_props() -> Prop<Props> {
    let mut window = Prop::default();

    window.push(Props::DerefWidget(ViewProps::Walk(Walk::Height(
        Size::Fill,
    ))));

    window.push(Props::DerefWidget(ViewProps::Walk(Walk::Width(Size::Fill))));

    window.push(Props::DerefWidget(ViewProps::Layout(Layout::Flow(
        Flow::Down,
    ))));

    window.push(Props::WindowSize(WindowSize::default()));

    window
}

impl TryFrom<&Item> for Prop<Props> {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        if let Some(table) = value.as_inline_table() {
            let mut res = Prop::default();
            for (key, value) in table.iter() {
                match key {
                    "os_type" => {
                        res.push(Props::OsType(value.try_into()?));
                    }
                    "show_title" => {
                        let _ = res.push(Props::ShowTitle(value_bool(value)?));
                    }
                    "show_icon" => {
                        let _ = res.push(Props::ShowIcon(value_bool(value)?));
                    }
                    "last_mouse_pos" => {
                        let _ = res.push(Props::LastMousePos(DVec2::try_from(value)?));
                    }
                    "mouse_cursor_size" => {
                        let _ = res.push(Props::MouseCursorSize(DVec2::try_from(value)?));
                    }
                    "hide_caption_on_fullscreen" => {
                        let _ = res.push(Props::HideCaptionOnFullscreen(value_bool(value)?));
                    }
                    "event_key" => {
                        let _ = res.push(Props::EventKey(value_bool(value)?));
                    }
                    "window_size" => {
                        let _ = res.push(Props::WindowSize(value.try_into()?));
                    }
                    _ => {
                        // handle deref widget
                        if let Ok(p) = ViewProps::try_from((key, value)) {
                            let _ = res.push(Props::DerefWidget(p));
                        } else {
                            return Err(err_from_to!("toml::Item" => "WindowProps"));
                        }
                    }
                }
            }
            return Ok(res);
        }

        Err(err_from_to!("toml::Item" => "WindowProps"))
    }
}

props_to_tokens! {
    Props,
    Props::OsType => os_type, false,
    Props::DerefWidget => deref_widget, true,
    Props::ShowTitle => show_title, false,
    Props::ShowIcon => show_icon, false,
    Props::LastMousePos => last_mouse_pos, false,
    Props::MouseCursorSize => mouse_cursor_size, false,
    Props::HideCaptionOnFullscreen => hide_caption_on_fullscreen, false,
    Props::EventKey => event_key, false,
    Props::WindowSize => window, false
}
