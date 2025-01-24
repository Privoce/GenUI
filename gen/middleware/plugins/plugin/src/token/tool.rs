use proc_macro2::TokenStream;
use syn::{parse2, parse_str};

pub fn parse_quote_str(input: &str) -> syn::Stmt {
    let tk = parse_str::<TokenStream>(input).unwrap();
    parse2(tk).unwrap()
}
