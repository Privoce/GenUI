use std::{collections::HashMap, fmt::Display, str::FromStr};

use gen_utils::{common::tokenizer::SPACE, error::Error};

use crate::value::{Bind, Function, Value};

// use crate::{Bind, Function, Value};
/// # Builtin props
/// |Name    | Description              | Format                         |
/// |--------|--------------------------|--------------------------------|
/// |for     | GenUI Loop Prop Key      | `:for="(index, item) in list"` |
/// |if      | GenUI If Prop Key        | `:if="condition"`              |
/// |else_if | GenUI Else If Prop Key   | `:else_if="condition"`         |
/// |else    | GenUI Else Prop Key      | `else`                         |
/// |as_prop | GenUI As Prop Key        | `as_prop="true"` or `as_prop`  |
/// |id      | GenUI Id Prop Key        | `id="id"`                      |
/// |class   | GenUI Class Prop Key     | `class="class1 class2"`        |
pub const BUILTIN_PROPS: [&str; 8] = [
    "for", "if", "else_if", "else", "as_prop", "id", "class", "inherits",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinProps {
    For,
    If,
    ElseIf,
    Else,
    AsProp,
    Id,
    Class,
    Inherits,
}

impl FromStr for BuiltinProps {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "for" => Ok(BuiltinProps::For),
            "if" => Ok(BuiltinProps::If),
            "else_if" => Ok(BuiltinProps::ElseIf),
            "else" => Ok(BuiltinProps::Else),
            "as_prop" => Ok(BuiltinProps::AsProp),
            "id" => Ok(BuiltinProps::Id),
            "class" => Ok(BuiltinProps::Class),
            "inherits" => Ok(BuiltinProps::Inherits),
            _ => Err(Error::from(format!("Invalid builtin props: {}", s))),
        }
    }
}

/// # Property Key Type
/// - Normal: normal property key, no prefix
/// - Bind: bind property key, use `:` to define
/// - Function: function property key, use `@` to define
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum PropKeyType {
    Normal,
    /// :xxx
    Bind,
    /// @xxx
    Function,
}

#[allow(dead_code)]
impl PropKeyType {
    /// ## convert value to Builtin Value
    /// - normal => Value::UnKnown
    /// - bind => Value::Bind
    /// - function => Value::Function
    pub fn to_value(&self, value: &str) -> Result<Value, Error> {
        match self {
            PropKeyType::Normal => Value::parse_template(value),
            PropKeyType::Bind => Bind::parse_template(value).and_then(|bind| Ok(bind.into())),
            PropKeyType::Function => Function::parse(value, false).and_then(|f| Ok(f.into())),
        }
    }
    /// ## check current property key type is normal or not
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }
    /// ## check current property key type is bind or not
    pub fn is_bind(&self) -> bool {
        matches!(self, Self::Bind)
    }
    /// ## check current property key type is function or not
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function)
    }
}

impl Default for PropKeyType {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for PropKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            PropKeyType::Normal => "normal",
            PropKeyType::Bind => "bind",
            PropKeyType::Function => "function",
        };
        f.write_str(res)
    }
}

impl From<&str> for PropKeyType {
    fn from(value: &str) -> Self {
        match value {
            "" => PropKeyType::Normal,
            ":" => PropKeyType::Bind,
            "@" => PropKeyType::Function,
            _ => panic!("Invalid property key"),
        }
    }
}

/// # Property Key
/// Parse the property key in template or style tag
/// ## Format
/// - normal: `name`
/// - bind: `:name`
/// - function: `@name`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropKey {
    /// property key name
    name: String,
    /// same as function
    /// judge the use place (template|style)
    /// has behave differently
    is_style: bool,
    /// property key type
    ty: PropKeyType,
}

impl PropKey {
    pub fn new(name: &str, is_style: bool, ty: PropKeyType) -> Self {
        PropKey {
            name: name.to_string(),
            is_style,
            ty,
        }
    }
    /// ## new props key
    /// new a props key in template or script tag which is type normal
    pub fn new_tag_normal(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_style: false,
            ty: PropKeyType::Normal,
        }
    }
    pub fn new_bind(name: &str, is_style: bool) -> Self {
        Self {
            name: name.to_string(),
            is_style,
            ty: PropKeyType::Bind,
        }
    }
    pub fn new_fn(name: &str, is_style: bool) -> Self {
        Self {
            name: name.to_string(),
            is_style,
            ty: PropKeyType::Function,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &PropKeyType {
        &self.ty
    }
    pub fn is_bind(&self) -> bool {
        self.ty.is_bind()
    }
    pub fn is_normal(&self) -> bool {
        self.ty.is_normal()
    }
    pub fn is_fn(&self) -> bool {
        self.ty.is_function()
    }
    /// ## check current props key is builtin or not
    pub fn is_builtin(&self) -> bool {
        BUILTIN_PROPS.contains(&self.name())
    }
    pub fn from_value_with(v: &Value, name: &str, is_style: bool) -> Self {
        match v {
            Value::Bind(_) => PropKey::new_bind(name, is_style),
            Value::Function(_) => PropKey::new_fn(name, is_style),
            _ => PropKey::new(name, is_style, PropKeyType::Normal),
        }
    }
}

impl Display for PropKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            PropKeyType::Normal => f.write_str(self.name()),
            PropKeyType::Bind => {
                if self.is_style {
                    f.write_str(self.name())
                } else {
                    f.write_fmt(format_args!(":{}", self.name()))
                }
            }
            PropKeyType::Function => {
                if self.is_style {
                    f.write_str(self.name())
                } else {
                    f.write_fmt(format_args!("@{}", self.name()))
                }
            }
        }
    }
}

pub type Props = HashMap<PropKey, Value>;

pub fn props_to_string<'a, F>(props: Props, format: F) -> String
where
    F: FnMut((PropKey, Value)) -> String,
{
    match props {
        Some(props) => props
            .into_iter()
            .map(format)
            .collect::<Vec<String>>()
            .join(SPACE),
        None => String::new(),
    }
}

pub fn props_to_template_string(props: Props) -> String {
    props_to_string(props, |(k, v)| {
        format!(r#"{}="{}""#, k.to_string(), v.to_string())
    })
}

pub fn props_to_style_string(props: Props) -> String {
    props_to_string(props, |(k, v)| {
        format!(r#"{}: {};"#, k.to_string(), v.to_string())
    })
}