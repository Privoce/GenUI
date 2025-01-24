use proc_macro2::TokenStream;

use crate::{common::Source, error::Error};

pub trait ToRs {
   fn content(&self) -> Result<TokenStream, Error>;
   fn source(&self) -> Option<&Source>;
}