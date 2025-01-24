use std::{fmt::Display, str::FromStr};

use super::{trans_color_rgb, Hex, Rgba};
use crate::Function;
use gen_utils::error::{Error, ParseError, ParseType};

/// 语法: `rgb(r, g, b)`
#[derive(Debug, Clone, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TryFrom<&Function> for Rgb {
    type Error = Error;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        // 检查是否fn的名称叫rgb
        if value.name.eq("rgb") {
            // 检查是否是三个参数，并且都是数字
            if let Some(params) = value.params.as_ref() {
                if params.len() == 3 {
                    // 将三个参数转换为数字且保证在0-255之间
                    let r = trans_color_rgb(&params[0].to_string())?;
                    let g = trans_color_rgb(&params[1].to_string())?;
                    let b = trans_color_rgb(&params[2].to_string())?;
                    return Ok(Rgb { r, g, b });
                }
            }

            let mut err = ParseError::new(&value.name, ParseType::Color("rgba".to_string()));
            let _ = err.set_other("rgb fn need three params `(r, g, b)`");
            return Err(err.into());
        }
        Err(ParseError::new(&value.name, ParseType::Color("rgb".to_string())).into())
    }
}

impl From<&Rgba> for Rgb {
    fn from(value: &Rgba) -> Self {
        Rgb {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<&Hex> for Rgb {
    fn from(value: &Hex) -> Self {
        let hex = value.0.trim_start_matches("#");
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Rgb { r, g, b }
    }
}

impl FromStr for Rgb {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 对于str来说如果要转为rgb需要先转hex
        let hex = Hex::from_str(s)?;
        Ok(Rgb::from(&hex))
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("rgb({}, {}, {})", self.r, self.g, self.b))
    }
}
