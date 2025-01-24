use quote::ToTokens;
use syn::Stmt;

#[derive(Default, Debug, Clone)]
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