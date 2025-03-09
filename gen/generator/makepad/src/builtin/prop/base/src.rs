use gen_analyzer::value::{Enum, EnumItem, Value};
use gen_utils::{err_from_to, error::Error};
use quote::{quote, ToTokens};

use crate::str_to_tk;

use super::LiveDependency;

#[derive(Clone, Debug, Default)]
pub enum Src {
    #[default]
    None,
    Live(LiveDependency),
    Base64(String),
    Url(String),
    File(String),
}

impl TryFrom<&Value> for Src {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Value::Enum(value) = value {
            value.try_into()
        } else if let Value::Function(func) = value {
            Ok(Src::Live(func.try_into()?))
        } else {
            Err(err_from_to!(
                &value.to_string() => "GenUI Src"
            ))
        }
    }
}

impl TryFrom<&Enum> for Src {
    type Error = Error;

    fn try_from(value: &Enum) -> Result<Self, <Self as TryFrom<&Enum>>::Error> {
        let Enum { field_chain } = value;
        field_chain.try_into()
    }
}

impl TryFrom<&Vec<EnumItem>> for Src {
    type Error = Error;

    fn try_from(value: &Vec<EnumItem>) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            return value.get(0).unwrap().try_into();
        } else if value.len() == 2 {
            let root = value.get(0).unwrap();
            let leaf = value.get(1).unwrap();
            if let EnumItem::Root(root) = root {
                if root == "Src" {
                    return leaf.try_into();
                }
            }
        }
        Err(err_from_to!(
            &format!("{:?}, is longer than allow enum item length", value) => "GenUI Src"
        ))
    }
}

impl TryFrom<&EnumItem> for Src {
    type Error = Error;

    fn try_from(value: &EnumItem) -> Result<Self, Self::Error> {
        if let EnumItem::Leaf(leaf, value) = value {
            if let Some(value) = value {
                return match leaf.as_str() {
                    "Live" => Ok(Src::Live(value.try_into()?)),
                    "Base64" => Ok(Src::Base64(value.as_string()?)),
                    "Url" => Ok(Src::Url(value.as_string()?)),
                    "File" => Ok(Src::File(value.as_string()?)),
                    _ => Err(err_from_to!(leaf => "Src")),
                };
            } else {
                if leaf == "None" {
                    return Ok(Src::None);
                }
            }
        }
        Err(err_from_to!("EnumItem" => "Src"))
    }
}

impl ToTokens for Src {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tk = match self {
            Src::None => {
                quote!(None)
            }
            Src::Live(live_dependency) => {
                let live_dep = live_dependency.to_token_stream();
                quote! (Live(#live_dep))
            }
            Src::Base64(base64) => {
                let base64 = str_to_tk!(base64).unwrap();
                quote!(Base64(#base64))
            }
            Src::Url(url) => {
                let url = str_to_tk!(url).unwrap();
                quote!(Url(#url))
            }
            Src::File(file) => {
                let file = str_to_tk!(file).unwrap();
                quote!(File(#file))
            }
        };

        tokens.extend(tk);
    }
}
