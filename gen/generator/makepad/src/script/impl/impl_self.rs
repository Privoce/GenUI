use quote::ToTokens;
use syn::Stmt;


/// Vec<ItemFn>
#[derive(Default, Debug, Clone)]
pub struct ImplSelf(pub Vec<Stmt>);

impl ImplSelf {
    pub fn extend(&mut self, stmts: Vec<Stmt>) {
        self.0.extend(stmts);
    }
}


impl ToTokens for ImplSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for stmt in &self.0 {
            stmt.to_tokens(tokens);
        }
    }
}