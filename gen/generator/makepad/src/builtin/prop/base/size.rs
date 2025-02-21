use std::str::FromStr;

use gen_analyzer::value::{Enum, EnumItem, Value};
use gen_utils::{
    common::format_float,
    error::{ConvertError, Error},
};
use quote::ToTokens;

use crate::builtin::prop::err_from_to;

use super::DVec2;

#[derive(Debug, Clone)]
pub struct WindowSize {
    pub inner_size: DVec2,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            inner_size: DVec2 {
                x: 1080.0,
                y: 720.0,
            },
        }
    }
}

impl ToTokens for WindowSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let inner_size = &self.inner_size;
        tokens.extend(quote::quote! {
            {
                inner_size: #inner_size
            }
        });
    }
}

impl From<&WindowSize> for toml_edit::Value {
    fn from(value: &WindowSize) -> Self {
        let mut table = toml_edit::InlineTable::new();
        table.insert("inner_size", (&value.inner_size).into());
        toml_edit::Value::InlineTable(table)
    }
}

impl TryFrom<&toml_edit::Value> for WindowSize {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        let size = value
            .as_inline_table()
            .ok_or_else(|| err_from_to("toml_edit::Value", "WindowSize"))?;

        let inner_size = size.get("inner_size").map_or_else(
            || DVec2 {
                x: 1080.0,
                y: 720.0,
            },
            |v| v.try_into().unwrap(),
        );

        Ok(Self { inner_size })
    }
}

/// # Makepad Size
/// the size of props
/// - height
/// - width
#[derive(Debug, Clone, Default)]
pub enum Size {
    #[default]
    /// Fill the size of the parent widget
    Fill,
    /// detail size of the current widget
    Fixed(f64),
    /// Fit content
    Fit,
    All,
}

impl From<&Size> for toml_edit::Value {
    fn from(value: &Size) -> Self {
        match value {
            Size::Fill => "Fill".into(),
            Size::Fixed(v) => (*v).into(),
            Size::Fit => "Fit".into(),
            Size::All => "All".into(),
        }
    }
}

impl TryFrom<&toml_edit::Value> for Size {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        if let Some(value) = value.as_str() {
            value.parse()
        } else if let Some(value) = value.as_float() {
            Ok(Size::Fixed(value))
        } else {
            Err(err_from_to("toml_edit::Item", "Size").into())
        }
    }
}

impl FromStr for Size {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fill" => Ok(Size::Fill),
            "All" => Ok(Size::All),
            "Fit" => Ok(Size::Fit),
            _ => Err(ConvertError::FromTo {
                from: s.to_string(),
                to: "Makepad Size".to_string(),
            }
            .into()),
        }
    }
}

impl TryFrom<&Enum> for Size {
    type Error = gen_utils::error::Error;

    fn try_from(value: &Enum) -> Result<Self, <Self as TryFrom<&Enum>>::Error> {
        let Enum { field_chain } = value;
        field_chain.try_into()
    }
}

impl TryFrom<&Vec<EnumItem>> for Size {
    type Error = Error;

    fn try_from(value: &Vec<EnumItem>) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            return value.get(0).unwrap().try_into();
        } else if value.len() == 2 {
            let root = value.get(0).unwrap();
            let leaf = value.get(1).unwrap();
            if let EnumItem::Root(root) = root {
                if root == "Size" {
                    return leaf.try_into();
                }
            }
        }
        Err(err_from_to("EnumItem", "Size").into())
    }
}

impl TryFrom<&EnumItem> for Size {
    type Error = Error;

    fn try_from(value: &EnumItem) -> Result<Self, Self::Error> {
        if let EnumItem::Leaf(leaf, value) = value {
            if leaf == "Fixed" && value.is_some() {
                return Ok(Size::Fixed(value.as_ref().unwrap().as_f64()?));
            } else {
                return leaf.parse();
            }
        }
        Err(err_from_to("EnumItem", "Size").into())
    }
}

impl TryFrom<&Value> for Size {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Value::Enum(value) = value {
            value.try_into()
        } else if let Value::String(value) = value {
            value.parse()
        } else if let Value::Double(value) = value {
            Ok(Size::Fixed(*value))
        } else {
            Err(ConvertError::FromTo {
                from: value.to_string(),
                to: "Makepad Size".to_string(),
            }
            .into())
        }
    }
}

impl ToTokens for Size {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let res = match self {
            Size::Fill => "Fill",
            Size::Fixed(v) => &format_float(*v),
            Size::Fit => "Fit",
            Size::All => "All",
        };
        tokens.extend(proc_macro2::TokenStream::from_str(&res).unwrap());
    }
}
