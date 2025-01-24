use gen_utils::{
    common::float_to_str,
    error::{Error, ParseError, ParseType},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{fmt::Display, str::FromStr};
use syn::parse_str;

use super::{parse_hex_color, Rgb, Rgba};

/// 16进制颜色
#[derive(Debug, Clone, PartialEq)]
pub struct Hex(pub String);

impl TryFrom<&str> for Hex {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Hex::from_str(value)
    }
}

impl FromStr for Hex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_hex_color(s) {
            Ok((remain, hex)) => {
                if remain.is_empty() {
                    return Ok(Hex(format!("#{}", hex)));
                } else {
                    let mut err = ParseError::new("hex", ParseType::Color("rgba".to_string()));
                    let _ = err.set_other(&format!("remain: {}", remain));
                    return Err(err.into());
                }
            }
            Err(e) => {
                Err(ParseError::new(&e.to_string(), ParseType::Color("hex".to_string())).into())
            }
        }
    }
}

impl From<&Rgb> for Hex {
    fn from(value: &Rgb) -> Self {
        Hex(format!("#{:02x}{:02x}{:02x}FF", value.r, value.g, value.b))
    }
}

impl From<&Rgba> for Hex {
    fn from(value: &Rgba) -> Self {
        Hex(format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            value.r,
            value.g,
            value.b,
            (value.a * 255.0) as u8
        ))
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 将16进制颜色转换为vec4
pub fn hex_to_vec4(hex: &Hex) -> TokenStream {
    fn u8_to_tk(hex: &str, start: usize, end: usize) -> TokenStream {
        parse_str::<TokenStream>(
            float_to_str(u8::from_str_radix(&hex[start..end], 16).unwrap() as f32 / 255.0).as_str(),
        )
        .unwrap()
    }

    // 去掉开头的 '#' 符号
    let hex = hex.0.trim_start_matches('#');

    // 解析 RGB 值
    let r = u8_to_tk(hex, 0, 2);
    let g = u8_to_tk(hex, 2, 4);
    let b = u8_to_tk(hex, 4, 6);
    let a = u8_to_tk(hex, 6, 8);

    quote! {
        vec4(#r, #g, #b, #a)
    }
}

impl ToTokens for Hex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // let s = self.0.as_str();
        // tokens.extend(parse_str::<TokenStream>(s).unwrap());
        tokens.extend(hex_to_vec4(self));
    }
}