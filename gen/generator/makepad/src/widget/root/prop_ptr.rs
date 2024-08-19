use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Fields, ItemStruct};

use crate::{
    from_struct_to_ptr, ptr_to_token,
    utils::struct_field,
    widget::utils::{deref_struct_ptr, quote_makepad_widget_struct},
    ToToken,
};

pub struct RootPropPtr(pub ItemStruct);

from_struct_to_ptr! {RootPropPtr, "Root"}

ptr_to_token!(RootPropPtr);

impl RootPropPtr {
    pub fn deref_struct_ptr(name: &str) -> TokenStream {
        deref_struct_ptr(name, "Root")
    }
}
