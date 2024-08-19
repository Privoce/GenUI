use std::{fmt::Display, num::ParseFloatError};

use gen_utils::error::Errors;
use gen_parser::Value;

use crate::str_to_string_try_from;
#[derive(Debug, Clone, PartialEq)]
pub enum DAlign {
    X,
    Y,
    All,
}

#[derive(Debug, Clone, Default)]
pub struct Align {
    pub x: f64,
    pub y: f64,
}

impl Align {
    pub fn new(x: f64, y: f64) -> Align {
        Align { x, y }
    }
    pub fn single_x(x: f64) -> Align {
        Align::new(x, f64::default())
    }
    pub fn single_y(y: f64) -> Align {
        Align::new(f64::default(), y)
    }
}

/// # convert align to Makepad Align
/// ## single
/// - gen:     `align: 16`
/// - makepad: `align: {x: 16, y:16}`
/// ## multi
/// - gen:     `align:16 24`
/// - makepad: `align: {x: 16, y:24}`
impl TryFrom<(&str, DAlign)> for Align {
    type Error = Errors;

    fn try_from(value: (&str, DAlign)) -> Result<Self, Self::Error> {
        match value
            .0
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(aligns) => match aligns.len() {
                1 => match value.1 {
                    DAlign::X => Ok(Align::single_x(aligns[0])),
                    DAlign::Y => Ok(Align::single_y(aligns[0])),
                    DAlign::All => Ok(Align::new(aligns[0], aligns[0])),
                },
                2 => Ok(Align::new(aligns[0], aligns[1])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} cannot be converted to Makepad::Align!",
                    value.0
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::Align!",
                value.0
            ))),
        }
    }
}

impl TryFrom<(&String, DAlign)> for Align {
    type Error = Errors;

    fn try_from(value: (&String, DAlign)) -> Result<Self, Self::Error> {
        (value.0.as_str(), value.1).try_into()
    }
}

impl TryFrom<&str> for Align {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(aligns) => match aligns.len() {
                1 => Ok(Align::new(aligns[0], aligns[0])),
                2 => Ok(Align::new(aligns[0], aligns[1])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} cannot be converted to Makepad::Align!",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::Align!",
                value
            ))),
        }
    }
}

str_to_string_try_from!(Align);

impl TryFrom<&Value> for Align {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else {
            value
                .is_string_and_get()
                .map(|s| s.try_into())
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{} cannot be converted to Makepad::Align!",
                        value
                    )))
                })
        }
    }
}

impl Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{x: {}, y: {}}}", self.x, self.y))
    }
}
