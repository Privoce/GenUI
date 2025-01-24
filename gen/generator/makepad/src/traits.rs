use gen_utils::error::Error;
use proc_macro2::TokenStream;

pub trait ToTokensExt {
    fn to_token_stream(&self) -> Result<TokenStream, Error>;
}
