use std::fmt::Display;

use gen_utils::error::{ConvertError, Error};
use toml_edit::{value, Item, Table};

/// Wasm Config
/// ```toml
/// [makepad.wasm]
/// fresh = true
/// port = 8016
/// ```
#[derive(Debug, Clone, Default)]
pub struct WasmConf {
    /// 是否需要在每次Gen更新后重新编译
    pub fresh: bool,
    /// 默认端口 (默认8010)
    pub port: Option<u16>,
}

impl TryFrom<&Item> for WasmConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        if let Some(table) = value.as_table() {
            // [fresh] ------------------------------------------------------------------------------------------------
            let fresh = table
                .get("fresh")
                .and_then(|item| item.as_bool())
                .unwrap_or_default();
            // [port] -------------------------------------------------------------------------------------------------
            let port = table
                .get("port")
                .map(|item| item.as_integer().unwrap_or(8010) as u16);

            return Ok(Self { fresh, port });
        }
        Err(ConvertError::FromTo {
            from: "toml::Item".to_string(),
            to: format!("toml::Table, Invalid: {}", value.to_string()),
        }
        .into())
    }
}

impl From<&WasmConf> for Item {
    fn from(conf: &WasmConf) -> Self {
        let mut table = Table::new();
        table.insert("fresh", value(conf.fresh));
        if let Some(port) = conf.port {
            table.insert("port", value(port as i64));
        }
        Item::Table(table)
    }
}

impl Display for WasmConf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

#[cfg(test)]
mod wasm_conf_test {

    #[test]
    fn to_toml2() {
        let conf = super::WasmConf {
            fresh: false,
            port: None,
        };
        let toml = conf.to_string();
        println!("{}", toml);
    }

    #[test]
    fn to_toml1() {
        let conf = super::WasmConf {
            fresh: true,
            port: Some(8016),
        };
        let toml = conf.to_string();
        println!("{}", toml);
    }
}
