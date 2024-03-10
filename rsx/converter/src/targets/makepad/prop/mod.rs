mod align;
mod bg;
mod clip;
mod display;
mod event;
mod flow;
mod link;
mod margin;
mod optimize;
mod padding;
mod position;
mod scroll;
mod size;
mod spacing;
mod text;

pub use align::*;
pub use bg::*;
pub use clip::*;
pub use display::*;
pub use event::*;
pub use flow::*;
pub use link::*;
pub use margin::*;
pub use optimize::*;
pub use padding::*;
pub use position::*;
pub use scroll::*;
pub use size::*;
pub use spacing::*;
pub use text::*;

use std::fmt::Display;

use parser::{PropsKey, Value};

use crate::error::Errors;

use super::{
    button,
    value::{MakepadPropValue, Size},
    view, window,
};

#[derive(Debug, PartialEq, Clone)]
pub enum PropRole {
    Normal(String, MakepadPropValue),
    Bind(String),
    Function,
    // this means: the current prop is id or class which can link to style properties  (class)
    Context(Vec<String>),
    // same as Context, but only have one (id)
    Special(String),
}

impl PropRole {
    pub fn normal(k: &str, v: MakepadPropValue) -> Self {
        PropRole::Normal(k.to_string(), v)
    }
    pub fn is_special(&self) -> bool {
        matches!(self, PropRole::Special(_))
    }
    pub fn is_context(&self) -> bool {
        matches!(self, PropRole::Context(_))
    }
    pub fn is_link_stylesheet(&self) -> bool {
        self.is_special() || self.is_context()
    }
    /// consume self to String
    pub fn to_special(self) -> String {
        match self {
            PropRole::Special(s) => s,
            _ => panic!("Only PropRole::Special can use this function!"),
        }
    }
    pub fn to_context(self) -> Vec<String> {
        match self {
            PropRole::Context(s) => s,
            _ => panic!("Only PropRole::Context can use this function!"),
        }
    }
}

/// Match properties based on the existing components in the current makepad widgets
/// - &str: tag_name
/// - (PropsKey,Value)
///     - PropsKey: style property name
///     - Value: style property value
///
/// match tag_name and use special handle functions
impl TryFrom<(&str, (&PropsKey, &Value))> for PropRole {
    type Error = Errors;

    fn try_from(value: (&str, (&PropsKey, &Value))) -> Result<Self, Self::Error> {
        // let k = value.0.name();
        match value.0 {
            "Window" => window(value.1 .0, value.1 .1),
            "Button" => button(value.1 .0, value.1 .1),
            "View" => view(value.1 .0, value.1 .1),
            _ => Err(Errors::UnMatchedWidget),
        }
    }
}

impl TryFrom<(&String, (&PropsKey, &Value))> for PropRole {
    type Error = Errors;

    fn try_from(value: (&String, (&PropsKey, &Value))) -> Result<Self, Self::Error> {
        (value.0.as_str(), value.1).try_into()
    }
}

impl Display for PropRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropRole::Normal(k, v) => f.write_fmt(format_args!("{}: {}, ", k, v.to_string())),
            PropRole::Bind(k) => todo!(),
            PropRole::Function => todo!(),
            PropRole::Context(c) => todo!(),
            PropRole::Special(s) => f.write_str(s),
        }
    }
}