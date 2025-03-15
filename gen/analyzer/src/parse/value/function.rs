use std::{fmt::Display, str::FromStr};

use gen_utils::{
    error::{Error, ParseError},
    parser::parse_value,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::recognize,
    sequence::{delimited, pair},
    IResult,
};

// use crate::{common::Special, target::function};

use super::{special::Special, Value};

/// # Function Value
/// - in template: `:clicked="easy([args...])"` easy is a function, args is a list of function args
/// - in style: `color: rgb(r, g, b);`
/// ## Test
/// See [test_func](tests/src/parser/value/function.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// function name (ident)
    pub name: String,
    /// function params
    pub params: Option<Vec<Value>>,
    /// use to recognize the function is used on the `template` or `style`
    /// if is `style`: `()` should be exist in the function when the function is called (although no args)
    pub is_style: bool,
}

impl Function {
    pub fn ident(&self) -> &str {
        &self.name
    }
    pub fn new(name: &str, params: Option<Vec<Value>>, is_style: bool) -> Self {
        // check params
        let params = match params {
            Some(p) => {
                if p.is_empty() {
                    None
                } else {
                    Some(p)
                }
            }
            None => None,
        };

        Function {
            name: String::from(name),
            params,
            is_style,
        }
    }
    pub fn parse(s: &str, is_style: bool) -> Result<Self, Error> {
        (s, is_style).try_into()
    }
    pub fn params_str(&self) -> Option<String> {
        if let Some(params) = self.params.as_ref() {
            return Some(
                params
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        } else {
            return None;
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        if let Some(params) = self.params_str() {
            return write!(f, "{}({})", &name, params);
        } else {
            return if self.is_style {
                write!(f, "{}()", &name)
            } else {
                write!(f, "{}", &name)
            };
        }
    }
}

impl From<(&str, Option<Vec<Value>>, bool)> for Function {
    fn from(value: (&str, Option<Vec<Value>>, bool)) -> Self {
        Function {
            name: String::from(value.0),
            params: value.1,
            is_style: value.2,
        }
    }
}

/// parse function from `(fn_name, params, is_style)`
impl TryFrom<(&str, &str, bool)> for Function {
    type Error = Error;

    fn try_from(value: (&str, &str, bool)) -> Result<Self, Self::Error> {
        // try split &str
        // remove `()`
        if let Ok(_) = Special::from_str(value.0) {
            return Ok((
                value.0,
                Some(vec![Value::String(value.1.to_string())]),
                value.2,
            )
                .into());
        }
        let (_, params_str) = remove_holder(value.1).unwrap();
        return if params_str.is_empty() {
            Ok((value.0, None, value.2).into())
        } else {
            let mut params = vec![];

            for param in params_str.split(",") {
                params.push((param, value.2).try_into()?);
            }

            Ok((value.0, Some(params), value.2).into())
        };
    }
}

impl TryFrom<(&str, bool)> for Function {
    type Error = Error;

    fn try_from(value: (&str, bool)) -> Result<Self, Self::Error> {
        let (s, is_style) = value;
        match parse_function(s) {
            Ok((remain, (name, params, _))) => {
                if remain.is_empty() {
                    return (name, params, is_style).try_into();
                } else {
                    return Err(ParseError::template("parse function still has remain").into());
                }
            }
            Err(e) => Err(e.to_string().into()),
        }
    }
}

fn parse_function(input: &str) -> IResult<&str, (&str, &str, bool)> {
    match function(input) {
        Ok((input, (_, (name, params, is_style)))) => {
            Ok((input, (name, params, is_style.unwrap())))
        }
        Err(e) => Err(e),
    }
}

/// end () `(type, (name,params))`
pub fn function(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
    fn normal_fn(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
        let (input, (name, params)) = pair(
            parse_value,
            recognize(delimited(tag("("), take_until(")"), tag(")"))),
        )(input)?;

        Ok((input, ("()", (name, params, Some(true)))))
    }
    alt((Special::makepad_shader_parser, normal_fn))(input)
}

fn remove_holder(input: &str) -> IResult<&str, &str> {
    delimited(tag("("), take_until(")"), tag(")"))(input)
}
