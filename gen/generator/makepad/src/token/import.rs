use proc_macro2::TokenStream;
use quote::quote;

pub fn import_default() -> TokenStream {
    quote! {
        use link::widgets::*;
        use link::gen_components::*;
    }
}

pub fn import_draw_shader() -> TokenStream {
    quote! {
        use link::shaders::*;
    }
}

pub fn import_default_all() -> TokenStream {
    let mut tk = import_default();
    let _ = tk.extend(import_draw_shader());
    tk
}
