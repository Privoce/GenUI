// use syn::parse_str;

// pub fn handle(s: &str) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
//     parse_str(s).map_err(|e| gen_utils::error::Error::FromDynError(e.to_string()))
// }

#[macro_export]
macro_rules! str_to_tk {
    ($S: expr) => {
        syn::parse_str::<proc_macro2::TokenStream>($S).map_err(|e| gen_utils::error::Error::FromDynError(e.to_string()))
    };
}
