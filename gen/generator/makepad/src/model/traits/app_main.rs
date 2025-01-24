use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_str;

#[derive(Debug, Clone)]
pub struct AppMainTrait {
    /// handle_event需要接受一个root节点的名字来生成具体的代码，需要使用者在一开始编译器设置时就提供，默认获取`root`
    /// 例如：`self.root.handle_event(cx, event, &mut Scope::empty());`
    pub handle_event: String,
}

impl Default for AppMainTrait {
    fn default() -> Self {
        Self {
            handle_event: "root".to_string(),
        }
    }
}

impl AppMainTrait {
    pub fn handle_event(&mut self, root: &str) -> () {
        self.handle_event = root.to_string();
    }
}
impl ToTokens for AppMainTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root = parse_str::<TokenStream>(&format!(
            "self.{}.handle_event(cx, event, &mut Scope::empty());",
            self.handle_event
        ))
        .unwrap();
        tokens.extend(quote! {
            impl AppMain for App {
                #[allow(unused_variables)]
                fn handle_event(&mut self, cx: &mut Cx, event: &Event){
                    self.match_event(cx, event);
                    #root
                }
            }
        });
    }
}

#[cfg(test)]
mod app_main_trait_tests {
    use super::*;
    #[test]
    fn test_app_main_trait_some() {
        let mut app_main = AppMainTrait::default();
        app_main.handle_event("ui");
        dbg!(&app_main.to_token_stream().to_string());
    }

    #[test]
    fn test_app_main_trait_none() {
        let mut app_main = AppMainTrait::default();
        app_main.handle_event("ui_root");
        dbg!(&app_main.to_token_stream().to_string());
    }
}
