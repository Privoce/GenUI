mod base;
mod float;
mod u_i_number;
mod macros;
mod theme;
mod traits;
mod utils;

use std::fmt::Debug;

pub use base::*;
pub use u_i_number::*;
pub use float::*;
use gen_analyzer::{PropKey, value::Value};
use gen_utils::error::{ConvertError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_str;
pub use theme::Themes;
pub use traits::*;
pub use utils::*;

use crate::{traits::ToTokensExt, try_from_value_ref_enum};

/// Makepad Prop
#[derive(Debug, Clone)]
pub struct Prop<T>(pub Vec<T>)
where
    T: Debug + ToTokens;

impl<T> Default for Prop<T>
where
    T: Debug + ToTokens,
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Prop<T>
where
    T: Debug + ToTokens,
{
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
}

impl<T> ToTokens for Prop<T>
where
    T: Debug + ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.0.is_empty() {
            let props = self.0.iter().map(|x| x.to_token_stream());
            tokens.extend(quote! {
                #(#props),*,
            });
        }
    }
}

pub fn props_callback<F, R, E>(props: Option<gen_analyzer::Props>, f: F) -> Result<R, E>
where
    F: FnOnce(Option<gen_analyzer::Props>) -> Result<R, E>,
{
    f(props)
}


// handle prop value for static -----------------------------------------------------------------------------------------------------
pub fn handle_prop_value_static(k: &PropKey, v: &Value) -> Result<TokenStream, Error> {
    handle_builtin_prop_value_static(k, v).or_else(|_| handle_custom_prop_value_static(v))
}

fn handle_builtin_prop_value_static(k: &PropKey, v: &Value) -> Result<TokenStream, Error> {
    match k.name.as_str() {
        "theme" => Themes::try_from(v).and_then(|v| Ok(v.to_token_stream())),
        _ => Err(ConvertError::UnSupport(k.name.as_str().to_string()).into()),
    }
}

fn handle_custom_prop_value_static(v: &Value) -> Result<TokenStream, Error> {
    // parse_str::<TokenStream>(&v.to_string()).map_err(|e| Error::FromDynError(e.to_string()))
    ToTokensExt::to_token_stream(v)
}

// handle prop value ----------------------------------------------------------------------------------------------------------------
pub fn handle_prop_value(k: &PropKey, v: &Value) -> Result<TokenStream, Error> {
    // first do handle builtin prop value
    handle_builtin_prop_value(k, v).or_else(|_| {
        // if builtin prop value not support, do handle custom prop value
        handle_custom_prop_value(k, v)
    })
}

fn handle_builtin_prop_value(k: &PropKey, v: &Value) -> Result<TokenStream, Error> {
    match k.name.as_str() {
        "theme" => Themes::try_from(v).and_then(|v| {
            Ok(quote! {
                theme: #v
            })
        }),
        _ => Err(ConvertError::UnSupport(k.name.as_str().to_string()).into()),
    }
}

fn handle_custom_prop_value(k: &PropKey, v: &Value) -> Result<TokenStream, Error> {
    let prop = k.name.as_str();
    let value =
        parse_str::<TokenStream>(&v.to_string()).map_err(|e| Error::FromDynError(e.to_string()))?;
    Ok(quote! {
        #prop: #value
    })
}

// from macros -----------------------------------------------------------------------------------------------------
try_from_value_ref_enum! {
    Themes, "Themes",
    GOsType, "GOsType",
    MouseCursor, "MouseCursor",
    ViewOptimize, "ViewOptimize",
    EventOrder, "EventOrder",
    Flow, "Flow",
    TextWrap, "TextWrap",
    Direction, "Direction",
    ImageFit, "ImageFit",
    LinkType, "LinkType",
    GChooseType, "GChooseType",
    GToggleType, "GToggleType",
    GLoadingType, "GLoadingType",
    Position4, "Position4",
    Position, "Position",
    TriggerMode, "TriggerMode",
    CloseMode, "CloseMode",
    PopupMode, "PopupMode",
    NavMode, "NavMode"
}
