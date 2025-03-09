use gen_utils::{common::format_float, err_from_to, error::Error};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_str;
use toml_edit::Formatted;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct F64(pub f64);

impl ToTokens for F64 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let v = format_float(self.0);
        tokens.extend(parse_str::<TokenStream>(&v).unwrap());
    }
}

impl From<f64> for F64 {
    fn from(v: f64) -> Self {
        Self(v)
    }
}

impl From<f32> for F64 {
    fn from(v: f32) -> Self {
        Self(v as f64)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct F32(pub f32);

impl ToTokens for F32 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let v = format_float(self.0 as f64);
        tokens.extend(parse_str::<TokenStream>(&v).unwrap());
    }
}

impl From<f32> for F32 {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

impl From<&F32> for toml_edit::Value {
    fn from(value: &F32) -> Self {
        toml_edit::Value::Float(Formatted::new(value.0 as f64))
    }
}

impl From<&F64> for toml_edit::Value {
    fn from(value: &F64) -> Self {
        toml_edit::Value::Float(Formatted::new(value.0))
    }
}

impl TryFrom<&toml_edit::Value> for F64 {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_float().map_or_else(
            || Err(Error::from(err_from_to!("toml_edit::Item" => "F64"))),
            |v| Ok(F64(v)),
        )
    }
}

impl TryFrom<&toml_edit::Value> for F32 {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_float().map_or_else(
            || Err(Error::from(err_from_to!("toml_edit::Item" => "F32"))),
            |v| Ok(F32(v as f32)),
        )
    }
}
