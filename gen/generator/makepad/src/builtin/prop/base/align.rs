use crate::{builtin::prop::convert_str_to_vec, struct_float_to_tokens, try_from_value_ref_struct};
use gen_analyzer::value::{Struct, Value};
use gen_utils::{err_from_to, error::Error};
use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug, Clone, Default)]
pub struct Align {
    pub x: f64,
    pub y: f64,
}

impl From<&Align> for toml_edit::Value {
    fn from(value: &Align) -> Self {
        let mut table = toml_edit::InlineTable::new();
        table.insert("x", value.x.into());
        table.insert("y", value.y.into());
        toml_edit::Value::InlineTable(table)
    }
}

impl TryFrom<&toml_edit::Value> for Align {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        let table = value
            .as_inline_table()
            .ok_or_else(|| Error::from("toml_edit::Item to Align"))?;
        let x = table
            .get("x")
            .and_then(|v| v.as_float())
            .unwrap_or_default();
        let y = table
            .get("y")
            .and_then(|v| v.as_float())
            .unwrap_or_default();

        Ok(Align { x, y })
    }
}

impl TryFrom<Vec<f64>> for Align {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(Align {
                x: value[0],
                y: value[0],
            }),
            2 => Ok(Align {
                x: value[0],
                y: value[1],
            }),
            _ => Err(err_from_to!(
                "Vec<f64>" => "Makepad Align, params number incorrect need 1 or 2"
            )),
        }
    }
}

impl TryFrom<&Struct> for Align {
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
                return Err(err_from_to!("Struct" => "Vec4, params number incorrect need [1, 2]"));
            }

            let mut x = None;
            let mut y = None;

            for (key, value) in fields {
                match key.as_str() {
                    "x" => x = Some(value.as_f64()?),
                    "y" => y = Some(value.as_f64()?),
                    _ => {}
                }
            }

            return Ok(Align {
                x: x.unwrap_or_default(),
                y: y.unwrap_or_default(),
            });
        }

        Err(err_from_to!("Struct" => "Align"))
    }
}

impl TryFrom<f64> for Align {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Align { x: value, y: value })
    }
}

try_from_value_ref_struct! {
    Align, "Align", f64
}

struct_float_to_tokens! {
    Align{x, y}
}
