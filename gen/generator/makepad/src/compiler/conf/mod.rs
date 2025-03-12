mod wasm;
mod router;
// mod mini_test;

use std::{fmt::Display, path::PathBuf, str::FromStr};
pub use router::RouterBuilder;
use gen_utils::{
    common::{string::FixedString, RustDependence},
    compiler::UnderlayerConfImpl,
    error::{ConvertError, Error, ParseError, ParseType},
};

use toml_edit::{value, Item, Table};
use wasm::WasmConf;

use crate::builtin::widget::RootConf;

pub const CONF_FORMAT_SUGGESTION: &str = r#"
## Easy Format for Makepad Config
```toml
[makepad.root]
path = "/path/to/root.gen"
```
## Full Format for Makepad Config
```toml
[makepad]
entry = "app"
[makepad.root]
path = "/path/to/root.gen"
[makepad.dependencies]
makepad-widgets = {path = "/path/to/makepad-widgets"}
[makepad.wasm]
fresh = true
port = 8016
```
"#;

/// # Makepad Config
/// See [FORMAT]
#[derive(Debug, Clone)]
pub struct Config {
    /// entry file name, default is app
    pub entry: Option<String>,
    /// root path of the project
    pub root: RootConf,
    /// rust dependencies in Cargo.toml
    /// it depends on the target
    /// - makepad: makepad-widgets
    /// > **you can add more other dependencies which you need**
    pub dependencies: Option<Vec<RustDependence>>,
    /// use wasm to run ?
    /// makepad wasm
    pub wasm: Option<WasmConf>,
}

impl Config {
    pub fn new(root: PathBuf) -> Self {
        Self {
            entry: None,
            root: root.into(),
            dependencies: None,
            wasm: None,
        }
    }
    pub fn push_dep(&mut self, dep: RustDependence) {
        if let Some(deps) = self.dependencies.as_mut() {
            deps.push(dep);
        } else {
            self.dependencies = Some(vec![dep]);
        }
    }
}

impl UnderlayerConfImpl for Config {
    fn to_item(&self) -> Item {
        self.into()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl From<&Config> for Item {
    fn from(conf: &Config) -> Self {
        let mut table = Table::new();
        if let Some(entry) = conf.entry.as_ref() {
            table.insert("entry", value(entry));
        }
        table.insert("root", (&conf.root).into());
        if let Some(deps) = conf.dependencies.as_ref() {
            let mut dep_table = Table::new();
            for dep in deps {
                let (name, dep_item) = dep.to_table_kv();
                dep_table.insert(&name, dep_item);
            }
            table.insert("dependencies", Item::Table(dep_table));
        }

        if let Some(wasm) = conf.wasm.as_ref() {
            table.insert("wasm", wasm.into());
        }

        // here need to wrap a new table outside key is makepad
        let mut makepad_table = Table::new();
        makepad_table.insert("makepad", Item::Table(table));
        Item::Table(makepad_table)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

impl TryFrom<Option<&Table>> for Config {
    type Error = Error;

    fn try_from(value: Option<&Table>) -> Result<Self, Self::Error> {
        if let Some(table) = value {
            // [entry] ------------------------------------------------------------------------------------------------
            let entry = table
                .get("entry")
                .map(|v| v.as_str().map_or("app", |s| s).to_string());
            // [root] -------------------------------------------------------------------------------------------------
            let root = table.get("root").map_or_else(
                || {
                    Err(Error::Parse(ParseError::new(
                        "can not found root",
                        ParseType::Toml,
                    )))
                },
                |v| v.try_into(),
            )?;
            // [dependencies] ------------------------------------------------------------------------------------------
            let dependencies = if let Some(deps) = table.get("dependencies") {
                let mut rust_deps = vec![];
                for dep in deps.to_string().split_fixed("\n") {
                    rust_deps.push(RustDependence::from_str(&dep)?);
                }
                Some(rust_deps)
            } else {
                None
            };
            // [wasm] --------------------------------------------------------------------------------------------------
            let wasm = if let Some(wasm) = table.get("wasm") {
                Some(WasmConf::try_from(wasm)?)
            } else {
                None
            };

            return Ok(Self {
                entry,
                root,
                dependencies,
                wasm,
            });
        }

        Err(ConvertError::FromTo {
            from: "toml::Item".to_string(),
            to: format!("toml::Table, Invalid: {:?}", value),
        }
        .into())
    }
}

impl TryFrom<&mut Item> for Config {
    type Error = Error;

    fn try_from(value: &mut Item) -> Result<Self, Self::Error> {
        value.as_table().try_into()
    }
}

impl TryFrom<&Item> for Config {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        value.as_table().try_into()
    }
}

#[cfg(test)]
mod test_conf {

    use super::Config;

    #[test]
    fn to_str() {
        let conf = Config {
            entry: Some("app1".to_string()),
            root: "/path/to/root.gen".into(),
            dependencies: Some(vec![
                "makepad-widgets = {path = \"/Users/shengyifei/projects/makepad/makepad/widgets\"}"
                    .parse()
                    .unwrap(),
            ]),
            wasm: None,
        };

        let toml = conf.to_string();
        println!("{}", toml);
    }

    #[test]
    fn deserde() {
        let input = r#"
[makepad]
entry = "app1"
root = "/path/to/root.gen"
[makepad.dependencies]
makepad-widgets = {path = "/path/to/makepad-widgets"}
gen_ui = {path = "/path/to/gen_ui"}
[makepad.wasm]
fresh = true
port = 8016
        "#;

        let table = input.parse::<toml_edit::DocumentMut>().unwrap();

        let config = Config::try_from(&table["makepad"]).unwrap();

        println!("{}", config);
    }
}
