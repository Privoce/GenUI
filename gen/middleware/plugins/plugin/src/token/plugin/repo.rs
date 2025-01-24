use toml_edit::{Formatted, InlineTable, Item, Value};

use crate::PluginError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Repo {
    Path(String),
    Git(String),
}

impl From<Repo> for Item {
    fn from(repo: Repo) -> Self {
        let mut table = InlineTable::new();
        match repo {
            Repo::Path(path) => table.insert("path", Value::String(Formatted::new(path))),
            Repo::Git(git) => table.insert("git", Value::String(Formatted::new(git))),
        };
        Item::Value(Value::InlineTable(table))
    }
}

impl TryFrom<&Item> for Repo {
    type Error = PluginError;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value
            .as_inline_table()
            .ok_or_else(|| PluginError::from("Invalid repo"))?;

        if let Some(path) = table.get("path").and_then(|v| v.as_str()) {
            Ok(Repo::Path(path.to_string()))
        } else if let Some(git) = table.get("git").and_then(|v| v.as_str()) {
            Ok(Repo::Git(git.to_string()))
        } else {
            Err(PluginError::from("Invalid repo [plugin.repo]"))
        }
    }
}
