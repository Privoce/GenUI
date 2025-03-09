use gen_analyzer::{value::Value, PropKey};
use gen_utils::{err_from_to, error::Error};
use quote::quote;

use crate::props_to_tokens;

use super::{DVec2, Margin, Size};

#[derive(Debug, Clone)]
pub struct TextWalk(pub Walk);

impl quote::ToTokens for TextWalk {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let prop = self.0.to_token_stream();
        tokens.extend(quote! {
            {#prop}
        });
    }
}

impl TryFrom<&(PropKey, Value)> for TextWalk {
    type Error = Error;

    fn try_from(value: &(PropKey, Value)) -> Result<Self, Self::Error> {
        Ok(TextWalk(Walk::try_from(value)?))
    }
}

impl From<Walk> for TextWalk {
    fn from(value: Walk) -> Self {
        TextWalk(value)
    }
}

#[derive(Debug, Clone)]
pub enum Walk {
    AbsPos(DVec2),
    Margin(Margin),
    Width(Size),
    Height(Size),
}

impl Walk {
    pub fn to_toml_item_kv(&self) -> (String, toml_edit::Value) {
        match self {
            Walk::AbsPos(v) => ("abs_pos".to_string(), v.into()),
            Walk::Margin(v) => ("margin".to_string(), v.into()),
            Walk::Width(v) => ("width".to_string(), v.into()),
            Walk::Height(v) => ("height".to_string(), v.into()),
        }
    }
}

impl TryFrom<(&str, &toml_edit::Value)> for Walk {
    type Error = Error;

    fn try_from(value: (&str, &toml_edit::Value)) -> Result<Self, Self::Error> {
        let (key, val) = value;
        match key {
            "abs_pos" => Ok(Walk::AbsPos(DVec2::try_from(val)?)),
            "margin" => Ok(Walk::Margin(Margin::try_from(val)?)),
            "width" => Ok(Walk::Width(Size::try_from(val)?)),
            "height" => Ok(Walk::Height(Size::try_from(val)?)),
            _ => {
                return Err(err_from_to!("GenUI Props" => "Makepad Walk, Invalid Prop"));
            }
        }
    }
}

impl TryFrom<&(PropKey, Value)> for Walk {
    type Error = Error;

    fn try_from(value: &(PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "abs_pos" => Ok(Walk::AbsPos(DVec2::try_from(&value.1)?)),
            "margin" => Ok(Walk::Margin(Margin::try_from(&value.1)?)),
            "width" => Ok(Walk::Width(Size::try_from(&value.1)?)),
            "height" => Ok(Walk::Height(Size::try_from(&value.1)?)),
            _ => {
                return Err(err_from_to!("GenUI Props" => "Makepad Walk, Invalid Prop"));
            }
        }
    }
}

props_to_tokens! {
    Walk,
    Walk::AbsPos => abs_pos, false,
    Walk::Margin => margin, false,
    Walk::Width => width, false,
    Walk::Height => height, false
}
