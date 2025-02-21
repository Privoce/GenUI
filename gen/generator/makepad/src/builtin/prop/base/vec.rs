use std::{num::ParseFloatError, str::FromStr};

use gen_analyzer::value::{Struct, Value};
use gen_utils::error::Error;
use proc_macro2::TokenStream;

use crate::{
    builtin::prop::{err_from_to, utils::convert_str_to_vec},
    struct_float_dvec_to_tokens, struct_float_to_tokens, try_from_f64_vec,
    try_from_value_ref_struct,
};

#[derive(Debug, Clone)]
pub struct VecValue<T>(pub Vec<T>)
where
    T: quote::ToTokens + TryFrom<&'static Value, Error = Error>;

impl<T> TryFrom<&Value> for VecValue<T>
where
    T: TryFrom<&'static Value, Error = Error> + quote::ToTokens,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Value::Vec(v) = value {
            let mut values = Vec::new();
            for item in v {
                let item_clone = item.clone();
                values.push(T::try_from(Box::leak(Box::new(item_clone)))?);
            }

            Ok(VecValue(values))
        } else {
            return Err(err_from_to("GenUI Value", "VecValue<T>").into());
        }
    }
}

impl<T> quote::ToTokens for VecValue<T>
where
    T: quote::ToTokens + TryFrom<&'static Value, Error = Error>,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let value = self
            .0
            .iter()
            .map(|x| x.to_token_stream())
            .collect::<Vec<TokenStream>>();
        tokens.extend(quote::quote! {
           [#(#value),*]
        });
    }
}

// [DVec2~DVec4] ---------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}

impl From<&DVec2> for toml_edit::Value {
    fn from(value: &DVec2) -> Self {
        let mut table = toml_edit::InlineTable::new();
        table.insert("x", value.x.into());
        table.insert("y", value.y.into());
        toml_edit::Value::InlineTable(table)
    }
}

impl TryFrom<&toml_edit::Value> for DVec2 {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        let table = value
            .as_inline_table()
            .ok_or_else(|| err_from_to("toml_edit::Value", "DVec2"))?;
        let x = table
            .get("x")
            .and_then(|v| v.as_float())
            .unwrap_or_default();
        let y = table
            .get("y")
            .and_then(|v| v.as_float())
            .unwrap_or_default();
        Ok(DVec2 { x, y })
    }
}

impl TryFrom<&Struct> for DVec2 {
    type Error = Error;

