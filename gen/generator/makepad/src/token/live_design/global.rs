use crate::{
    builtin::prop::handle_prop_value_static, traits::ToTokensExt,
};
// use gen_converter::ConvertStyle;
// use gen_parser::PropsKey;
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// # generate static style live design
/// ## Rules
/// - `.` in modify will be replaced by `___`
/// - prop in class|id will be converted to `UPPERCASE_MODIFY__UPPERCASE_PROP_KEY = prop_value`
/// ## Example
/// ```
/// <style>
/// @BASIC_COLOR: #fff; // wait impl for GenUI version > v0.1.1
/// @THEME_COLOR: #000;
///
/// .container {
///     width: 100.0;
///     .inner {
///         text: "hello world";
///     }
/// }
/// </style>
/// // after convert ->
/// live_design!{
///     BASIC_COLOR = vec4(1.0, 1.0, 1.0, 1.0);
///     THEME_COLOR = vec4(0.0, 0.0, 0.0, 1.0);
///     CONTAINER__WIDTH = 100.0;
///     CONTAINER__INNER__TEXT = "hello world";
/// }
/// ```
/// more test: see `tests/static_live_design.rs`
/// ## TODO
/// - [ ] import (GenUI version > v0.1.1)
/// - [ ] Bind (GenUI version > v0.1.1)
impl ToTokensExt for Option<ConvertStyle> {
    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        let mut tk = TokenStream::new();
        if let Some(props) = self {
            for (modify, prop_key) in props {
                let modify = handle_modify(modify);

                for (k, v) in prop_key {
                    if !k.is_bind() {
                        let pv = handle_prop_value_static(k, v)?;
                        tk.extend(gen_prop_key(&modify, k, pv)?);
                    }
                }
            }
        }
        Ok(tk)
    }
}

fn gen_prop_key(modify: &str, k: &PropsKey, pv: TokenStream) -> Result<TokenStream, Error> {
    parse_str::<TokenStream>(&format!(
        "{}__{}",
        modify.to_uppercase(),
        k.name().to_uppercase()
    ))
    .map_or_else(
        |e| Err(e.to_string().into()),
        |k| {
            Ok(quote! {
                #k = #pv;
            })
        },
    )
}

fn handle_modify(modify: &str) -> String {
    // split `.` to `___`
    modify.replace(".", "___")
}
