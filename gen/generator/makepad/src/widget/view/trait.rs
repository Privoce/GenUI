use gen_converter::model::{script::PropFn, PropTree};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::widget::utils::quote_handle_event;

pub fn handle_event(
    event: &Option<Vec<PropFn>>,
    binds: &PropTree,
    instance_name: Option<&Ident>,
    prop_fields: Option<&Vec<Ident>>,
) -> TokenStream {
    quote_handle_event(event, binds, instance_name, prop_fields)
}

pub fn draw_walk() -> TokenStream {
    quote! {
        self.view.draw_walk(cx, scope, walk)
    }
}
