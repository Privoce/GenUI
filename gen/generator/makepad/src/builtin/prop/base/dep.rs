use std::fmt::Display;

use gen_analyzer::value::{Function, Value};
use gen_utils::{err_from_to, error::Error};
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct LiveDependency(pub String);

impl TryFrom<&Function> for LiveDependency {
    type Error = Error;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        let Function { name, params, .. } = value;

        // only one param
        if let Some(params) = params {
            if params.len() == 1 && name == "dep" {
                if let Value::String(s) = params.get(0).unwrap() {
                    return Ok(LiveDependency(s.to_string()));
                }
            }
        }

        return Err(err_from_to!(
            "Function" => "LiveDependency, dep fn need one param (Into<pathbuf>)"
        ));
    }
}

impl TryFrom<&Value> for LiveDependency {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Value::Function(f) = value {
            return f.try_into();
        } else {
            return Err(err_from_to!("Value" => "LiveDependency"));
        }
    }
}

impl Display for LiveDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dep(\"{}\")", self.0)
    }
}

impl ToTokens for LiveDependency {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(syn::parse_str::<proc_macro2::TokenStream>(&self.to_string()).unwrap());
    }
}
