use quote::ToTokens;
use syn::{Ident, Pat, PatType, Stmt, Type};

#[allow(dead_code)]
pub trait TypeGetter {
    /// Get the type of the variable
    fn get(&self, var: &str) -> Option<&PatType>;
    fn has(&self, var: &str) -> bool {
        self.get(var).is_some()
    }
    fn ty(&self, var: &str) -> Option<String>;
}

impl TypeGetter for Stmt {
    fn get(&self, var: &str) -> Option<&PatType> {
        if let Stmt::Local(local) = self {
            if let Pat::Type(ident_ty) = &local.pat {
                if let Pat::Ident(ident) = &*ident_ty.pat {
                    if ident.ident.eq(var) {
                        return Some(ident_ty);
                    }
                }
            }
        }
        None
    }

    fn ty(&self, var: &str) -> Option<String> {
        self.get(var).map(|x| {
            if let Type::Path(path) = &*x.ty {
                path.path.segments.to_token_stream().to_string()
            } else {
                panic!("Type not supported")
            }
        })
    }
}

pub trait PatTypeGetter {
    /// Get the type in Pat::Type
    fn get(&self) -> Option<(Ident, Ident)>;
}

impl PatTypeGetter for Pat {
    fn get(&self) -> Option<(Ident, Ident)> {
        if let Pat::Type(pat_type) = self {
            let k = if let Pat::Ident(ident) = &*pat_type.pat {
                Some(ident.ident.clone())
            } else {
                None
            };
            let v = if let Type::Path(path) = &*pat_type.ty {
                Some(path.path.segments.first().unwrap().ident.clone())
            } else {
                None
            };
            return Some((k.unwrap(), v.unwrap()));
        }
        None
    }
}
