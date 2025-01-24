mod category;
mod repo;

pub use category::*;
pub use repo::*;
use toml_edit::{value, Array, Table};

use super::PluginError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Plugin {
    /// The name of the plugin (must be unique)
    pub name: String,
    /// the category of the plugin
    pub category: Category,
    /// repo of the plugin
    pub repo: Repo,
    pub authors: Option<Vec<String>>,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
}

impl From<Plugin> for Table {
    fn from(plugin: Plugin) -> Self {
        let mut table = Table::new();
        table.insert("name", value(&plugin.name));
        if let Some(author) = plugin.authors {
            let mut arr = Array::new();
            for a in author {
                arr.push(&a);
            }
            table.insert("authors", value(arr));
        }
        table.insert("version", value(&plugin.version));
        if let Some(description) = plugin.description {
            table.insert("description", value(&description));
        }
        if let Some(license) = plugin.license {
            table.insert("license", value(&license));
        }
        table.insert("category", plugin.category.into());
        table.insert("repo", plugin.repo.into());

        table
    }
}

impl TryFrom<&Table> for Plugin {
    type Error = PluginError;

    fn try_from(value: &Table) -> Result<Self, Self::Error> {
        let name = value
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PluginError::from("Invalid name [plugin.name]"))?;
        let category = value
            .get("category")
            .ok_or_else(|| PluginError::from("Invalid category [plugin.category]"))?;
        let repo = value
            .get("repo")
            .ok_or_else(|| PluginError::from("Invalid repo [plugin.repo]"))?;
        let authors = value
            .get("authors")
            .and_then(|v| v.as_array())
            .map_or_else(
                || Ok(None),
                |v| {
                    let mut authors = vec![];
                    for a in v.iter() {
                        if let Some(a) = a.as_str() {
                            authors.push(a.to_string());
                        } else {
                            return Err(PluginError::from("Invalid authors [plugin.authors]"));
                        }
                    }
                    Ok(Some(authors))
                },
            )?;

        let version = value
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PluginError::from("Invalid version [plugin.version]"))?
            .to_string();
        let description = value
            .get("description")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());
        let license = value
            .get("license")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string());

        Ok(Plugin {
            name: name.to_string(),
            category: Category::try_from(category)?,
            repo: Repo::try_from(repo)?,
            authors,
            version,
            description,
            license,
        })
    }
}

impl From<Plugin> for toml_edit::Item {
    fn from(value: Plugin) -> Self {
        toml_edit::Item::Table(value.into())
    }
}