    fn try_from(value: &Struct) -> Result<Self, Self::Error> {
        let Struct {
            fields,
            is_anonymous,
            ..
        } = value;
        if *is_anonymous {
            let len = fields.len();
            if len == 0 || len > 2 {
                return Err(
                    err_from_to("Struct", "DVec2, params number incorrect need [1, 2]").into(),
                );
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

            return Ok(DVec2 {
                x: x.unwrap_or_default(),
                y: y.unwrap_or_default(),
            });
        }
        Err(err_from_to("Struct", "DVec2").into())
    }
}

impl TryFrom<Vec<f64>> for DVec2 {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(DVec2 {
                x: value[0],
                y: value[0],
            }),
            2 => Ok(DVec2 {
                x: value[0],
                y: value[1],
            }),
            _ => Err(err_from_to("str", "DVec2, params number incorrect need 1 or 2").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl TryFrom<&Struct> for DVec3 {
    type Error = Error;

    fn try_from(value: &Struct) -> Result<Self, Self::Error> {
        let Struct {
            fields,
            is_anonymous,
            ..
        } = value;

        if *is_anonymous {
            let len = fields.len();
            if len == 0 || len > 3 {
                return Err(
                    err_from_to("Struct", "DVec3, params number incorrect need [1, 3]").into(),
                );
            }

            let mut x = None;
            let mut y = None;
            let mut z = None;

            for (key, value) in fields {
                match key.as_str() {
                    "x" => x = Some(value.as_f64()?),
                    "y" => y = Some(value.as_f64()?),
                    "z" => z = Some(value.as_f64()?),
                    _ => {}
                }
            }

            return Ok(DVec3 {
                x: x.unwrap_or_default(),
                y: y.unwrap_or_default(),
                z: z.unwrap_or_default(),
            });
        }

        Err(err_from_to("Struct", "DVec3").into())
    }
}

impl TryFrom<Vec<f64>> for DVec3 {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(DVec3 {
                x: value[0],
                y: value[0],
                z: value[0],
            }),
            3 => Ok(DVec3 {
                x: value[0],
                y: value[1],
                z: value[2],
            }),
            _ => Err(err_from_to("str", "DVec3, params number incorrect need 1 or 3").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl TryFrom<&Struct> for DVec4 {
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
                    err_from_to("Struct", "DVec4, params number incorrect need [1, 4]").into(),
                );
            }

            let mut x = None;
            let mut y = None;
            let mut z = None;
            let mut w = None;

            for (key, value) in fields {
                match key.as_str() {
                    "x" => x = Some(value.as_f64()?),
                    "y" => y = Some(value.as_f64()?),
                    "z" => z = Some(value.as_f64()?),
                    "w" => w = Some(value.as_f64()?),
                    _ => {}
                }
            }

            return Ok(DVec4 {
                x: x.unwrap_or_default(),
                y: y.unwrap_or_default(),
                z: z.unwrap_or_default(),
                w: w.unwrap_or_default(),
            });
        }

        Err(err_from_to("Struct", "DVec4").into())
    }
}

impl TryFrom<Vec<f64>> for DVec4 {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(DVec4 {
                x: value[0],
                y: value[0],
                z: value[0],
                w: value[0],
            }),
            4 => Ok(DVec4 {
                x: value[0],
                y: value[1],
                z: value[2],
                w: value[3],
            }),
            _ => Err(err_from_to("str", "DVec4, params number incorrect need 1 or 4").into()),
        }
    }
}
// [Vec2~Vec4] ---------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<&Vec2> for toml_edit::Value {
    fn from(value: &Vec2) -> Self {
        let mut table = toml_edit::InlineTable::new();
        table.insert("x", (value.x as f64).into());
        table.insert("y", (value.y as f64).into());
        toml_edit::Value::InlineTable(table)
    }
}

impl TryFrom<&toml_edit::Value> for Vec2 {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        let table = value
            .as_inline_table()
            .ok_or_else(|| err_from_to("toml_edit::Item", "Vec2"))?;

        let x = table
            .get("x")
            .and_then(|v| v.as_float())
            .unwrap_or_default() as f32;
        let y = table
            .get("y")
            .and_then(|v| v.as_float())
            .unwrap_or_default() as f32;
        Ok(Vec2 { x, y })
    }
}

impl TryFrom<&Struct> for Vec2 {
    type Error = Error;

    fn try_from(value: &Struct) -> Result<Self, Self::Error> {
        let Struct {
            fields,
            is_anonymous,
            ..
        } = value;
        if *is_anonymous {
            let len = fields.len();
            if len == 0 || len > 2 {
                return Err(
                    err_from_to("Struct", "Vec2, params number incorrect need [1, 2]").into(),
                );
            }
            let mut x = None;
            let mut y = None;

            for (key, value) in fields {
                match key.as_str() {
                    "x" => x = Some(value.as_f32()?),
                    "y" => y = Some(value.as_f32()?),
                    _ => {}
                }
            }

            return Ok(Vec2 {
                x: x.unwrap_or_default(),
                y: y.unwrap_or_default(),
            });
        }
        Err(err_from_to("Struct", "Vec2").into())
    }
}

impl TryFrom<Vec<f32>> for Vec2 {
    type Error = Error;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(Vec2 {
                x: value[0],
                y: value[0],
            }),
            2 => Ok(Vec2 {
                x: value[0],
                y: value[1],
            }),
            _ => Err(err_from_to("str", "Vec2, params number incorrect need 1 or 2").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TryFrom<Vec<f32>> for Vec3 {
    type Error = Error;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(Vec3 {
                x: value[0],
                y: value[0],
                z: value[0],
            }),
            3 => Ok(Vec3 {
                x: value[0],
                y: value[1],
                z: value[2],
            }),
            _ => Err(err_from_to("str", "Vec3, params number incorrect need 1 or 3").into()),
        }
    }
}

