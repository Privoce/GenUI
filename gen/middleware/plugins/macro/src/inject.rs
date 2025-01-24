use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_str, Ident};

pub fn impl_inject_ref(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name = parse_str::<Ident>(&name.to_string().to_uppercase()).unwrap();
    quote! {
        #name.read().unwrap()
    }
    .into()
}

pub fn impl_inject_mut(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name = parse_str::<Ident>(&name.to_string().to_uppercase()).unwrap();
    quote! {
        #name.write().unwrap()
    }
    .into()
}
