use std::{
    collections::HashMap,
    fmt::Display,
    path::Path,
};

use gen_utils::{common::ToToml, err_from_to, error::Error};
use rssyin::bridger::Import;
use toml_edit::{DocumentMut, Item};

use crate::builtin::prop::{LiveDependency, NavMode, Themes};

#[derive(Debug, Clone)]
pub struct RouterBuilder {
    /// page name
    pub name: String,
    /// router id
    pub id: String,
    /// router mode
    pub mode: NavMode,
    /// default active page
    pub active: String,
    pub tabbar: TabbarBuilder,
    pub bar_pages: HashMap<String, Page>,
    pub nav_pages: HashMap<String, Page>,
}

#[derive(Debug, Clone)]
pub struct TabbarBuilder {
    pub theme: Themes,
    pub active: bool,
    pub bars: HashMap<String, TabbarItem>,
}

impl TryFrom<&Item> for TabbarBuilder{
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let value = value.as_table().ok_or(err_from_to!("toml::Item" => "Table (TabbarBuilder)"))?;
        let theme = value.get("theme").map_or_else(
            || Err(err_from_to!("toml::Table" => "Themes, can not find `theme`")),
            |v| v.as_str().map_or_else(| | Err(err_from_to!("toml::Item" => "str")), |s| s.parse())
        )?;

        let active = value.get("active").map_or_else(
            || Err(err_from_to!("toml::Table" => "bool, can not find `active`")),
            |v| v.as_bool().map_or_else(| | Err(err_from_to!("toml::Item" => "bool")), |s| Ok(s))
        )?;

        let bars = value.get("bars").map_or_else(
            || Err(err_from_to!("toml::Table" => "HashMap<String, TabbarItem>, can not find `bars`")),
            |v| {
                let mut bars = HashMap::new();
                for (k, v) in v.as_table().unwrap() {
                    bars.insert(k.to_string(), v.try_into()?);
                }
                Ok(bars)
            }
        )?;


        Ok(Self { theme, active, bars })
    }
}

#[derive(Debug, Clone)]
pub struct TabbarItem {
    pub icon: LiveDependency,
    pub text: String,
}

impl TryFrom<&Item> for TabbarItem {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let value = value.as_inline_table().ok_or(err_from_to!("toml::Item" => "InlineTable (TabbarItem)"))?;
        let icon = value.get("icon").map_or_else(
            || Err(err_from_to!("toml::Value" => "LiveDependency, can not find `icon`")),
            |v| v.as_str().map_or_else(| | Err(err_from_to!("toml::Item" => "str")), |s| Ok(LiveDependency(s.to_string())))
        )?;

        let text = value.get("text").map_or_else(
            || Err(err_from_to!("toml::Value" => "String, can not find `text`")),
            |v| v.as_str().map_or_else(| | Err(err_from_to!("toml::Item" => "str")), |s| Ok(s.to_string()))
        )?;

        Ok(Self { icon, text })
    }
}

#[derive(Debug, Clone)]
pub enum Page {
    Path(Import),
    Component { path: Import, component: String },
}

impl TryFrom<&Item> for Page {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        value.as_str()
        .map_or_else(|| {
            value.as_inline_table().map_or_else(||{
                Err(err_from_to!("toml::Item" => "str|InlineTable, please check the page format"))
            }, |v| {
                let path = v.get("path").map_or_else(
                    || Err(err_from_to!("toml::InlineTable" => "Import, can not find `path`")),
                    |v| v.as_str().map_or_else(| | Err(err_from_to!("toml::Item" => "str")), |s| {
                        let path = s.parse().map_err(|e: rssyin::error::Error| Error::from(e.to_string()))?;
                        Ok(path)
                    })
                )?;

                let component = v.get("component").map_or_else(
                    || Err(err_from_to!("toml::InlineTable" => "String, can not find `component`")),
                    |v| v.as_str().map_or_else(| | Err(err_from_to!("toml::Item" => "str")), |s| Ok(s.to_string()))
                )?;

                Ok(Self::Component { path, component })
            })
        }, |v| {
            let path = v.parse().map_err(|e: rssyin::error::Error| Error::from(e.to_string()))?;
            Ok(Self::Path(path))
        })
    }
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

