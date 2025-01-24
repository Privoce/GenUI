use toml_edit::{Formatted, Value};

use crate::token::PluginError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    PropMacro,
    AttrMacro,
    DeriveMacro,
}

impl From<Category> for Value {
    fn from(category: Category) -> Self {
        let cat = match category {
            Category::PropMacro => "prop_macro",
            Category::AttrMacro => "attr_macro",
            Category::DeriveMacro => "derive_macro",
        }
        .to_string();

        Value::String(Formatted::new(cat))
    }
}

impl TryFrom<&Value> for Category {
    type Error = PluginError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.as_str() {
            Some("prop_macro") => Ok(Category::PropMacro),
            Some("attr_macro") => Ok(Category::AttrMacro),
            Some("derive_macro") => Ok(Category::DeriveMacro),
            Some(other) => Err(PluginError::from(format!(
                "Invalid category [macros.{}]",
                other
            ))),
            None => Err(PluginError::from("Invalid category [macros]")),
        }
    }
}
