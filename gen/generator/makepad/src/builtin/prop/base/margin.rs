use std::{num::ParseFloatError, str::FromStr};

use gen_analyzer::value::{Struct, Value};
use gen_utils::{err_from_to, error::Error};

use crate::{builtin::prop::convert_str_to_vec, struct_float_to_tokens, try_from_value_ref_struct};

#[derive(Debug, Clone, Default)]
pub struct Margin {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl From<&Margin> for toml_edit::Value {
    fn from(value: &Margin) -> Self {
        let mut table = toml_edit::InlineTable::new();

        table.insert("left", value.left.into());
        table.insert("top", value.top.into());
        table.insert("right", value.right.into());
        table.insert("bottom", value.bottom.into());

        toml_edit::Value::InlineTable(table)
    }
}

impl TryFrom<&toml_edit::Value> for Margin {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        let table = value
            .as_inline_table()
            .ok_or_else(|| Error::from("toml_edit::Item to Margin"))?;
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

        Ok(Margin {
            left,
            top,
            right,
            bottom,
        })
    }
}

impl TryFrom<Vec<f64>> for Margin {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(Margin {
                left: value[0],
                top: value[0],
                right: value[0],
                bottom: value[0],
            }),
            2 => Ok(Margin {
                left: value[1],
                top: value[0],
                right: value[1],
                bottom: value[0],
            }),
            4 => Ok(Margin {
                left: value[3],
                top: value[0],
                right: value[1],
                bottom: value[2],
            }),
            _ => Err(err_from_to!(
                "Vec<f64>" =>"Makepad Margin, params number incorrect need 1 | 2 | 4"
            )),
        }
    }
}

impl TryFrom<&Struct> for Margin {
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
                return Err(err_from_to!("Struct" => "Vec4, params number incorrect need [1, 4]"));
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

            return Ok(Margin {
                left: l.unwrap_or_default(),
                right: r.unwrap_or_default(),
                top: t.unwrap_or_default(),
                bottom: b.unwrap_or_default(),
            });
        }

        Err(err_from_to!("Struct" => "Margin"))
    }
}

impl TryFrom<f64> for Margin {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Margin {
            left: value,
            top: value,
            right: value,
            bottom: value,
        })
    }
}

try_from_value_ref_struct! {
    Margin, "Margin", f64
}

struct_float_to_tokens! {
    Margin {left, top, right, bottom}
}