impl TryFrom<DocumentMut> for RouterBuilder {
    type Error = Error;

    fn try_from(value: DocumentMut) -> Result<Self, Self::Error> {
        fn from_str(value: &DocumentMut, key: &str) -> Result<String, Error> {
            value
                .get(key)
                .ok_or_else(|| err_from_to!("toml::DocumentMut" => &format!("String, can not find `{}`", key)))
                .and_then(|v| {
                    v.as_str()
                    .map_or_else(|| Err(err_from_to!("toml::Value" => "String")), |s| Ok(s.to_string()))
                       
                })
        }

        let name = from_str(&value, "name")?;
        let id = from_str(&value, "id")?;
        let mode = from_str(&value, "mode")?.parse()?;
        let active = from_str(&value, "active")?;
        let tabbar = value.get("tabbar").map_or_else(
            || Err(err_from_to!("toml::DocumentMut" => "TabbarBuilder, can not find `tabbar`")),
            |v| v.try_into(),
        )?;
        let bar_pages = value.get("bar_pages").map_or_else(
            || Err(err_from_to!("toml::DocumentMut" => "HashMap<String, Page>, can not find `bar_pages`")),
            |v| {
                let mut pages = HashMap::new();
                for (k, v) in v.as_table().unwrap() {
                    pages.insert(k.to_string(), v.try_into()?);
                }
                Ok(pages)
            },
        )?;
        let nav_pages = value.get("nav_pages").map_or_else(
            || Err(err_from_to!("toml::DocumentMut" => "HashMap<String, Page>, can not find `nav_pages`")),
            |v| {
                let mut pages = HashMap::new();
                for (k, v) in v.as_table().unwrap() {
                    pages.insert(k.to_string(), v.try_into()?);
                }
                Ok(pages)
            },
        )?;

        Ok(Self {
            name,
            id,
            mode,
            active,
            tabbar,
            bar_pages,
            nav_pages,
        })
    }
}

impl RouterBuilder {
    pub fn new<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let doc = Self::read(path)?;
        doc.try_into()
    }
}


#[cfg(test)]
mod test_router{
    
    use toml_edit::DocumentMut;

    use super::RouterBuilder;

    #[test]
    fn t(){
        let input = r#"
name = "UiRoot"
id = "app_router"
# History, Switch, History是历史模式，记录页面历史，当使用nav_back()时会一直返回直到没有历史记录
# Switch是切换模式，不记录历史，当使用nav_back()时会直接返回到上一个页面，呈现出两个页面之间的切换效果
mode = "History"
active = "login" # 默认显示的页面


# 配置tabbar，tabbar会和bar_pages中的配置一一对应
[tabbar]
theme = "Dark"
active = true # 是否使用tabbar
[tabbar.bars]
login = {icon = "crate://self/resources/login.svg", text = "Login Page"}
register = {icon = "crate://self/resources/register.svg", text = "Register Page"}

# 配置有两种方式, 一种是直接配置, 一种是通过配置文件引入
# 配置bar页面，bar页面是主要页面，当启用tabbar时，bar页面会显示在tabbar上
[bar_pages]
# 说明Login页面的路径为: crate::views::login, 组件为Login
login = { path = "crate::views::login::*", component = "Login" }
# 说明Register页面的路径为: crate::views::register, 组件为Register
register = "crate::views::register::Register"

# 配置nav页面，nav页面属于次要页面，并不会与tabbar有联系，常使用nav_to()进行跳转，并且nav页面具有header，提供快速返回
[nav_pages]
nav_home = { path = "crate::views::home::*", component = "Home" }
nav_todo = { path = "crate::views::todo::*", component = "Todo" }
nav_about = { path = "crate::views::about::*", component = "About" }
        "#;

        let router = input.parse::<DocumentMut>().unwrap();
        let router = RouterBuilder::try_from(router).unwrap();
        dbg!(router);
    }
}