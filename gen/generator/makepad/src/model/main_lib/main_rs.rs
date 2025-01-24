use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// 创建main.rs文件, 需要传入项目名
pub fn create_main_rs(name: &str) -> TokenStream{
    let name = parse_str::<TokenStream>(name).unwrap();
    quote!{
        fn main(){
            #name::app::app_main()
        }        
    }
}