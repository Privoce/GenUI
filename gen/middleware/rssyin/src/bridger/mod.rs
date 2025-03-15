mod import;
mod lifecycle;

pub use import::{Import, Imports};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug)]
pub struct ScriptBridger {
    pub imports: Option<Imports>,
    pub component: Option<syn::ItemStruct>,
    /// default impl
    pub instance: Option<syn::ItemImpl>,
    pub events: Option<Vec<syn::ItemEnum>>,
    pub impl_component: Option<syn::ItemImpl>,
    /// prop struct|enum which use `#[prop(true)] or #[prop(false)]`
    pub props: Option<Vec<PropItem>>,
    // 非追踪部分
    pub others: Vec<syn::Stmt>,
}

impl ToTokens for ScriptBridger {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(imports) = &self.imports {
            imports.to_tokens(tokens);
        }
        if let Some(component) = &self.component {
            component.to_tokens(tokens);
        }
        if let Some(instance) = &self.instance {
            instance.to_tokens(tokens);
        }
        if let Some(event) = &self.events {
            tokens.extend(quote! {
                #(#event)*
            });
        }
        if let Some(impl_component) = &self.impl_component {
            impl_component.to_tokens(tokens);
        }
        let others = &self.others;
        tokens.extend(quote! {
            #(#others)*
        });
    }
}

#[derive(Debug)]
pub enum PropItem {
    Struct(syn::ItemStruct),
    Enum(syn::ItemEnum),
}
