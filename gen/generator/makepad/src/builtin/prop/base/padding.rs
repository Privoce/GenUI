use std::{num::ParseFloatError, str::FromStr};

use gen_parser::{Struct, Value};
use gen_utils::error::Error;

use crate::{
    builtin::prop::{convert_str_to_vec, err_from_to},
    struct_float_to_tokens, try_from_value_ref_struct,
};

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
#[derive(Debug, Clone, Default)]
pub struct Padding {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl From<&Padding> for toml_edit::Value {
    fn from(value: &Padding) -> Self {
        let mut table = toml_edit::InlineTable::new();
        
        table.insert("left", value.left.into());
        table.insert("top", value.top.into());
        table.insert("right", value.right.into());
        table.insert("bottom", value.bottom.into());

        toml_edit::Value::InlineTable(table)
    }
}

impl TryFrom<&toml_edit::Value> for Padding {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        let table = value
            .as_inline_table()
            .ok_or_else(|| Error::from("toml_edit::Item to Padding"))?;
        let left = table
            .get("left")
            .and_then(|v| v.as_float())
            .unwrap_or_default();
        let top = table
            .get("top")
            .and_then(|v| v.as_float())
            .unwrap_or_default();

        let right = table
            .get("right")
            .and_then(|v| v.as_float())
            .unwrap_or_default();

        let bottom = table
            .get("bottom")
            .and_then(|v| v.as_float())
            .unwrap_or_default();

        Ok(Padding {
            left,
            top,
            right,
            bottom,
        })
    }
}

impl TryFrom<Vec<f64>> for Padding {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(Padding {
                left: value[0],
                top: value[0],
                right: value[0],
                bottom: value[0],
            }),
            2 => Ok(Padding {
                left: value[1],
                top: value[0],
                right: value[1],
                bottom: value[0],
            }),
            4 => Ok(Padding {
                left: value[3],
                top: value[0],
                right: value[1],
                bottom: value[2],
            }),
            _ => Err(err_from_to(
                "Vec<f64>",
                "Makepad Padding, params number incorrect need 1 | 2 | 4",
            )
            .into()),
        }
    }
}

impl TryFrom<&Struct> for Padding {
    type Error = Error;

    fn try_from(value: &Struct) -> Result<Self, Self::Error> {
        let Struct {
            fields,
            is_anonymous,
            ..
        } = value;

        if *is_anonymous {
            let len = fields.len();
            if len == 0 || len > 4 {
                return Err(
                    err_from_to("Struct", "Vec4, params number incorrect need [1, 4]").into(),
                );
            }

            let mut l = None;
            let mut r = None;
            let mut t = None;
            let mut b = None;

            for (key, value) in fields {
                match key.as_str() {
                    "top" => t = Some(value.as_f64()?),
                    "left" => l = Some(value.as_f64()?),
                    "right" => r = Some(value.as_f64()?),
                    "bottom" => b = Some(value.as_f64()?),
                    _ => {}
                }
            }

            return Ok(Padding {
                left: l.unwrap_or_default(),
                right: r.unwrap_or_default(),
                top: t.unwrap_or_default(),
                bottom: b.unwrap_or_default(),
            });
        }

        Err(err_from_to("Struct", "Padding").into())
    }
}

impl TryFrom<f64> for Padding {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Padding {
            left: value,
            top: value,
            right: value,
            bottom: value,
        })
    }
}

try_from_value_ref_struct! {
    Padding, "Padding", f64
}

struct_float_to_tokens! {
    Padding{left, top, right, bottom}
}
