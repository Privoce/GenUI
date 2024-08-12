use gen_converter::model::script::PropFn;

use gen_utils::common::ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::widget::utils::quote_handle_event;

pub fn handle_event(
    event: &Option<Vec<PropFn>>,
    props: &Option<Vec<PropFn>>,
    instance_name: Option<&Ident>,
    prop_fields: Option<&Vec<Ident>>,
) -> TokenStream {
    quote_handle_event(
        Some(ident("deref_widget")),
        event,
        props,
        instance_name,
        prop_fields,
    )
}

pub fn draw_walk() -> TokenStream {
    quote! {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
}
