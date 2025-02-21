mod import;
mod lifecycle;


pub use import::{Import, Imports};
use lifecycle::LifeCycle;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug)]
pub struct ScriptBridger {
    pub imports: Option<Imports>,
    pub prop: Option<syn::ItemStruct>,
    /// default impl
    pub instance: Option<syn::ItemImpl>,
    pub event: Option<syn::ItemEnum>,
    // lifecycles: LifeCycle,
    pub impl_prop: Option<syn::ItemImpl>,
    // 非追踪部分
    pub others: TokenStream,
}

impl ToTokens for ScriptBridger{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(imports) = &self.imports {
            imports.to_tokens(tokens);
        }
        if let Some(prop) = &self.prop {
            prop.to_tokens(tokens);
        }
        if let Some(instance) = &self.instance {
            instance.to_tokens(tokens);
        }
        if let Some(event) = &self.event {
            event.to_tokens(tokens);
        }
        if let Some(impl_prop) = &self.impl_prop {
            impl_prop.to_tokens(tokens);
        }
        self.others.to_tokens(tokens);
    }
}