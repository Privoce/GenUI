mod draw;

use std::str::FromStr;

use draw::*;
use gen_analyzer::value::{BuiltinColor, Hex, Value};
// use gen_parser::{
//     common::{hex_to_vec4, BuiltinColor, Hex},
//     Value,
// };
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_str;

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
        value
            .as_str()
            .map_or_else(
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

impl ToTokens for MakepadColor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.color {
            BuiltinColor::Hex(hex) => tokens.extend(hex_to_vec4(hex)),
            BuiltinColor::Rgb(rgb) => tokens.extend(hex_to_vec4(&Hex::from(rgb))),
            BuiltinColor::Rgba(rgba) => tokens.extend(hex_to_vec4(&Hex::from(rgba))),
            BuiltinColor::LinearGradient(linear_gradient) => {
                let tk = draw_linear_gradient(linear_gradient);
                let fn_name = parse_str::<TokenStream>(
                    self.fn_name.as_ref().unwrap_or(&String::from("pixel")),
                )
                .unwrap();
                tokens.extend(quote! {
                    {
                        fn #fn_name(self) -> vec4{
                            #tk
                        }
                    }
                });
            }
            BuiltinColor::RadialGradient(radial_gradient) => {
                let tk = draw_radial_gradient(radial_gradient);
                let fn_name = parse_str::<TokenStream>(
                    self.fn_name.as_ref().unwrap_or(&String::from("pixel")),
                )
                .unwrap();
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
    }
}
