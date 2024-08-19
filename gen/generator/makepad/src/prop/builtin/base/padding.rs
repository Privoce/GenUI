use std::{fmt::Display, num::ParseFloatError};

use gen_utils::error::Errors;
use gen_parser::Value;

use crate::str_to_string_try_from;
#[derive(Debug, Clone, Default)]
pub struct Padding {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Padding {
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Padding {
            left,
            top,
            right,
            bottom,
        }
    }
    pub fn single(space: f64) -> Self {
        Padding::new(space, space, space, space)
    }
    pub fn multi_2(top_bottom: f64, left_right: f64) -> Self {
        Padding::new(top_bottom, left_right, top_bottom, left_right)
    }
    pub fn multi_4(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Padding::new(top, right, bottom, left)
    }
}

/// Convert padding to Makepad Padding
/// ## single
/// - gen:      `padding: 10`
/// - makepad:  `padding: 10`
/// ### multi 2
/// - gen:      `padding: 10 20`
/// - makepad:  `padding: {top: 10, right: 20, bottom: 10, left: 20}`
/// ### multi 4
/// - gen:      `padding: 10 20 0 29`
/// - makepad:  `padding: {top: 10, right: 20, bottom: 0, left: 29}`
impl TryFrom<&str> for Padding {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // try to split ` ` from str
        match value
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(spaces) => match spaces.len() {
                1 => Ok(Padding::single(spaces[0])),
                2 => Ok(Padding::multi_2(spaces[0], spaces[1])),
                4 => Ok(Padding::multi_4(spaces[0], spaces[1], spaces[2], spaces[3])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to padding",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to padding",
                value
            ))),
        }
    }
}

str_to_string_try_from! {Padding}

impl TryFrom<&Value> for Padding {
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
                        "{} cannot be converted to Padding!",
                        value
                    )))
                })
        }
    }
}

impl Display for Padding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{top: {}, right: {}, bottom: {}, left: {}}}",
            self.top, self.right, self.bottom, self.left
        ))
    }
}

#[cfg(test)]
mod test_padding {
    use super::*;
    #[test]
    fn to_tk() {
        let padding = Padding::try_from("10.0 20.0").unwrap();
        let tk = padding.to_string();
        let prop = "{top: 10, right: 20, bottom: 10, left: 20}";
        assert_eq!(tk.as_str(), prop);
    }
}
