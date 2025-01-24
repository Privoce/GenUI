use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_str, Expr, Ident, Local, Pat};

/// 模仿proc_default_prop!宏的实现
pub fn proc_tk(local: &Local) -> TokenStream {
    // 获取结构体的标识符和所有被重置的字段
    return if let Expr::Macro(expr_macro) = &*local.init.as_ref().unwrap().expr {
        // 第一个是struct的名字，第二个Group是内部的字段 `field: value,` -> `self.field = value;`
        // TokenStream是无法处理的所以我们需要转为TokenTree
        expr_macro
            .mac
            .tokens
            .clone()
            .into_iter()
            .collect::<Vec<TokenTree>>()
            .last()
            .unwrap()
            .to_token_stream()
            .to_string()
            .trim()
            .split(',')
            .fold(TokenStream::new(), |mut tk, item| {
                // 将 : 替换为 =
                let item = parse_str::<TokenStream>(
                    &item
                        .replace(":", "=")
                        .trim_matches(|c| c == '{' || c == '}'),
                )
                .unwrap();
                if !item.is_empty() {
                    tk.extend(quote! {
                        self.#item;
                    });
                }

                tk
            })
    } else {
        panic!("Instance must be a macro")
    };
}

pub fn get_ident(local: &Local) -> &Ident {
    if let Pat::Ident(ident) = &local.pat {
        &ident.ident
    } else {
        panic!("Instance must be a ident")
    }
}
