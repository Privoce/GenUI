use quote::ToTokens;
use syn::{parse_quote, Stmt};

use crate::two_way_binding::default_impl_ref_get_set;

#[derive(Debug, Clone)]
pub struct ImplSelfRef(pub Vec<Stmt>);

impl ImplSelfRef {
    pub fn extend(&mut self, stmts: Vec<Stmt>) {
        self.0.extend(stmts);
    }
}

impl ToTokens for ImplSelfRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for stmt in &self.0 {
            stmt.to_tokens(tokens);
        }
    }
}

impl Default for ImplSelfRef {
    fn default() -> Self {
        let impl_ref_get_set = default_impl_ref_get_set();
        let ref_render: Stmt = parse_quote! {
            ref_render!();
        };
        let ref_redraw_mut: Stmt = parse_quote! {
            ref_redraw_mut!();
        };

        Self(vec![
            parse_quote! {#impl_ref_get_set},
            ref_render,
            ref_redraw_mut,
        ])
    }
}
