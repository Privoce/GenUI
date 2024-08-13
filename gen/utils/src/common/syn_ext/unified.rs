use syn::{Pat, Stmt};

use super::{TypeGetter, VarGetter};

pub trait UnifiedGetter {
    fn ty(&self, var: &str) -> Option<String>;
}

impl UnifiedGetter for Option<&Vec<Stmt>> {
    fn ty(&self, var: &str) -> Option<String> {
        dbg!(&self);
        self.map(|stmts| {
            stmts
                .iter()
                .find_map(|stmt| {
                    if let Stmt::Local(local) = stmt {
                        if let Pat::Type(_) = local.pat {
                            TypeGetter::ty(stmt, var)
                        } else if let Pat::Ident(_) = local.pat {
                            VarGetter::ty(stmt, var)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap()
        })
    }
}
