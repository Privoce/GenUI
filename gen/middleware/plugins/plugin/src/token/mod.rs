mod macros;
mod plugin;
mod tool;

use std::{hash::Hash, path::PathBuf, str::FromStr};

use gen_utils::common::RustDependence;
pub use macros::{Category as MacroCategory, Macro};
pub use plugin::{Category as PluginCategory, Plugin, Repo};

use proc_macro2::TokenStream;
use toml_edit::{DocumentMut, Item, Table};
pub use tool::*;

pub type PluginError = Box<dyn std::error::Error>;

#[derive(Debug, Clone)]
pub struct Token {
    pub plugin: Plugin,
    pub macros: Vec<Macro>,
    /// The dependencies of the plugin(rust crate)
    pub dependencies: Vec<RustDependence>,
}

impl Token {
    pub fn to_dyn_code(&self) -> TokenStream {
        self.macros.iter().fold(TokenStream::new(), |mut acc, mac| {
            acc.extend(mac.to_expr());
            acc
        })
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.plugin == other.plugin
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.plugin.hash(state);
    }
}

impl From<Token> for DocumentMut {
    fn from(tk: Token) -> Self {
        let mut doc = DocumentMut::new();
        doc.insert("plugin", tk.plugin.into());

        if !tk.macros.is_empty() {
            let mut table = Table::new();

            for mac in tk.macros {
                let (key, item) = mac.to_table();
                table.insert(&key, item);
            }

            doc.insert("macros", Item::Table(table));
        }

        let deps = RustDependence::vec_to_item(&tk.dependencies);

        doc.insert("dependencies", deps);

        doc
    }
}

impl TryFrom<DocumentMut> for Token {
    type Error = PluginError;

    fn try_from(value: DocumentMut) -> Result<Self, Self::Error> {
        let plugin = value
            .get("plugin")
            .and_then(|v| v.as_table())
            .ok_or_else(|| PluginError::from("Invalid plugin [plugin]"))?;

        let macros: Result<Vec<Macro>, PluginError> =
            if let Some(table) = value.get("macros").and_then(|v| v.as_table()) {
                let mut macros = vec![];
                for (key, item) in table.iter() {
                    let item = item
                        .as_inline_table()
                        .ok_or_else(|| PluginError::from("Invalid macros [macros] item"))?;
                    macros.push(Macro::try_from((key, item))?);
                }

                Ok(macros)
            } else {
                Ok(vec![])
            };

        let dependencies = value
            .get("dependencies")
            .map_or_else(|| Ok(vec![]), |deps| RustDependence::from_item(deps))?;

        Ok(Token {
            plugin: plugin.try_into()?,
            macros: macros?,
            dependencies,
        })
    }
}

impl TryFrom<&PathBuf> for Token {
    type Error = PluginError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let path = value.join("token.toml");
        let doc: DocumentMut = std::fs::read_to_string(path)?.parse()?;
        doc.try_into()
    }
}

impl FromStr for Token {
    type Err = PluginError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let doc: DocumentMut = s.parse()?;
        doc.try_into()
    }
}

#[cfg(test)]
mod test {

    use gen_utils::common::RustDependence;
    use quote::ToTokens;
    use syn::parse_str;
    use toml_edit::DocumentMut;

    use crate::token::plugin::Repo;

    use super::{Macro, MacroCategory, Plugin, PluginCategory, Token};

    #[test]
    fn parse() {
        let s = r#"
        [plugin]
        name = "hello"
        category = "network"
        
        [macros]
        http_get = {category = "prop_macro", stmts = "let _ = http_get_tk_stmt(expr_macro);"}
        "#;

        let doc: DocumentMut = s.parse().unwrap();
        // dbg!(doc);
        let tk: Token = doc.try_into().unwrap();

        dbg!(tk);
    }

    #[test]
    fn token() {
        let s = r#"
        {
            if mac.ident == "http_get"{
                mac.tokens.push_str(", cx");
            }
        }
        "#;
        let tk = Token {
            plugin: Plugin {
                name: "gen_makepad_http".to_string(),
                category: PluginCategory::Network,
                repo: Repo::Git(
                    "https://github.com/Privoce/genui_plugins/tree/main/projects/gen_makepad_http"
                        .to_string(),
                ),
                authors: Some(vec!["syf<syf20020816@outlook.com>".to_string()]),
                version: "0.1.0".to_string(),
                description: Some("http support for makepad".to_string()),
                license: None,
            },
            macros: vec![Macro {
                name: "http_get".to_string(),
                category: MacroCategory::PropMacro,
                stmts: parse_str::<syn::Block>(s).unwrap().stmts,
            }],
            dependencies: vec![RustDependence {
                name: "serde".to_string(),
                version: Some("0.1.0".to_string()),
                features: None,
                default_features: None,
                ty: gen_utils::common::DepType::Crate,
            }],
        };

        dbg!(tk.macros[0].to_expr().to_token_stream().to_string());

        let doc: DocumentMut = tk.into();
        println!("{}", doc);
    }

    #[test]
    fn t() {
        let input = r#"
        [macros]
        a = {name = "hello"}
        "#;

        let doc = input.parse::<toml_edit::DocumentMut>().unwrap();

        dbg!(doc);
    }
}
