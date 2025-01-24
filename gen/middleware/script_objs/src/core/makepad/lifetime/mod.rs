pub use app_main::AppLifeTime;
use quote::ToTokens;
use syn::ItemFn;
pub use widget::WidgetLifeTime;

use crate::error::{AttrMacroError, SCResult};

mod app_main;
pub mod network;
mod widget;

#[derive(Debug, Clone)]
pub enum LifeTime {
    AppMain(AppLifeTime),
    Widget(WidgetLifeTime),
}

impl LifeTime {
    pub fn push(&mut self, item: &ItemFn) -> SCResult<()>{
        match self {
            LifeTime::AppMain(app) => app.push(item),
            LifeTime::Widget(widget) => widget.push(item),
        }
    }
}

impl TryFrom<&ItemFn> for LifeTime {
    type Error = crate::error::Error;

    fn try_from(value: &ItemFn) -> Result<Self, Self::Error> {
        // 这里只允许一个属性宏
        if value.attrs.is_empty() {
            return Err(AttrMacroError::NoLifeTimeMacro.into());
        }

        if let Some(attr) = value.attrs[0].path().get_ident() {
            // 先尝试让AppMain来解析再尝试Widget，若都不行则返回None
            return AppLifeTime::try_from((attr, value)).map_or_else(
                |_| WidgetLifeTime::try_from((attr, value)).map(|widget| widget.into()),
                |app| Ok(app.into()),
            );
        }

        Err(AttrMacroError::NoLifeTimeMacro.into())
    }
}

impl From<AppLifeTime> for LifeTime {
    fn from(app: AppLifeTime) -> Self {
        LifeTime::AppMain(app)
    }
}

impl From<WidgetLifeTime> for LifeTime {
    fn from(widget: WidgetLifeTime) -> Self {
        LifeTime::Widget(widget)
    }
}

impl ToTokens for LifeTime {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            LifeTime::AppMain(app) => app.to_tokens(tokens),
            LifeTime::Widget(widget) => widget.to_tokens(tokens),
        }
    }
}
