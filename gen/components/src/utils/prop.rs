use makepad_widgets::Vec4;

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
