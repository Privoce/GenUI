use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemEnum};

pub fn impl_attr_event(input: TokenStream) -> TokenStream {
    let input_enum = parse_macro_input!(input as ItemEnum);
    let enum_vis = &input_enum.vis;
    let enum_name = &input_enum.ident;
    let mut enum_derives = input_enum.attrs;
    enum_derives.push(parse_quote!(#[derive(DefaultNone)]));
   

    let variants = input_enum.variants.iter().map(|var| {
        let variant_name = &var.ident;
        let fields = &var.fields;
        // 保持原始变体的属性
        let attrs = &var.attrs;
        quote! {
            #(#attrs)*
            #variant_name #fields
        }
    });

    let expanded = quote! {
        #(#enum_derives)*
        #enum_vis enum #enum_name {
            #(#variants,)*
            None
        }
    };

    TokenStream::from(expanded)
}
