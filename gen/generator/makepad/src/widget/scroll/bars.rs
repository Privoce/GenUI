use std::fmt::Display;

use gen_utils::error::Errors;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Fields, ItemStruct};

use crate::{
    from_struct_to_ptr, props_to_token, ptr_to_token,
    utils::struct_field,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, bool_prop, quote_makepad_widget_struct, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

pub struct ScrollBarsPropPtr(pub ItemStruct);

from_struct_to_ptr! {ScrollBarsPropPtr, "ScrollBars"}

ptr_to_token!(ScrollBarsPropPtr);

#[derive(Debug, Clone, Default)]
pub struct ScrollBarsProps {
    pub show_scroll_x: Option<bool>,
    pub show_scroll_y: Option<bool>,
}

impl DynProps for ScrollBarsProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            "show_scroll_x" => quote_prop(vec!["show_scroll_x"], &value),
            "show_scroll_y" => quote_prop(vec!["show_scroll_y"], &value),
            _ => panic!("cannot match prop in BuiltIn ScrollBars"),
        }
    }
}

impl StaticProps for ScrollBarsProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = ScrollBarsProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v)
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            "show_scroll_x" => self.show_scroll_x(&value),
            "show_scroll_y" => self.show_scroll_y(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop: {}", prop_name);
                } else {
                    panic!("unslolved prop: {}", prop_name);
                }
            }
        };
    }
}

#[allow(dead_code)]
impl ScrollBarsProps {
    fn show_scroll_x(&mut self, value: &gen_parser::Value) -> Result<(), Errors> {
        bool_prop(value, |b| self.show_scroll_x = Some(b))
    }
    fn show_scroll_y(&mut self, value: &gen_parser::Value) -> Result<(), Errors> {
        bool_prop(value, |b| self.show_scroll_y = Some(b))
    }
}

impl Display for ScrollBarsProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = self.show_scroll_x.as_ref() {
            let _ = f.write_fmt(format_args!("show_scroll_x: {}, ", x));
        }
        if let Some(y) = self.show_scroll_y.as_ref() {
            let _ = f.write_fmt(format_args!("show_scroll_y: {}, ", y));
        }
        write!(f, "")
    }
}

props_to_token!(ScrollBarsProps);
