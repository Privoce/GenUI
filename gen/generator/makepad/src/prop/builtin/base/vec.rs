use std::fmt::Display;

use std::num::ParseFloatError;

use gen_utils::error::Errors;
use gen_parser::Value;

use crate::str_to_string_try_from;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }
    pub fn single(f: f32) -> Vec4 {
        Vec4::new(f, f, f, f)
    }
    pub fn rgb(r: f32, g: f32, b: f32) -> Vec4 {
        Vec4::new(r, g, b, 1.0)
    }
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Vec4 {
        Vec4::new(r, g, b, a)
    }
}


impl TryFrom<&str> for Vec4 {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
            .split_whitespace()
            .map(|x| x.parse::<f32>())
            .collect::<Result<Vec<f32>, ParseFloatError>>()
        {
            Ok(spaces) => match spaces.len() {
                1 => Ok(Vec4::single(spaces[0])),
                3 => Ok(Vec4::rgb(spaces[0], spaces[1], spaces[2])),
                4 => Ok(Vec4::rgba(spaces[0], spaces[1], spaces[2], spaces[3])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to Vec4",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to Vec4",
                value
            ))),
        }
    }
}

str_to_string_try_from!(Vec4);

impl TryFrom<f32> for Vec4 {
    type Error = Errors;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Vec4::single(value))
    }
}

impl TryFrom<f64> for Vec4 {
    type Error = Errors;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Vec4::single(value as f32))
    }
}

impl TryFrom<&Value> for Vec4 {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else if let Some(d) = value.is_double_and_get() {
            Ok(Vec4::single(d as f32))
        } else if let Some(d) = value.is_float_and_get() {
            Ok(Vec4::single(d))
        } else {
            value
                .is_string_and_get()
                .map(|s| s.try_into())
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{} can not convert to Vec4",
                        value
                    )))
                })
        }
    }
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{x: {}, y: {}, z: {}, w: {}}}",
            self.x, self.y, self.z, self.w
        ))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2(DVec2);

impl TryFrom<&str> for Vec2 {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DVec2::try_from(value).map(|d| Vec2(d))
    }
}

str_to_string_try_from!(Vec2);

impl TryFrom<&Value> for Vec2 {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        DVec2::try_from(value).map(|d| Vec2(d))
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("vec2({}, {})", self.0.x, self.0.y))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}

impl DVec2 {
    pub fn new(x: f64, y: f64) -> DVec2 {
        DVec2 { x, y }
    }
    pub fn single(f: f64) -> DVec2 {
        DVec2::new(f, f)
    }
}

/// Convert to Makepad Walk abs_pos
/// ## single
/// - gen:      `absolute_position: 12;`
/// - makepad:  `abs_pos: vec2(12, 12)`
/// ## multi
/// - gen:      `absolute_position: 12 20;`
/// - makepad:  `abs_pos: vec2(12, 20)`
impl TryFrom<&str> for DVec2 {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
            .split_whitespace()
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(spaces) => match spaces.len() {
                1 => Ok(DVec2::single(spaces[0])),
                2 => Ok(DVec2::new(spaces[0], spaces[1])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to DVec2",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to DVec2",
                value
            ))),
        }
    }
}

str_to_string_try_from! {DVec2}

impl TryFrom<&Value> for DVec2 {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else {
            Err(Errors::PropConvertFail(format!(
                "{} can not convert to DVec2",
                value
            )))
        }
    }
}

impl Display for DVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{x: {}, y: {}}}", self.x, self.y))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y , z}
    }
    pub fn single(f: f64) -> Vec3 {
        Vec3::new(f, f, f)
    }
}


impl TryFrom<&str> for Vec3 {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
            .split_whitespace()
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(spaces) => match spaces.len() {
                1 => Ok(Vec3::single(spaces[0])),
                3 => Ok(Vec3::new(spaces[0], spaces[1], spaces[2])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to Vec3",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to Vec3",
                value
            ))),
        }
    }
}

str_to_string_try_from! {Vec3}

impl TryFrom<&Value> for Vec3 {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else {
            Err(Errors::PropConvertFail(format!(
                "{} can not convert to Vec3",
                value
            )))
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{x: {}, y: {}, z: {}}}", self.x, self.y, self.z))
    }
}
