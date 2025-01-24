use std::{collections::BTreeMap, fmt::Display};

use gen_utils::{
    error::{Error, ParseError},
    parser::{parse_value, trim},
};
use nom::{bytes::complete::take_till1, character::complete::char, multi::many0, IResult};

use crate::PropertyKeyType;

use super::Value;

/// # Struct Value
/// struct value used in style (seldom used now)
/// ## Test
/// See [test_struct](tests/src/parser/value/struct.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: Option<String>,
    pub fields: BTreeMap<String, Value>,
    pub is_anonymous: bool,
}

impl Struct {
    pub fn new(name: &str) -> Self {
        Struct {
            name: Some(name.to_string()),
            fields: BTreeMap::new(),
            is_anonymous: false,
        }
    }
    pub fn insert(&mut self, key: &str, value: Value) {
        self.fields.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }
    pub fn set_name(&mut self, name: &str) {
        self.name.replace(name.to_string());
        self.is_anonymous = false;
    }
    pub fn parse_style(s: &str) -> Result<Self, Error> {
        match parse_struct_style(s) {
            Ok((_, res)) => Ok(res),
            Err(_) => Err(ParseError::template("parse style struct failed").into()),
        }
    }
    pub fn parse_template(s: &str) -> Result<Self, Error> {
        Self::parse_style(s)
    }
}

impl TryFrom<serde_json::Value> for Struct {
    type Error = Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let mut fields = BTreeMap::new();
        if let serde_json::Value::Object(obj) = value {
            for (k, v) in obj {
                fields.insert(k, Value::try_from(v)?);
            }
        }
        Ok(Struct {
            name: None,
            fields,
            is_anonymous: true,
        })
    }
}

impl From<Vec<(&str, &str)>> for Struct {
    fn from(value: Vec<(&str, &str)>) -> Self {
        let mut fields = BTreeMap::new();
        for (k, v) in value {
            fields.insert(k.to_string(), Value::from(v));
        }
        Struct {
            name: None,
            fields,
            is_anonymous: true,
        }
    }
}

impl From<Vec<(&str, (Value, PropertyKeyType))>> for Struct {
    fn from(value: Vec<(&str, (Value, PropertyKeyType))>) -> Self {
        let mut fields = BTreeMap::new();
        for (k, (v, _)) in value {
            fields.insert(k.to_string(), v);
        }
        Struct {
            name: None,
            fields,
            is_anonymous: true,
        }
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = String::new();
        for (k, v) in &self.fields {
            fields.push_str(&format!("{}: {}, ", k, v));
        }
        if self.is_anonymous {
            write!(f, "{{{}}}", fields)
        } else {
            write!(
                f,
                "{} {{{}}}",
                self.name.as_ref().unwrap_or(&String::new()),
                fields
            )
        }
    }
}

fn parse_kv(input: &str) -> IResult<&str, (&str, Value)> {
    let mut res = Struct {
        name: None,
        fields: BTreeMap::new(),
        is_anonymous: true,
    };

    let (input, k) = trim(parse_value)(input)?;
    let (input, _) = trim(char(':'))(input)?;

    if input.trim().starts_with('{') {
        let (input, _) = char('{')(input)?;
        let (input, fields) = many0(parse_kv)(input)?;
        let (input, _) = char('}')(input)?;
        for (k, v) in fields {
            res.fields.insert(k.to_string(), v);
        }
        return Ok((input, (k, Value::Struct(res))));
    } else {
        let (input, v) = trim(take_till1(|c| c == ',' || c == '}'))(input)?;
        let input = if input.starts_with(',') {
            let (input, _) = trim(char(','))(input)?;
            input
        } else {
            input
        };
        let v = Value::parse_style(v).map_err(|_| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail))
        })?;
        return Ok((input, (k, v)));
    }
}
/// parse struct in <style>
fn parse_struct_style(s: &str) -> IResult<&str, Struct> {
    // let mut map = BTreeMap::new();
    let mut res = Struct {
        name: None,
        fields: BTreeMap::new(),
        is_anonymous: true,
    };
    // let input = s.trim().trim_matches(|c| c == '{' || c == '}');
    let (input, _) = char('{')(s.trim())?;
    let (input, fields) = match trim(char('}'))(input) {
        Ok((input, _)) => (input, vec![]),
        Err(_) => {
            let (input, fields) = many0(parse_kv)(input)?;

            // let (input, children) = many0(parse_struct_style)(input)?;
            // let (input, _) = trim(char('}'))(input)?;
            (input, fields)
        }
    };

    if !fields.is_empty() {
        for (k, v) in fields {
            res.fields.insert(k.to_string(), v);
        }
    }

    // if !children.is_empty() {
    //     for child in children {
    //         dbg!(child);
    //         // res.fields.insert(k.to_string(), Value::from(v));
    //     }
    // }

    Ok((input, res))
}

