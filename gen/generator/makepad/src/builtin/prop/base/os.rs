use gen_analyzer::value::{Enum, EnumItem};
use gen_utils::{err_from_to, error::Error};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::str::FromStr;
use syn::parse_str;
use toml_edit::Formatted;

const WINDOWS: &str = "Windows";
const MAC: &str = "Mac";
const LINUX: &str = "Linux";
const OTHER: &str = "Other";

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum GOsType {
    Windows,
    Mac,
    #[default]
    Linux,
    Other,
}

impl From<&GOsType> for toml_edit::Value {
    fn from(ty: &GOsType) -> Self {
        let v = match ty {
            GOsType::Windows => WINDOWS,
            GOsType::Mac => MAC,
            GOsType::Linux => LINUX,
            GOsType::Other => OTHER,
        };

        toml_edit::Value::String(Formatted::new(v.to_string()))
    }
}

impl TryFrom<&toml_edit::Value> for GOsType {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_str().map_or_else(
            || Err(err_from_to!("toml_edit::Value" => "GOsType")),
            |s| s.parse(),
        )
    }
}

impl FromStr for GOsType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            WINDOWS => Ok(GOsType::Windows),
            MAC => Ok(GOsType::Mac),
            LINUX => Ok(GOsType::Linux),
            OTHER => Ok(GOsType::Other),
            _ => Ok(GOsType::Other),
        }
    }
}

impl TryFrom<&Enum> for GOsType {
    type Error = Error;

    fn try_from(value: &Enum) -> Result<Self, Self::Error> {
        let Enum { field_chain } = value;
        let e_len = field_chain.len();
        if e_len > 2 || e_len == 0 {
            return Err(err_from_to!("Enum" => "GOsType"));
        } else if e_len == 1 {
            field_chain.get(0).unwrap().try_into()
        } else {
            // len == 2
            field_chain.try_into()
        }
    }
}

impl TryFrom<&EnumItem> for GOsType {
    type Error = gen_utils::error::Error;

    fn try_from(value: &EnumItem) -> Result<Self, <Self as TryFrom<&EnumItem>>::Error> {
        match value {
            EnumItem::Leaf(s, _) => Ok(GOsType::from_str(s)?),
            _ => Err(err_from_to!("EnumItem" => "GOsType")),
        }
    }
}

impl TryFrom<&Vec<EnumItem>> for GOsType {
    type Error = gen_utils::error::Error;

    fn try_from(value: &Vec<EnumItem>) -> Result<Self, <Self as TryFrom<&EnumItem>>::Error> {
        if value.len() == 1 {
            return value.get(0).unwrap().try_into();
        } else if value.len() == 2 {
            let root = value.get(0).unwrap();
            let leaf = value.get(1).unwrap();
            if let EnumItem::Root(root) = root {
                if root == "GOsType" {
                    return leaf.try_into();
                }
            }
        }
        Err(err_from_to!("EnumItem" => "GOsType"))
    }
}

impl ToTokens for GOsType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let res = match self {
            GOsType::Windows => WINDOWS,
            GOsType::Mac => MAC,
            GOsType::Linux => LINUX,
            GOsType::Other => OTHER,
        };
        tokens.extend(parse_str::<TokenStream>(res));
    }
}

#[cfg(test)]
mod test_os {

    use gen_analyzer::value::{Enum, EnumItem, Value};
    use quote::ToTokens;

    use crate::builtin::prop::GOsType;

    #[test]
    fn test1() {
        let v1 = Value::Enum(Enum {
            field_chain: vec![EnumItem::Leaf("Windows".to_string(), None)],
        });
        let v2 = Value::Enum(Enum {
            field_chain: vec![
                EnumItem::Root("GOsType".to_string()),
                EnumItem::Leaf("Mac".to_string(), None),
            ],
        });
        let res1 = GOsType::try_from(&v1).unwrap();
        let res2 = GOsType::try_from(&v2).unwrap();
        dbg!(res1.to_token_stream().to_string());
        dbg!(res2.to_token_stream().to_string());
        assert_eq!(res1, GOsType::Windows);
        assert_eq!(res2, GOsType::Mac);
    }
}
