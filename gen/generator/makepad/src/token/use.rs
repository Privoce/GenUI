use proc_macro2::TokenStream;
use quote::quote;

pub fn use_default_all() -> TokenStream{
    quote! {
        use makepad_widgets::*;
        use gen_components::*;
    }
}

pub fn use_makepad_widgets() -> TokenStream{
    quote! {
        use makepad_widgets::*;
    }
}

pub fn use_crate_all() -> TokenStream{
    quote! {
        use crate::*;
    }
}