impl TryFrom<&Struct> for Vec3 {
    type Error = Error;

    fn try_from(value: &Struct) -> Result<Self, Self::Error> {
        let Struct {
            fields,
            is_anonymous,
            ..
        } = value;

        if *is_anonymous {
            let len = fields.len();
            if len == 0 || len > 3 {
                return Err(
                    err_from_to("Struct", "Vec3, params number incorrect need [1, 3]").into(),
                );
            }

            let mut x = None;
            let mut y = None;
            let mut z = None;

            for (key, value) in fields {
                match key.as_str() {
                    "x" => x = Some(value.as_f32()?),
                    "y" => y = Some(value.as_f32()?),
                    "z" => z = Some(value.as_f32()?),
                    _ => {}
                }
            }

            return Ok(Vec3 {
                x: x.unwrap_or_default(),
                y: y.unwrap_or_default(),
                z: z.unwrap_or_default(),
            });
        }

        Err(err_from_to("Struct", "Vec3").into())
    }
}

#[derive(Debug, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl TryFrom<Vec<f32>> for Vec4 {
    type Error = Error;

    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        match value.len() {
            1 => Ok(Vec4 {
                x: value[0],
                y: value[0],
                z: value[0],
                w: value[0],
            }),
            4 => Ok(Vec4 {
                x: value[0],
                y: value[1],
                z: value[2],
                w: value[3],
            }),
            _ => Err(err_from_to("str", "Vec4, params number incorrect need 1 or 4").into()),
        }
    }
}

impl TryFrom<&Struct> for Vec4 {
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

            let mut x = None;
            let mut y = None;
            let mut z = None;
            let mut w = None;

            for (key, value) in fields {
                match key.as_str() {
                    "x" => x = Some(value.as_f32()?),
                    "y" => y = Some(value.as_f32()?),
                    "z" => z = Some(value.as_f32()?),
                    "w" => w = Some(value.as_f32()?),
                    _ => {}
                }
            }

            return Ok(Vec4 {
                x: x.unwrap_or_default(),
                y: y.unwrap_or_default(),
                z: z.unwrap_or_default(),
                w: w.unwrap_or_default(),
            });
        }

        Err(err_from_to("Struct", "Vec4").into())
    }
}

try_from_f64_vec! {
    DVec2 { x, y },
    DVec3 { x, y, z },
    DVec4 { x, y, z, w },
    Vec2 { x, y },
    Vec3 { x, y, z },
    Vec4 { x, y, z, w }
}

struct_float_to_tokens! {
    Vec2{ x , y },
    Vec3{ x , y , z },
    Vec4{ x , y , z , w }
}

struct_float_dvec_to_tokens! {
    DVec2 => vec2(x, y),
    DVec3 => vec3(x, y, z),
    DVec4 => vec4(x, y, z, w)
}

try_from_value_ref_struct! {
    DVec2, "DVec2", f64,
    DVec3, "DVec3", f64,
    DVec4, "DVec4", f64,
    Vec2, "Vec2", f32,
    Vec3, "Vec3", f32,
    Vec4, "Vec4", f32
}

#[cfg(test)]
mod test_vec {
    use gen_analyzer::value::Value;

    use crate::builtin::prop::DVec2;

    #[test]
    fn test_vec2() {
        let v = Value::parse_style("{x: 1.0, y: 2.0}").unwrap();
        let d = DVec2::try_from(&v).unwrap();
        let v2 = Value::String("1.0 2.0".to_string());
        let d2 = DVec2::try_from(&v2).unwrap();
        let v3 = Value::parse_style("{y: 2.0}").unwrap();
        let d3 = DVec2::try_from(&v3).unwrap();
        dbg!(d);
        dbg!(d2);
        dbg!(d3);
    }
}
