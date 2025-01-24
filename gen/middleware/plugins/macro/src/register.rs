use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, parse_str, Expr, Ident, Token};
/// ## impl plugin macro
/// ### format
/// #### call function
/// ```rust
/// plugin!{
///     http: HttpPublisher => http_init()
/// }
/// ```
/// #### expression
/// ```rust
/// plugin!{
///     http: HttpPublisher => {
///         let mut publisher = HttpPublisher::new();
///         // ...
///         publisher  
///     }
/// }
/// ```
pub fn impl_plugin(input: TokenStream) -> TokenStream {
    let PluginPropMacro { name, ty, init } = parse_macro_input!(input as PluginPropMacro);

    // transform name to uppercase all characters
    let name = parse_str::<Ident>(&name.to_string().to_uppercase()).unwrap();

    quote! {
        lazy_static::lazy_static!{
            pub static ref #name: std::sync::RwLock<#ty> = std::sync::RwLock::new(#init);
        }
    }
    .into()
}

struct PluginPropMacro {
    name: Ident,
    ty: Ident,
    init: Expr,
}

impl Parse for PluginPropMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse()?;
        input.parse::<Token![=>]>()?;
        let init = input.parse()?;
        Ok(PluginPropMacro { name, ty, init })
    }
}
