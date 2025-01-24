use toml_edit::{value, Item};

use crate::token::PluginError;

/// # The category of the plugin
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Network,
    Storage,
    Tool,
    Security,
    System,
    Other(String),
}

impl From<Category> for Item {
    fn from(category: Category) -> Self {
        match category {
            Category::Network => value("network"),
            Category::Storage => value("storage"),
            Category::Tool => value("tool"),
            Category::Security => value("security"),
            Category::System => value("system"),
            Category::Other(s) => value(&s),
        }
    }
}

impl TryFrom<&Item> for Category {
    type Error = PluginError;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        match value.as_str() {
            Some("network") => Ok(Category::Network),
            Some("storage") => Ok(Category::Storage),
            Some("tool") => Ok(Category::Tool),
            Some("security") => Ok(Category::Security),
            Some("system") => Ok(Category::System),
            Some(other) => Ok(Category::Other(other.to_string())),
            None => Err(PluginError::from("Invalid category [plugin.category]")),
        }
    }
}
