use std::path::PathBuf;

use gen_utils::{
    common::fs,
    error::{ConvertError, Error, ParseError, ParseType},
};
use toml_edit::{value, Formatted, InlineTable, Item, Table, Value};

use crate::builtin::{
    prop::Prop,
    widget::nav::window::{default_window_props, WindowProps},
};

#[derive(Debug, Clone)]
pub struct RootConf {
    // pub name: String,
    pub path: PathBuf,
    pub window: Prop<WindowProps>,
}

impl RootConf {
    pub fn is_root<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        self.path == path.as_ref()
    }
}

impl From<PathBuf> for RootConf {
    fn from(value: PathBuf) -> Self {
        Self {
            // name: "UiRoot".to_string(),
            path: value,
            window: default_window_props(),
        }
    }
}

impl From<&str> for RootConf {
    fn from(value: &str) -> Self {
        PathBuf::from(value).into()
    }
}

impl TryFrom<&Item> for RootConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        if let Some(table) = value.as_table() {
            // let name = table
            //     .get("name")
            //     .and_then(|v| v.as_str())
            //     .unwrap_or("UiRoot");
            let path = table.get("path").map_or_else(
                || {
                    Err(Error::from(ParseError::new(
                        "[makepad.root.path]",
                        ParseType::Toml,
                    )))
                },
                |path| {
                    path.as_str()
                        .map(PathBuf::from)
                        .ok_or_else(|| Error::from(ParseError::new("path", ParseType::Toml)))
                },
            )?;
            let window = table
                .get("window")
                .map_or_else(|| Ok(default_window_props()), |v| v.try_into())?;

            return Ok(Self {
                // name: name.to_string(),
                path,
                window,
            });
        }

        Err(ConvertError::FromTo {
            from: "toml::Item".to_string(),
            to: "RootConf".to_string(),
        }
        .into())
    }
}

impl From<&RootConf> for Item {
    fn from(conf: &RootConf) -> Self {
        let mut table = Table::new();

        // table.insert("name", value(&conf.name));
        table.insert("path", value(fs::path_to_str(&conf.path)));
        table.insert("window", window_to_item(&conf.window));

        Item::Table(table)
    }
}

fn window_to_item(props: &Prop<WindowProps>) -> Item {
    let mut table = InlineTable::new();
    for prop in props.0.iter() {
        match prop {
            WindowProps::OsType(gos_type) => {
                table.insert("os_type", gos_type.into());
            }
            WindowProps::DerefWidget(props) => {
                let (k, v) = props.to_toml_item_kv();
                table.insert(&k, v);
            }
            WindowProps::ShowTitle(show) => {
                table.insert("show_title", Value::Boolean(Formatted::new(*show)));
            }
            WindowProps::ShowIcon(show) => {
                table.insert("show_icon", Value::Boolean(Formatted::new(*show)));
            }
            WindowProps::LastMousePos(dvec2) => {
                table.insert("last_mouse_pos", dvec2.into());
            }
            WindowProps::MouseCursorSize(dvec2) => {
                table.insert("mouse_cursor_size", dvec2.into());
            }
            WindowProps::HideCaptionOnFullscreen(hide) => {
                table.insert(
                    "hide_caption_on_fullscreen",
                    Value::Boolean(Formatted::new(*hide)),
                );
            }
            WindowProps::EventKey(key) => {
                table.insert("event_key", Value::Boolean(Formatted::new(*key)));
            }
            WindowProps::WindowSize(window_size) => {
                table.insert("window_size", window_size.into());
            }
        }
    }

    Item::Value(Value::InlineTable(table))
}
