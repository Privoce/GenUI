use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Fields, ItemStruct};

use crate::{from_struct_to_ptr, ptr_to_token, utils::struct_field, widget::utils::quote_makepad_widget_struct, ToToken};

pub struct ScrollBarPropPtr(pub ItemStruct);

from_struct_to_ptr!{ScrollBarPropPtr, "ScrollBar"}

ptr_to_token!(ScrollBarPropPtr);