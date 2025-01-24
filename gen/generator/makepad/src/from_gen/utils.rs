use syn::parse_str;

pub fn handle(s: &str) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
    parse_str(s).map_err(|e| gen_utils::error::Error::FromDynError(e.to_string()))
}
