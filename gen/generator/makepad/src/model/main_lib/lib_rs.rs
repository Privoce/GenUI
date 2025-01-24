use std::{collections::HashSet, path::PathBuf};

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// # 生成lib.rs
pub fn create_lib_rs(
    libs: Option<HashSet<PathBuf>>,
    entry: Option<&String>,
    addition: Option<&String>,
) -> TokenStream {
    let mods = if let Some(libs) = libs.as_ref() {
        Some(libs.iter().fold(TokenStream::new(), |mut tk, lib| {
            let lib = parse_str::<TokenStream>(lib.file_stem().unwrap().to_str().unwrap()).unwrap();

            tk.extend(quote! {
                #[allow(unused)] 
                pub mod #lib;
            });

            tk
        }))
    } else {
        None
    };

    let entry = parse_str::<TokenStream>(entry.unwrap_or(&String::from("app"))).unwrap();

    let addition = parse_str::<TokenStream>(addition.unwrap_or(&String::from(""))).unwrap();
    quote! {
        pub use makepad_widgets;
        pub use gen_components;

        pub mod #entry;
        #mods
        #addition
    }
}
