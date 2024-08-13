use quote::ToTokens;
use syn::{Ident, Pat, Stmt};

#[allow(dead_code)]
pub trait VarGetter {
    fn get(&self, var: &str) -> Option<&Ident>;
    fn get_to_str(&self, var: &str) -> Option<String> {
        self.get(var).map(|x| x.to_string())
    }
    fn has(&self, var: &str) -> bool {
        self.get(var).is_some()
    }
    fn ty(&self, var: &str) -> Option<String>;
}
impl VarGetter for Stmt {
    fn get(&self, var: &str) -> Option<&Ident> {
        if let Stmt::Local(local) = self {
            if let Pat::Ident(ident) = &local.pat {
                if ident.ident.eq(var) {
                    return Some(&ident.ident);
                }
            }
        }
        None
    }
    
    fn ty(&self, var: &str) -> Option<String> {
        if let Stmt::Local(local) = self {
            let mut flag = false;
            if let Pat::Ident(ident) = &local.pat {
                if ident.ident.eq(var) {
                    // get type
                    flag = true;
                }
            }

            if flag{
                // get init, and get expr lit
                if let Some(init) = &local.init {
                    if let syn::Expr::Lit(lit) = &*init.expr {
                        
                        // wait to make a rustc type analysis
                        return Some(lit.lit.to_token_stream().to_string());
                    }
                }
            }
        }
        None
    }
    
}