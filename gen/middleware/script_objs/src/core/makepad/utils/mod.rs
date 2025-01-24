use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Type};

use crate::error::{AttrMacroError, SCResult};

pub const LOW_LIVES: [&str; 19] = [
    "i32",
    "i64",
    "isize",
    "u32",
    "u64",
    "usize",
    "f32",
    "f64",
    "bool",
    "String",
    "LiveDependency",
    "Vec4",
    "Vec3",
    "Vec2",
    "Margin",
    "Padding",
    "GOsType",
    "Direction",
    "Align",
];

/// ## Get the attribute from the field type
/// Get the attribute from the field type, if the field has `#[live]` or `#[rust]` return it, or return `#[live]` if the type is in LOW_LIVES else `#[rust]`
pub fn get_attr_from_field_type(ty: &Type, attrs: &Vec<Attribute>) -> SCResult<TokenStream> {
    // first try to find `#[live]` or `#[rust]` in attrs
    // if has, return it or return `#[live]` if the type is in LOW_LIVES else `#[rust]`
    // live and rust can not both be true
    let mut live = None;
    let mut rust = None;
    for attr in attrs {
        if attr.path().is_ident("live") {
            live.replace(attr);
        } else if attr.path().is_ident("rust") {
            rust.replace(attr);
        }
    }

    match (live, rust) {
        (None, None) => {
            let ty_str = ty.to_token_stream().to_string();
            if LOW_LIVES.contains(&ty_str.as_str()) {
                Ok(quote! {
                    #[live]
                })
            } else {
                Ok(quote! {
                    #[rust]
                })
            }
        }
        (None, Some(attr)) => Ok(attr.to_token_stream()),
        (Some(attr), None) => Ok(attr.to_token_stream()),
        (Some(_), Some(_)) => Err(AttrMacroError::LiveRustConflict.into()),
    }
}
