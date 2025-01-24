mod bind;
mod r#enum;
mod function;
mod r#struct;

use std::{collections::HashMap, fmt::Display, str::FromStr};

pub use bind::*;
pub use function::Function;
use gen_utils::{
    common::format_float,
    error::{ConvertError, Error},
    from_i_number, from_u_number, from_value_to_primitive,
};
pub use r#enum::{Enum, EnumItem};

pub use r#struct::Struct;

use crate::{as_value, common::BuiltinColor, PropsKey};

/// # GenUI Value Type
/// Value Type need to use in <template>|<style> tag to handle the value
/// ## Call Function
/// - parse_style: parse the value in <style> 👍
/// - parse_template: parse the value in <template> 👍
/// ## Test
/// - See [test_value](tests/src/parser/value/mod.rs)
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    // u type number
    // U8(u8),
    // U16(u16),
    // U32(u32),
    // U64(u64),
    USize(usize),
    // i type number
    // I8(i8),
    // I16(i16),
    // I32(i32),
    // I64(i64),
    ISize(isize),
    // float
    Double(f64),
    /// bool
    Bool(bool),
    Vec(Vec<Value>),
    /// String
    String(String),
    /// value inject
    /// <xxx :value="xValue" />
    /// <script> let xValue:&str = "hello!";</script>
    /// <script> let xValue:Vec<&str> = vec!["a","b"];</script>
    Bind(Bind),
    /// function inject
    /// <xxx @click="doClick" />
    Function(Function),
    /// ⚠️ deprecated!!! function return ()  as :`fn xxx()->(){}`
    // Void,
    // /// color value
    // /// - hex color: #fff00f
    // /// - rgb color: rgb(211,23,255)
    // /// - rgba color: rgba(255,255,87,0.4)
    // Color(BuiltinColor),
    Struct(Struct),
    /// Enum value
    Enum(Enum),
    /// unknown value type, use when the value is not other type, such as deg, px, #color, etc.
    /// means you need to convert it to the correct type in the needed place
    UnKnown(String),
    /// Dep means the value is the static dependency ⚠️ deprecated!!! now move to Function
    // Dep(String),
    /// animation value
    Animation(HashMap<PropsKey, Value>),
}

impl Value {
    pub fn parse(s: &str, is_style: bool) -> Result<Value, Error> {
        if is_style {
            Value::parse_style(s)
        } else {
            Value::parse_template(s)
        }
    }
    pub fn bind_name(&self) -> String {
        let bind = self.as_bind().expect("Value is not Bind");
        bind.ident()
    }

    /// parse the value in <style>
    pub fn parse_style(s: &str) -> Result<Value, Error> {
        Bind::parse_style(s)
            .map(Value::Bind)
            .or_else(|_| Function::parse(s, true).map(Value::Function))
            .or_else(|_| Struct::parse_style(s).map(Value::Struct))
            .or_else(|_| s.parse())
            .or_else(|_| Enum::parse_style(s).map(Value::Enum))
            .or_else(|_| Ok(Value::UnKnown(s.to_string())))
    }
    /// parse the value in <template>
    pub fn parse_template(s: &str) -> Result<Value, Error> {
        fn value_string(s: &str) -> Result<Value, Error> {
            // here do as serde value parse, but in template, use `'` as string
            let s = s.trim();
            if s.starts_with('\'') && s.ends_with('\'') {
                let s = s.trim_matches(|c| c == '\'');
                return Ok(Value::String(s.to_string()));
            }

            Err(ConvertError::FromTo {
                from: "str".to_string(),
                to: "Value::String".to_string(),
            }
            .into())
        }

        fn value_vec(s: &str) -> Result<Value, Error> {
            let s = s.trim();
            if s.starts_with('[') && s.ends_with(']') {
                let s = s.trim_matches(|c| c == '[' || c == ']');
                let v = s
                    .split(',')
                    .map(|x| x.trim())
                    .map(|x| Value::parse_template(x))
                    .collect::<Result<Vec<Value>, Error>>()?;
                return Ok(Value::Vec(v));
            }
            Err(ConvertError::FromTo {
                from: "str".to_string(),
                to: "Value::Vec".to_string(),
            }
            .into())
        }

        s.parse()
            .or_else(|_| value_vec(s))
            .or_else(|_| value_string(s))
            .or_else(|_| Function::parse(s, false).map(Value::Function))
            .or_else(|_| Struct::parse_template(s).map(Value::Struct))
            .or_else(|_| Enum::parse_template(s).map(Value::Enum))
            .or_else(|_| Ok(Value::UnKnown(s.to_string())))
    }
    /// check if the value is unknown
    /// - true: unknown
    fn check_unknown(s: &str) -> bool {
        // if split by whitespace, need to check length, length must > 1
        let res = s.split_whitespace().collect::<Vec<&str>>();
        if res.len() == 1 {
            return true;
        } else {
            // try convert to f64, if any one is not f64, return true
            return !(res.iter().all(|x| x.parse::<f64>().is_ok()));
        }
    }
    as_value! {
        as_usize, usize, "usize" => Value::USize,
        as_isize, isize, "isize" => Value::ISize,
        as_f64, f64, "f64" => Value::Double,
        as_bool, bool, "bool" => Value::Bool,
        as_fn, Function, "Function" => Value::Function,
        as_enum , Enum, "Enum" => Value::Enum,
        as_string, String, "String" => Value::String,
        as_bind, Bind, "Bind" => Value::Bind,
        as_struct, Struct, "Struct" => Value::Struct,
        as_unkonwn, String, "UnKnown" => Value::UnKnown
    }
    pub fn as_f32(&self) -> Result<f32, Error> {
        match self {
            Value::Double(n) => Ok(*n as f32),
            _ => Err(ConvertError::FromTo {
                from: "Value".to_string(),
                to: "f32".to_string(),
            }
            .into()),
        }
    }
    pub fn is_color_and_get(&self) -> Result<BuiltinColor, Error> {
        self.try_into()
    }
    pub fn is_animation_and_get(&self) -> Option<&HashMap<PropsKey, Value>> {
        match self {
            Value::Animation(a) => Some(a),
            _ => None,
        }
    }
}

