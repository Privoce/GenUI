use std::{fmt::Display, str::FromStr};

use gen_utils::{
    common::float_to_str,
    error::{ConvertError, Error, ParseError},
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_str;

/// 百分比
/// 语法: `percentage(%)`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(pub f32);

impl Percentage {
    /// 修正百分比
    pub fn fix(&mut self, start: f32, end: f32, index: usize, len: usize) -> () {
        let step = (end - start) / (len as f32);
        self.0 = step * (index as f32);
    }
}

impl FromStr for Percentage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 匹配百分比语法，输入类似: 11.5%
        if s.ends_with("%") {
            let s = s.trim_end_matches("%");
            let p = s.parse::<f32>().map_err(|_| {
                Error::Convert(ConvertError::FromTo {
                    from: s.to_string(),
                    to: "f32".to_string(),
                })
            })?;
            Ok(Percentage(p))
        } else {
            let mut err = ParseError::other(s, "Percentage");
            let _ = err.set_other("percentage need `%` as end");
            Err(err.into())
        }
    }
}

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl ToTokens for Percentage {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let s = float_to_str(self.0 / 100.0);
        tokens.extend(parse_str::<TokenStream>(&s).unwrap());
    }
}
