use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Fields, ItemStruct};

use crate::utils::get_attr_from_field_type;

/// ## Implementation of the `Prop` attr macro
/// See [prop]
pub fn impl_attr_prop(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let name = &input_struct.ident;
    let vis = &input_struct.vis;
    let mut struct_attrs = input_struct.attrs;
    struct_attrs.push(parse_quote!(#[derive(Live, Widget)]));

    let mut field_tks = vec![quote! {
        #[deref]
        pub deref_widget: GView,
    }];

    if let Fields::Named(fields) = input_struct.fields {
        for field in fields.named {
            let field_name = field.ident;
            let field_type = field.ty;
            let field_attr = get_attr_from_field_type(&field_type, &field.attrs);
            let field_vis  = field.vis;
            field_tks.push(quote! {
                #field_attr
                #field_vis #field_name: #field_type,
            });
        }
    }
    
    let expanded = quote! {
        #(#struct_attrs)*
        #vis struct #name {
            #(#field_tks)*
        }
    };

    TokenStream::from(expanded)
}
