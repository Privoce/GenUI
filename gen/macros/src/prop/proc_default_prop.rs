use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprStruct};

pub fn impl_proc_default_prop(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprStruct);
    // 获取结构体的标识符和指定的字段
    let struct_name = &input.path;
    let fields = &input.fields.iter().collect::<Vec<_>>();

    // 为指定的字段进行解构，并保持未指定的字段使用默认值
    let expanded = quote! {
        {
            let mut instance = #struct_name::default();
            
            #(
                instance.#fields = #fields;
            )*

            instance
        }
    };

    TokenStream::from(expanded)
}
