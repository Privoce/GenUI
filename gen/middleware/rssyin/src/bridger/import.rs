use std::str::FromStr;

use crate::analyzer::AnalyzerStr;
use crate::error::Error;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_str;

#[derive(Debug, Clone)]
pub struct Imports(pub Vec<Import>);

impl ToTokens for Imports {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.iter().for_each(|import| {
            tokens.extend(import.to_token_stream());
        });
    }
}

impl FromStr for Imports {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_macro_holder()?;

        Ok(Self(
            s.split(';')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<Import>())
                .collect::<Result<Vec<Import>, Error>>()?,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Import(pub Vec<String>);

impl Import {
    pub fn component(&self) -> Option<TokenStream> {
        self.0.last().and_then(|s| {
            if s != "*" {
                Some(parse_str::<TokenStream>(s).unwrap())
            } else {
                None
            }
        })
    }
}

impl ToTokens for Import {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(parse_str::<TokenStream>(&format!(
            "use {};",
            self.0.join("::")
        )));
    }
}

impl FromStr for Import {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split("::")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
        ))
    }
}
