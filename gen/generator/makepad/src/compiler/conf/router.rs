use std::{collections::HashMap, fmt::Display};

use gen_utils::common::ToToml;
use rssyin::bridger::Import;

use crate::builtin::prop::{LiveDependency, NavMode, Themes};

#[derive(Debug, Clone)]
pub struct RouterBuilder {
    /// page name
    name: String,
    /// router id
    id: String,
    /// router mode
    mode: NavMode,
    /// default active page
    active: String,
    tabbar: TabbarBuilder,
    bar_pages: HashMap<String, Page>,
    mav_pages: HashMap<String, Page>,
}

#[derive(Debug, Clone)]
pub struct TabbarBuilder {
    theme: Themes,
    active: bool,
    bars: HashMap<String, TabbarItem>,
}

#[derive(Debug, Clone)]
pub struct TabbarItem {
    icon: LiveDependency,
    text: String,
}

#[derive(Debug, Clone)]
pub enum Page {
    Path(Import),
    Component { path: Import, component: String },
}

impl ToToml for RouterBuilder {
    fn to_toml(&self) -> toml_edit::DocumentMut {
        unreachable!("router builder will not use this method")
    }
}

impl Display for RouterBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}
