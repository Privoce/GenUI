use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Fields, ItemStruct};

use crate::{from_struct_to_ptr, ptr_to_token, utils::struct_field, widget::utils::quote_makepad_widget_struct, ToToken};

pub struct RootPropPtr(pub ItemStruct);

from_struct_to_ptr!{RootPropPtr, "root", "Root"}

ptr_to_token!(RootPropPtr);

impl RootPropPtr{
    pub fn deref_struct_ptr(name: &str) -> TokenStream{
        let name = parse_str::<TokenStream>(name).unwrap();

        quote!{
            #[derive(Live, Widget)]
            pub struct #name{
                #[live] #[deref] pub deref_widget: Root,
            }
        }
    }
}