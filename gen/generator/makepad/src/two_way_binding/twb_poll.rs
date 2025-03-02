// #[derive(Default)]
// struct TwoWayBindingPoll{
//     pub on_is_select_change: Option<Box<dyn Fn(&mut Cx, bool)>>,
// }

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, parse_str, Field, ItemStruct};

/// # 双向绑定事件池构建器
/// 用于动态生成组件需要的TwoWayBindingPoll这个结构体
/// - 每一个由GenUI生成的组件都会有一个TwoWayBindingPoll
/// - key: 属性名, value: 属性类型
#[derive(Default, Debug, Clone)]
pub struct TWBPollBuilder(pub HashMap<String, String>);

impl TWBPollBuilder {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn to_item_struct(self) -> ItemStruct {
        let struct_tk = self.to_token_stream();
        parse_quote! {
            #struct_tk
        }
    }
    pub fn field_token_stream() -> Field {
        parse_quote! {
            #[rust]
            twb_poll: TwoWayBindingPoll
        }
    }

    /// set into trait `after_apply_from_doc`
    pub fn init_tk(&self, ident: TokenStream) -> Option<TokenStream> {
        let tk = self.0.iter().fold(TokenStream::new(), |mut tk, (k, _)| {
            let ident = parse_str::<TokenStream>(k).unwrap();
            let on_change_ident =
                parse_str::<TokenStream>(format!("on_{}_change", k).as_str()).unwrap();
            tk.extend(quote! {
                self.twb_poll.#on_change_ident = Some(Box::new(move |cx, new_state| {
                    unsafe {
                        (*c_ptr).#ident = new_state;
                    }
                }));
            });
            tk
        });

        if !tk.is_empty() {
            return Some(quote! {
                let c_ptr = self as *mut #ident;
                #tk
            });
        } else {
            None
        }
    }
    pub fn fields(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }
}

impl ToTokens for TWBPollBuilder {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let fields = self.0.iter().fold(TokenStream::new(), |mut tk, (k, v)| {
            let k = parse_str::<TokenStream>(format!("on_{}_change", k).as_str()).unwrap();
            let v = parse_str::<TokenStream>(v).unwrap();
            tk.extend(quote! {
                pub #k: Option<Box<dyn Fn(&mut Cx, #v)>>,
            });

            tk
        });

        tokens.extend(quote! {
            #[derive(Default)]
            struct TwoWayBindingPoll{
                #fields
            }
        });
    }
}
