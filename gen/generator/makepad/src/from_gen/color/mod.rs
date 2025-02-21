mod draw;

use crate::{str_to_tk, traits::ToTokensExt};
use draw::*;
use gen_analyzer::value::{BuiltinColor, Hex, Value};
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;

/// Makepad Color
/// 专门用语处理Makepad的颜色的结构体，需要借助BuiltinColor
/// see [BuiltinColor]
#[derive(Debug, Clone)]
pub struct MakepadColor {
    pub fn_name: Option<String>,
    pub color: BuiltinColor,
}

impl From<&MakepadColor> for toml_edit::Value {
    fn from(value: &MakepadColor) -> Self {
        value.color.to_string().into()
    }
}

impl TryFrom<(&Value, Option<String>)> for MakepadColor {
    type Error = Error;

    fn try_from(value: (&Value, Option<String>)) -> Result<Self, Self::Error> {
        let (value, fn_name) = value;
        let color = BuiltinColor::try_from(value)?;
        Ok(MakepadColor { fn_name, color })
    }
}

impl TryFrom<&toml_edit::Value> for MakepadColor {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_str().map_or_else(
            || Err(Error::from("toml_edit::Item to MakepadColor")),
            |s| {
                let color = BuiltinColor::from_str(s)?;
                Ok(MakepadColor {
                    fn_name: None,
                    color,
                })
            },
        )
    }
}

impl ToTokensExt for MakepadColor {
    fn to_token_stream(&self) -> Result<TokenStream, Error> {
        let mut tokens = TokenStream::new();
        match &self.color {
            BuiltinColor::Hex(hex) => tokens.extend(str_to_tk!(&hex.to_vec4())?),
            BuiltinColor::Rgb(rgb) => tokens.extend(str_to_tk!(&Hex::from(rgb).to_vec4())?),
            BuiltinColor::Rgba(rgba) => tokens.extend(str_to_tk!(&Hex::from(rgba).to_vec4())?),
            BuiltinColor::LinearGradient(linear_gradient) => {
                let tk = draw_linear_gradient(linear_gradient)?;
                let fn_name = str_to_tk!(self.fn_name.as_ref().unwrap_or(&String::from("pixel")))?;
                tokens.extend(quote! {
                    {
                        fn #fn_name(self) -> vec4{
                            #tk
                        }
                    }
                });
            }
            BuiltinColor::RadialGradient(radial_gradient) => {
                let tk = draw_radial_gradient(radial_gradient)?;
                let fn_name = str_to_tk!(self.fn_name.as_ref().unwrap_or(&String::from("pixel")))?;
                tokens.extend(quote! {
                    {
                        fn #fn_name(self) -> vec4{
                            #tk
                        }
                    }
                });
            }
            BuiltinColor::Shader(makepad_shader) => {
                let tk = makepad_shader.0.clone();
                tokens.extend(quote! {
                    {
                        #tk
                    }
                });
            }
        }
        Ok(tokens)
    }
}

impl ToTokens for MakepadColor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(ToTokensExt::to_token_stream(self).unwrap());
    }
}
