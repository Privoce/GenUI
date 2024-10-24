use std::{path::PathBuf, str::FromStr};

use makepad_widgets::{DVec2, LiveDependency, LiveId, Rect, Vec2, Vec4};

use crate::themes::{get_color, hex_to_vec4, Themes};

// -------------------------------------------------------------------------------------------------
/// This trait is used to get the color of the theme
pub trait ThemeColor {
    /// Get the color of the theme. if color is none, return the default color
    fn get(&self, theme: Themes, default: u32) -> Vec4;
    fn use_or(&self, hex: &str) -> Vec4;
}

impl ThemeColor for Option<Vec4> {
    fn get(&self, theme: Themes, default: u32) -> Vec4 {
        get_color(theme, self.as_ref(), default)
    }
    fn use_or(&self, hex: &str) -> Vec4 {
        self.unwrap_or_else(|| hex_to_vec4(hex))
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

            fn hex(target: u32) -> &'static str {
                match target {
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
                }
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

// ----------------------------------------------------------------------------------------------------
pub trait RectExpand {
    fn abs_start(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_start_center(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_end(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_end_center(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
}

impl RectExpand for Rect {
    fn abs_start(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, .. } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x;
            self.pos.y = pos.y + v.y;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y;
        };
    }

    fn abs_start_center(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x;
            self.pos.y = pos.y + v.y + size.y * 0.5 - self.size.y * 0.5;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y + size.y * 0.5 - self.size.y * 0.5;
        };
    }

    fn abs_end(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x + size.x;
            self.pos.y = pos.y + v.y;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y;
        };
    }

    fn abs_end_center(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x + size.x;
            self.pos.y = pos.y + v.y + size.y * 0.5 - self.size.y * 0.5;
        } else {
            self.pos.x = pos.x + size.x;
            self.pos.y = pos.y + size.y * 0.5 - self.size.y * 0.5;
        };
    }
}

// ------------------------------------------------------------------------------------------------------------

pub trait ToDVec {
    fn to_dvec2(self) -> DVec2;
}

impl ToDVec for Vec2 {
    fn to_dvec2(self) -> DVec2 {
        DVec2 {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

// ------------------------------------------------------------------------------------------------------------

pub trait LiveIdExp {
    fn as_slice(&self) -> &[LiveId];
}

impl LiveIdExp for LiveId {
    fn as_slice(&self) -> &[LiveId] {
        std::slice::from_ref(self)
    }
}
