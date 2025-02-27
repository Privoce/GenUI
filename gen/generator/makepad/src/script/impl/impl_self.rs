use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ImplItem, ItemImpl};

/// Vec<ItemFn>
#[derive(Debug, Clone)]
pub struct ImplSelf(pub ItemImpl);

impl ImplSelf {
    pub fn new(ident: &TokenStream, self_impl: Option<ItemImpl>) -> Self {
        Self(if let Some(self_impl) = self_impl {
            self_impl
        } else {
            parse_quote! {
                impl #ident {}
            }
        })
    }
    pub fn extend(&mut self, items: Vec<ImplItem>) {
        self.0.items.extend(items);
    }
}

impl ToTokens for ImplSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
