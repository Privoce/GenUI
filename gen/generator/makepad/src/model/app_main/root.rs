use gen_utils::common::Source;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// # RootRef
/// 根引用，存在于AppMain中，用于提供根引用的信息
#[derive(Debug, Clone, Default)]
pub struct RootRef {
    /// 实例源文件
    pub source: Option<Source>,
    /// 实例名, 可以没有，默认为`ui_root`
    pub name: Option<String>,
}

impl RootRef {
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "ui_root".to_string())
    }
    pub fn import_root(&self) -> TokenStream {
        // remove header two path

        let to = self
            .source
            .as_ref()
            .unwrap()
            .to
            .to_path_buf()
            .with_extension("")
            .components()
            .skip(2)
            .map(|item| item.as_os_str().to_str().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("::");

        let tk = parse_str::<TokenStream>(&to).unwrap();

        quote! {
            use crate::#tk::*;
        }
    }
}
