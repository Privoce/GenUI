use std::{collections::HashMap, fmt::Display};

use gen_utils::common::tokenizer::SPACE;

use crate::{Bind, Value};
/// # Builtin props
/// |Name    | Description              | Format                         |
/// |--------|--------------------------|--------------------------------|
/// |for     | GenUI Loop Prop Key      | `:for="(index, item) in list"` |
/// |if      | GenUI If Prop Key        | `:if="condition"`              |
/// |else_if | GenUI Else If Prop Key   | `:else_if="condition"`         |
/// |else    | GenUI Else Prop Key      | `else`                         |
/// |as_prop | GenUI As Prop Key        | `as_prop="true"` or `as_prop`  |
/// |id      | GenUI Id Prop Key        | `id="id"`                      |
pub const BUILTIN_PROPS: [&str; 7] = ["for", "if", "else_if", "else", "as_prop", "id", "class"];

/// # Property Key Type
/// - Normal: normal property key, no prefix
/// - Bind: bind property key, use `:` to define
/// - Function: function property key, use `@` to define
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum PropertyKeyType {
    Normal,
    /// :xxx
    Bind,
    /// @xxx
    Function,
}

#[allow(dead_code)]
impl PropertyKeyType {
    /// ## convert value to Builtin Value
    /// - normal => Value::UnKnown
    /// - bind => Value::Bind
    /// - function => Value::Function
    pub fn to_value(&self, value: &str) -> Value {
        let value = value.to_string();
        match self {
            PropertyKeyType::Normal => Value::UnKnown(value),
            PropertyKeyType::Bind => Value::Bind(value.parse::<Bind>().unwrap()),
            PropertyKeyType::Function => Value::Function(value.into()),
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

impl Default for PropertyKeyType {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for PropertyKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            PropertyKeyType::Normal => "normal",
            PropertyKeyType::Bind => "bind",
            PropertyKeyType::Function => "function",
        };
        f.write_str(res)
    }
}

impl From<&str> for PropertyKeyType {
    fn from(value: &str) -> Self {
        match value {
            "" => PropertyKeyType::Normal,
            ":" => PropertyKeyType::Bind,
            "@" => PropertyKeyType::Function,
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
pub struct PropsKey {
    /// property key name
    name: String,
    /// same as function
    /// judge the use place (template|style)
    /// has behave differently
    is_style: bool,
    ty: PropertyKeyType,
}

impl PropsKey {
    pub fn new(name: &str, is_style: bool, ty: PropertyKeyType) -> Self {
        PropsKey {
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
            ty: PropertyKeyType::Normal,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &PropertyKeyType {
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
}

impl Display for PropsKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            PropertyKeyType::Normal => f.write_str(self.name()),
            PropertyKeyType::Bind => {
                if self.is_style {
                    f.write_str(self.name())
                } else {
                    f.write_fmt(format_args!(":{}", self.name()))
                }
            }
            PropertyKeyType::Function => {
                if self.is_style {
                    f.write_str(self.name())
                } else {
                    f.write_fmt(format_args!("@{}", self.name()))
                }
            }
        }
    }
}

pub type Props = Option<HashMap<PropsKey, Value>>;

pub fn props_to_string<'a, F>(props: Props, format: F) -> String
where
    F: FnMut((PropsKey, Value)) -> String,
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
