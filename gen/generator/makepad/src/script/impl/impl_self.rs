use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ImplItem, ImplItemFn, ItemImpl};

use crate::two_way_binding::default_impl_get_set;

/// ## impl `${Widget}`{}
/// it will add `getter` and `setter` as `GView` (inherits from `GView`)
#[derive(Debug, Clone)]
pub struct ImplSelf(pub ItemImpl);

impl ImplSelf {
    pub fn new(ident: &TokenStream, self_impl: Option<ItemImpl>) -> Self {
        Self(if let Some(mut self_impl) = self_impl {
            self_impl.attrs.push(parse_quote! {
                #[allow(unused)]
            });
            self_impl
        } else {
            let impl_get_set = default_impl_get_set(ident);
            parse_quote! {
                #[allow(unused)]
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
    /// get the impl fn item by name
    pub fn get_mut_fn(&mut self, fn_name: &str) -> Option<&mut ImplItemFn> {
        self.0.items.iter_mut().find_map(|item| {
            if let ImplItem::Fn(item_fn) = item {
                if item_fn.sig.ident.to_string() == fn_name {
                    return Some(item_fn);
                }
            }
            None
        })
    }
}

impl ToTokens for ImplSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
