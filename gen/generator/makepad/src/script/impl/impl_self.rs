use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ImplItem, ItemImpl};

use crate::two_way_binding::default_impl_get_set;

/// ## impl `${Widget}`{}
/// it will add `getter` and `setter` as `GView` (inherits from `GView`)
#[derive(Debug, Clone)]
pub struct ImplSelf(pub ItemImpl);

impl ImplSelf {
    pub fn new(ident: &TokenStream, self_impl: Option<ItemImpl>) -> Self {
        Self(if let Some(self_impl) = self_impl {
            self_impl
        } else {
            let impl_get_set = default_impl_get_set(ident);
            parse_quote! {
                impl #ident {
                    #impl_get_set
                }
            }
        })
    }
    pub fn extend(&mut self, items: Vec<ImplItem>) {
        self.0.items.extend(items);
    }
    pub fn push(&mut self, item: ImplItem) {
        self.0.items.push(item);
    }
    /// do patch , just get current impl_item into patch_impl and replace current ImplSelf
    /// cost self and return new ImplSelf
    pub fn patch(self, mut patch_impl: ItemImpl) -> Self {
        patch_impl.items.extend(self.0.items);
        Self(patch_impl)
    }
}

impl ToTokens for ImplSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
