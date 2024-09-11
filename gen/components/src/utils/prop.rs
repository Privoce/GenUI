use std::{path::PathBuf, str::FromStr};

use makepad_widgets::{LiveDependency, Vec4};

use crate::themes::{get_color, Themes};

// -------------------------------------------------------------------------------------------------
/// This trait is used to get the color of the theme
pub trait ThemeColor {
    /// Get the color of the theme. if color is none, return the default color
    fn get(&self, theme: Themes, default: u32) -> Vec4;
}

impl ThemeColor for Option<Vec4> {
    fn get(&self, theme: Themes, default: u32) -> Vec4 {
        get_color(theme, self.as_ref(), default)
    }
}

#[macro_export]
macro_rules! color_v_trait {
    ($T:ty) => {
        impl ThemeColorValue for $T {
            fn v(target: u32) -> Vec4 {
                hex_to_vec4(match target {
                    25 => Self::_25,
                    50 => Self::_50,
                    100 => Self::_100,
                    200 => Self::_200,
                    300 => Self::_300,
                    400 => Self::_400,
                    500 => Self::_500,
                    600 => Self::_600,
                    700 => Self::_700,
                    800 => Self::_800,
                    900 => Self::_900,
                    _ => panic!("invalid target"),
                })
            }

            fn get(&self) -> Vec4 {
                self.0
            }
        }
    };
}
// -------------------------------------------------------------------------------------------------
/// This trait is used to transform f32/f64 to bool
pub trait ToBool {
    /// Transform f32/f64 to bool
    fn to_bool(&self) -> bool;
}

impl ToBool for f32 {
    fn to_bool(&self) -> bool {
        *self != 0.0
    }
}

pub trait BoolToF32 {
    /// Transform bool to f32/f64
    fn to_f32(&self) -> f32;
}

impl BoolToF32 for bool {
    fn to_f32(&self) -> f32 {
        *self as u8 as f32
    }
}

// --------------------------------------------------------------------------------------------------
pub trait Render {
    fn render(&mut self);
}

// ---------------------------------------------------------------------------------------------------
pub trait ToPath {
    fn to_pathbuf(&self) -> PathBuf;
}

impl ToPath for LiveDependency {
    fn to_pathbuf(&self) -> PathBuf {
        PathBuf::from_str(self.as_str()).unwrap()
    }
}
