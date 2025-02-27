use gen_analyzer::PropComponent;
use gen_utils::{common::camel_to_snake, error::Error};
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{builtin::BuiltinWidget, str_to_tk};

pub trait ToTokensExt {
    fn to_token_stream(&self) -> Result<TokenStream, Error>;
}

pub trait MakepadExtComponent {
    fn name(&self) -> String;
    fn snake_name(name: &str) -> String {
        BuiltinWidget::is_built_in(name).map_or_else(
            |_| camel_to_snake(name),
            |builtin| builtin.snake_name().to_string(),
        )
    }
}

impl MakepadExtComponent for PropComponent {
    fn name(&self) -> String {
        Self::snake_name(&self.name)
    }
}

impl ToTokensExt for gen_analyzer::value::Ident {
    fn to_token_stream(&self) -> Result<TokenStream, Error> {
        str_to_tk!(&self.to_string())
    }
}
