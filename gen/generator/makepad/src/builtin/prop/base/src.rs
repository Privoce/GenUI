use gen_analyzer::value::{Enum, EnumItem, Value};
use gen_utils::{
    err_from_to,
    error::{ConvertError, Error},
};



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
        if let EnumItem::Leaf(leaf, value) = value {}
        Err(err_from_to!("EnumItem" => "Size"))
    }
}
