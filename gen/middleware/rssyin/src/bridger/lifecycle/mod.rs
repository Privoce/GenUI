pub use app_main::AppLifeCycle;
use quote::ToTokens;
use syn::ItemFn;
pub use widget::WidgetLifeCycle;

use crate::error::{AttrMacroError, SCResult};

mod app_main;
pub mod network;
mod widget;

#[derive(Debug, Clone)]
pub enum LifeCycle {
    AppMain(AppLifeCycle),
    Widget(WidgetLifeCycle),
}

impl LifeCycle {
    #[allow(unused)]
    pub fn push(&mut self, item: &ItemFn) -> SCResult<()>{
        match self {
            LifeCycle::AppMain(app) => app.push(item),
            LifeCycle::Widget(widget) => widget.push(item),
        }
    }
}

impl TryFrom<&ItemFn> for LifeCycle {
    type Error = crate::error::Error;

    fn try_from(value: &ItemFn) -> Result<Self, Self::Error> {
        // 这里只允许一个属性宏
        if value.attrs.is_empty() {
            return Err(AttrMacroError::NoLifeCycleMacro.into());
        }

        if let Some(attr) = value.attrs[0].path().get_ident() {
            // 先尝试让AppMain来解析再尝试Widget，若都不行则返回None
            return AppLifeCycle::try_from((attr, value)).map_or_else(
                |_| WidgetLifeCycle::try_from((attr, value)).map(|widget| widget.into()),
                |app| Ok(app.into()),
            );
        }

        Err(AttrMacroError::NoLifeCycleMacro.into())
    }
}

impl From<AppLifeCycle> for LifeCycle {
    fn from(app: AppLifeCycle) -> Self {
        LifeCycle::AppMain(app)
    }
}

impl From<WidgetLifeCycle> for LifeCycle {
    fn from(widget: WidgetLifeCycle) -> Self {
        LifeCycle::Widget(widget)
    }
}

impl ToTokens for LifeCycle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            LifeCycle::AppMain(app) => app.to_tokens(tokens),
            LifeCycle::Widget(widget) => widget.to_tokens(tokens),
        }
    }
}
