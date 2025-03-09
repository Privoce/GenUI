use gen_analyzer::value::Value;
use gen_utils::{err_from_to, error::Error};
use quote::ToTokens;
use syn::parse_str;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct I32(pub i32);

impl ToTokens for I32 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let v = self.0.to_string();
        tokens.extend(parse_str::<proc_macro2::TokenStream>(&v).unwrap());
    }
}

impl TryFrom<&Value> for I32 {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::ISize(v) => Ok((*v).into()),
            _ => Err(err_from_to!("Value" => "I32")),
        }
    }
}

impl From<i32> for I32 {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<isize> for I32 {
    fn from(v: isize) -> Self {
        Self(v as i32)
    }
}
