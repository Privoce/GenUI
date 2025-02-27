mod impl_self;
mod impl_self_ref;
mod impl_traits;


pub use impl_self::*;
pub use impl_self_ref::*;
pub use impl_traits::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, ItemImpl};

use crate::{model::Traits, two_way_binding::TWBPollBuilder};

#[derive( Debug, Clone)]
pub struct Impls{
    pub self_impl: ImplSelf,
    pub self_ref_impl: ImplSelfRef,
    pub traits_impl: ImplTraits,
}


impl Impls {
    pub fn default(ident: &TokenStream,self_impl: Option<ItemImpl>) -> Self{
        Self{
            self_impl: ImplSelf::new(ident, self_impl),
            self_ref_impl: ImplSelfRef::default(),
            traits_impl: ImplTraits::default(),
        }
    }
    pub fn traits(&mut self) -> &mut Traits {
        &mut self.traits_impl.0
    }
    pub fn to_token_stream(&self, ident: &TokenStream,twb_poll: Option<&TWBPollBuilder>) -> TokenStream {
        let ident_ref = parse_str::<TokenStream>(format!("{}Ref", ident.to_string()).as_str()).unwrap();
        let self_impl = self.self_impl.to_token_stream();
        let self_ref_impl = self.self_ref_impl.to_token_stream();
        let traits_impl = self.traits_impl.to_token_stream(ident, twb_poll);

        quote! {
            #self_impl
            
            #[allow(unused)]
            impl #ident_ref{
                #self_ref_impl
            }   

            #traits_impl
        }
    }
}