// impl From<(&str, Option<Vec<&str>>, bool)> for Value {
//     fn from(value: (&str, Option<Vec<&str>>, bool)) -> Self {
//         Value::Function(value.into())
//     }
// }

from_u_number!(u8);
from_u_number!(u16);
from_u_number!(u32);
from_u_number!(u64);
from_i_number!(i8);
from_u_number!(i16);
from_u_number!(i32);
from_u_number!(i64);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Value::String(value.to_string())
    }
}

// impl From<Color> for Value {
//     fn from(value: Color) -> Self {
//         Value::Color(value)
//     }
// }

impl From<Function> for Value {
    fn from(value: Function) -> Self {
        Value::Function(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Double(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<Struct> for Value {
    fn from(value: Struct) -> Self {
        Value::Struct(value)
    }
}

impl From<Enum> for Value {
    fn from(value: Enum) -> Self {
        Value::Enum(value)
    }
}

impl From<Bind> for Value {
    fn from(value: Bind) -> Self {
        Value::Bind(value)
    }
}

impl TryFrom<serde_json::Value> for Value {
    type Error = Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Null => {
                Err(ConvertError::Serde("null value now is not supported".to_string()).into())
            }
            serde_json::Value::Bool(b) => Ok(Value::Bool(b)),
            serde_json::Value::Number(n) => {
                if n.is_f64() {
                    Ok(Value::Double(n.as_f64().unwrap()))
                } else if n.is_i64() {
                    Ok(Value::ISize(n.as_i64().unwrap() as isize))
                } else if n.is_u64() {
                    Ok(Value::USize(n.as_u64().unwrap() as usize))
                } else {
                    Err(ConvertError::FromTo {
                        from: "type number".to_string(),
                        to: "Value::Double | Value::ISize | Value::USize".to_string(),
                    }
                    .into())
                }
            }
            serde_json::Value::String(s) => Ok(Value::String(s)),
            serde_json::Value::Array(arr) => {
                // 递归转换
                Ok(Value::Vec(
                    arr.into_iter()
                        .map(|x| x.try_into().unwrap())
                        .collect::<Vec<Value>>(),
                ))
            }
            serde_json::Value::Object(_) => Ok(Value::Struct(value.try_into()?)),
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(v) => v.try_into(),
            Err(e) => Err(ConvertError::Serde(e.to_string()).into()),
        }
    }
}

/// from str to value
/// (parse_value, is_style)
impl TryFrom<(&str, bool)> for Value {
    type Error = Error;

    fn try_from(value: (&str, bool)) -> Result<Self, Self::Error> {
        Value::parse(value.0, value.1)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Value::USize(n) => n.to_string(),
            Value::ISize(n) => n.to_string(),
            Value::Double(n) => format_float(*n),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::Bind(bind) => bind.to_string(),
            Value::Function(func) => func.to_string(),
            Value::Struct(s) => s.to_string(),
            Value::Enum(e) => e.to_string(),
            Value::Vec(v) => format!(
                "[{}]",
                v.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Animation(anim) => format!(
                "{:?}",
                anim.into_iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<String>>()
            ),
            Value::UnKnown(s) => s.to_string(),
        };

        f.write_str(&res)
    }
}

// impl ToTokens for Value {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         let tk = match self{
//             Value::USize(num) => parse_str::<TokenStream>(&num.to_string()).unwrap(),
//             Value::ISize(num) => parse_str::<TokenStream>(&num.to_string()).unwrap(),
//             Value::Double(num) => parse_str::<TokenStream>(&format_float(*num)).unwrap() ,
//             Value::Bool(b) => parse_str::<TokenStream>(&b.to_string()).unwrap(),
//             Value::Vec(vecs) => {
//                 let vecs_tk = vecs.iter().map(|x| x.to_token_stream());
//                 quote! {
//                     [#(#vecs_tk),*]
//                 }
//             } ,
//             Value::String(s) => s.to_token_stream(),
//             Value::Bind(bind) => todo!(),
//             Value::Function(function) => todo!(),
//             Value::Struct(_) => todo!(),
//             Value::Enum(_) => todo!(),
//             Value::Animation(hash_map) => todo!(),
//             Value::UnKnown(s) => parse_str::<TokenStream>(s).unwrap(),
//         };

//         tokens.extend(tk);
//     }
// }

fn primitive_callback<T, F, R>(value: T, f: F) -> R
where
    F: FnOnce(T) -> R,
{
    f(value)
}

from_value_to_primitive! {
    usize , "usize", Value::USize => |n| n as usize,
    isize, "isize", Value::ISize => |n| n as isize,
    i32, "i32", Value::ISize => |n| n as i32,
    i64, "i64", Value::ISize => |n| n as i64,
    u32, "u32", Value::USize => |n| n as u32,
    u64, "u64", Value::USize => |n| n as u64,
    f64, "f64", Value::Double => |n| n as f64,
    f32, "f32", Value::Double => |n| n as f32,
    bool, "bool", Value::Bool => |b| b
}
