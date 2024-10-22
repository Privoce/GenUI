use std::fmt::Display;

use makepad_widgets::*;

use crate::{shader::icon_lib::types::base::Base, themes::hex_to_vec4};
#[derive(Live, LiveHook, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum GOsType {
    Windows,
    Mac,
    #[pick]
    Linux,
    Other,
}

impl From<GOsType> for OsType {
    fn from(value: GOsType) -> Self {
        match value {
            GOsType::Windows => OsType::Windows,
            GOsType::Mac => OsType::Macos,
            GOsType::Linux => OsType::LinuxDirect,
            GOsType::Other => OsType::Unknown,
        }
    }
}

impl From<OsType> for GOsType {
    fn from(value: OsType) -> Self {
        match value {
            OsType::Unknown => GOsType::Other,
            OsType::Windows => GOsType::Windows,
            OsType::Macos => GOsType::Mac,
            OsType::Ios => GOsType::Other,
            OsType::Android(_android_params) => GOsType::Other,
            OsType::OpenHarmony(_open_harmony_params) => GOsType::Other,
            OsType::LinuxWindow(_linux_window_params) => GOsType::Other,
            OsType::LinuxDirect => GOsType::Linux,
            OsType::Web(_web_params) => GOsType::Other,
        }
    }
}

impl GOsType {
    pub fn get() -> GOsType {
        let os = std::env::consts::OS;
        match os {
            "windows" => GOsType::Windows,
            "macos" => GOsType::Mac,
            "linux" => GOsType::Linux,
            _ => GOsType::Other,
        }
    }
    pub fn bg_color(&self, btn_type: GToolButtonType) -> Vec4 {
        match self {
            GOsType::Windows | GOsType::Other => match btn_type {
                GToolButtonType::Min
                | GToolButtonType::Max
                | GToolButtonType::FullScreen
                | GToolButtonType::Close => vec4(0.0, 0.0, 0.0, 0.0),
            },
            GOsType::Mac | GOsType::Linux => match btn_type {
                GToolButtonType::Min => hex_to_vec4("#FDC12F"),
                GToolButtonType::Max => hex_to_vec4("#21CE36"),
                GToolButtonType::FullScreen => hex_to_vec4("#D6D0D9"),
                GToolButtonType::Close => hex_to_vec4("#F1615B"),
            },
        }
    }
    /// get the hover color
    /// in linux and mac, color should be light than bg color
    pub fn hover_color(&self, btn_type: GToolButtonType) -> Vec4 {
        match self {
            GOsType::Windows | GOsType::Other => match btn_type {
                GToolButtonType::Min | GToolButtonType::Max | GToolButtonType::FullScreen => {
                    hex_to_vec4("#3F444A")
                }
                GToolButtonType::Close => hex_to_vec4("#E81123"),
            },
            GOsType::Mac | GOsType::Linux => match btn_type {
                GToolButtonType::Min => hex_to_vec4("#fdbc21"),
                GToolButtonType::Max => hex_to_vec4("#17d82e"),
                GToolButtonType::FullScreen => hex_to_vec4("#cbc5ce"),
                GToolButtonType::Close => hex_to_vec4("#ec4f48"),
            },
        }
    }
    pub fn focus_color(&self, btn_type: GToolButtonType) -> Vec4 {
        match self {
            GOsType::Windows | GOsType::Other => match btn_type {
                GToolButtonType::Min | GToolButtonType::Max | GToolButtonType::FullScreen => {
                    hex_to_vec4("#5B5F64")
                }
                GToolButtonType::Close => hex_to_vec4("#981A27"),
            },
            GOsType::Mac | GOsType::Linux => match btn_type {
                GToolButtonType::Min => hex_to_vec4("#fdbc21"),
                GToolButtonType::Max => hex_to_vec4("#17d82e"),
                GToolButtonType::FullScreen => hex_to_vec4("#cbc5ce"),
                GToolButtonType::Close => hex_to_vec4("#ec4f48"),
            },
        }
    }
    /// get the border color
    /// in linux and mac, color should be focus color
    /// in windows, color should be no border and border color is transparent
    pub fn border_color_width(&self, btn_type: GToolButtonType) -> (Vec4, f32) {
        match self {
            GOsType::Windows | GOsType::Other => match btn_type {
                GToolButtonType::Min
                | GToolButtonType::Max
                | GToolButtonType::FullScreen
                | GToolButtonType::Close => (vec4(0.0, 0.0, 0.0, 0.0), 0.0),
            },
            GOsType::Mac | GOsType::Linux => match btn_type {
                GToolButtonType::Min => (hex_to_vec4("#fdbc21"), 0.0),
                GToolButtonType::Max => (hex_to_vec4("#17d82e"), 0.0),
                GToolButtonType::FullScreen => (hex_to_vec4("#cbc5ce"), 0.0),
                GToolButtonType::Close => (hex_to_vec4("#ec4f48"), 0.0),
            },
        }
    }
}

#[derive(Live, LiveHook, Clone, Copy, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum GToolButtonType {
    #[pick]
    Min = shader_enum(1),
    Max = shader_enum(2),
    FullScreen = shader_enum(3),
    Close = shader_enum(4),
}

impl From<GToolButtonType> for Base {
    fn from(value: GToolButtonType) -> Self {
        match value {
            GToolButtonType::Min => Base::Min,
            GToolButtonType::Max => Base::Max,
            GToolButtonType::FullScreen => Base::FullScreen,
            GToolButtonType::Close => Base::Close,
        }
    }
}

impl Display for GToolButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GToolButtonType::Min => "Min",
            GToolButtonType::Max => "Max",
            GToolButtonType::FullScreen => "FullScreen",
            GToolButtonType::Close => "Close",
        })
    }
}
