use crate::{builtin::BuiltinWidget, token::ToLiveDesign};
use gen_converter::ConvertStyle;
use gen_parser::Props;
use gen_utils::error::Error;
use proc_macro2::TokenStream;

use super::DefineWidget;

/// # WidgetType
/// Widget的类型, 可能是内置组件或者自定义组件
#[derive(Debug, Clone)]
pub enum WidgetType {
    Builtin(BuiltinWidget),
    Define(DefineWidget),
    Global(Option<ConvertStyle>),
}

impl ToLiveDesign for WidgetType {
    fn name(&self) -> TokenStream {
        match self {
            WidgetType::Builtin(builtin_widget) => builtin_widget.name(),
            WidgetType::Define(define_widget) => define_widget.name(),
            WidgetType::Global(_hash_map) => {
                panic!("Global WidgetType is not support name! If you see this error, please check your ast!")
            }
        }
    }

    fn props(&self) -> Option<TokenStream> {
        match self {
            WidgetType::Builtin(builtin_widget) => builtin_widget.props(),
            WidgetType::Define(define_widget) => define_widget.props(),
            WidgetType::Global(_hash_map) => {
                panic!("Global WidgetType is not support props! If you see this error, please check your ast!")
            }
        }
    }
}

impl WidgetType {
    pub fn is_define(&self) -> bool {
        matches!(self, WidgetType::Define(_))
    }
    pub fn root_name(&self) -> TokenStream {
        match self {
            WidgetType::Define(define_widget) => define_widget.root_name(),
            _ => {
                panic!("only DefineWidget has root_name!")
            }
        }
    }
    pub fn snake_name(&self) -> String {
        match self {
            WidgetType::Define(define_widget) => define_widget.snake_name(),
            WidgetType::Builtin(builtin_widget) => builtin_widget.snake_name(),
            WidgetType::Global(_hash_map) => {
                panic!("Global WidgetType is not support name! If you see this error, please check your ast!")
            }
        }
    }
}

impl TryFrom<(String, Props, bool)> for WidgetType {
    type Error = Error;

    fn try_from(value: (String, Props, bool)) -> Result<Self, Self::Error> {
        match BuiltinWidget::try_from(value.clone()) {
            Ok(w) => {
                return Ok(WidgetType::Builtin(w));
            }
            Err(e) => {
                if e.is_runtime() {
                    return Err(e);
                }

                return Ok(WidgetType::Define(value.try_into()?));
            }
        }
    }
}
