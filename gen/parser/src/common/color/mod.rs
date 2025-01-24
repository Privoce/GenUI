mod hex;
mod linear;
mod percentage;
mod radial;
mod rgb;
mod rgba;

use std::{fmt::Display, str::FromStr};

pub use hex::*;
pub use linear::*;
pub use percentage::*;
pub use radial::*;
pub use rgb::*;
pub use rgba::*;

use crate::{Function, Value};
use gen_utils::error::{ConvertError, Error, ParseError, ParseType};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    sequence::{preceded, tuple},
    IResult,
};

use super::MakepadShader;

/// ## GenUI 内置颜色类型
/// 颜色写法参考: https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
/// - 16进制颜色: #3, #333, #333333
/// - rgb(r, g, b)
/// - rgba(r, g, b, a)
/// - linear_gradient(angle, color percentage, color percentage, ...)
/// - radial_gradient(color percentage, color percentage, ...)
#[derive(Debug, Clone)]
pub enum BuiltinColor {
    /// 16进制颜色
    Hex(Hex),
    /// rgb(r, g, b)
    Rgb(Rgb),
    /// rgba(r, g, b, a)
    Rgba(Rgba),
    /// 线性渐变
    LinearGradient(LinearGradient),
    /// 径向渐变
    RadialGradient(RadialGradient),
    /// Shader for Makepad
    #[cfg(feature = "makepad")]
    Shader(MakepadShader),
}

impl Display for BuiltinColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinColor::Hex(h) => write!(f, "{}", h),
            BuiltinColor::Rgb(rgb) => write!(f, "{}", rgb),
            BuiltinColor::Rgba(rgba) => write!(f, "{}", rgba),
            BuiltinColor::LinearGradient(linear) => write!(f, "{}", linear),
            BuiltinColor::RadialGradient(radial) => write!(f, "{}", radial),
            BuiltinColor::Shader(shader) => write!(f, "{}", shader),
        }
    }
}

impl TryFrom<&Value> for BuiltinColor {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::UnKnown(s) => BuiltinColor::from_str(s.as_str()),
            Value::Function(f) => BuiltinColor::try_from(f),
            Value::String(s) => BuiltinColor::from_str(s),
            _ => Err(ConvertError::FromTo {
                from: value.to_string(),
                to: "Builtin Color Type".to_string(),
            }
            .into()),
        }
    }
}

impl FromStr for BuiltinColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 只需要解析16进制颜色
        Hex::from_str(s).map(BuiltinColor::Hex)
    }
}

impl TryFrom<&Function> for BuiltinColor {
    type Error = Error;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        match value.name.as_str() {
            "rgb" => Rgb::try_from(value).map(BuiltinColor::Rgb),
            "rgba" => Rgba::try_from(value).map(BuiltinColor::Rgba),
            "linear_gradient" => LinearGradient::try_from(value).map(BuiltinColor::LinearGradient),
            "radial_gradient" => RadialGradient::try_from(value).map(BuiltinColor::RadialGradient),
            "shader" => MakepadShader::try_from(value).map(BuiltinColor::Shader),
            _ => Err(ConvertError::FromTo {
                from: value.name.to_string(),
                to: "Builtin Color Type".to_string(),
            }
            .into()),
        }
    }
}

/// parse single hex color
fn hex_digit(input: &str) -> IResult<&str, &str> {
    recognize(one_of("0123456789abcdefABCDEF"))(input)
}

/// parse 3 hex color
fn three_hex_digits(input: &str) -> IResult<&str, String> {
    map_res(tuple((hex_digit, hex_digit, hex_digit)), |(a, b, c)| {
        format!("{}{}{}{}{}{}FF", a, a, b, b, c, c).parse()
    })(input)

    // recognize(tuple((
    //     hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    // )))(input)
}

/// Parse 6 hex color
fn six_hex_digits(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    )))(input)
}

fn eight_hex_digits(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    )))(input)
}

/// parse hex color
/// - #3       : single
/// - #333     : third
/// - #333333  : sixth
/// - #33333333: eighth
pub fn parse_hex_color(input: &str) -> IResult<&str, String> {
    preceded(
        tag("#"),
        alt((
            map_res(eight_hex_digits, |s: &str| s.parse()),
            map_res(six_hex_digits, |s: &str| format!("{}FF", s).parse()),
            three_hex_digits,
            map_res(hex_digit, |s| format!("{}FF", s.repeat(6)).parse()),
        )),
    )(input)
}

/// 将三个参数转换为数字且保证在0-255之间
pub fn trans_color_rgb(v: &str) -> Result<u8, Error> {
    match v.parse::<u8>() {
        Ok(val) => {
            if val.ge(&0) && val.le(&255) {
                Ok(val)
            } else {
                let mut err =
                    ParseError::new(&val.to_string(), ParseType::Color("rgb".to_string()));
                let _ = err.set_other("value must between 0-255");
                Err(err.into())
            }
        }
        Err(_) => {
            let mut err = ParseError::new(v, ParseType::Color("rgb".to_string()));
            let _ = err.set_other("value must be number");
            return Err(err.into());
        }
    }
}

pub fn trans_opacity(v: &str) -> Result<f32, Error> {
    match v.parse::<f32>() {
        Ok(val) => {
            if val.ge(&0.0) && val.le(&1.0) {
                Ok(val)
            } else {
                let mut err =
                    ParseError::new(&val.to_string(), ParseType::Color("rgba".to_string()));
                let _ = err.set_other("value must between 0-1");
                Err(err.into())
            }
        }
        Err(_) => {
            let mut err = ParseError::new(v, ParseType::Color("rgba".to_string()));
            let _ = err.set_other("value must be number");
            return Err(err.into());
        }
    }
}