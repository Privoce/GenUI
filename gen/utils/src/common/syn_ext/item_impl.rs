//! external for syn::token::ItemImpl

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{token::For, Expr, ImplItem, ItemImpl, Stmt};

use crate::common;

pub trait ImplGetter {
    /// ## get the fields of the struct in the impl
    /// ```rust
    /// let item_impl = syn::parse_str::<ItemImpl>("impl Default for Component { fn default() -> Self { Self{ value: 0, } } }").unwrap();
    /// assert_eq!(item_impl.fields(), vec!["value".to_string()]);
    /// ```
    fn fields(&self) -> Vec<String>;
}

impl ImplGetter for ItemImpl {
    fn fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        if let ImplItem::Fn(item_fn) = self.items.last().unwrap() {
            for stmt in &item_fn.block.stmts {
                if let Stmt::Expr(expr, _) = stmt {
                    if let Expr::Struct(expr_struct) = expr {
                        for field in &expr_struct.fields {
                            fields.push(field.member.to_token_stream().to_string());
                        }
                    }
                }
            }
        }
        fields
    }
}

/// # Converter for syn::token::ItemImpl
pub trait ImplConverter {
    /// ## convert the Default impl trait to local
    /// **if use this function, you should use `check_impl_ident` to check the impl is Default!**
    /// ```
    /// // origin default impl
    /// impl Default for Component {
    ///     fn default() -> Self {
    ///         Self{ value: 0, }
    ///     }
    /// }
    /// // after convert
    /// self.value = 0;
    /// ```
    fn default_to_self(&self) -> TokenStream;
    /// ## check the ident of the impl
    /// ```
    /// let item_impl = syn::parse_str::<ItemImpl>("impl Default for Component { fn default() -> Self { Self{ value: 0, } } }").unwrap();
    /// assert_eq!(item_impl.check_impl_ident("Default"), true);
    /// ```
    fn check_impl_ident(&self, ident: &str) -> bool;
}

impl ImplConverter for ItemImpl {
    fn default_to_self(&self) -> TokenStream {
        let mut tk = TokenStream::new();
        if let ImplItem::Fn(item_fn) = &self.items[0] {
            for index in 0..item_fn.block.stmts.len() {
                // if the last stmt, convert to self, else extend the token stream
                let stmt = if index == item_fn.block.stmts.len() - 1 {
                    if let Stmt::Expr(expr, _) = &item_fn.block.stmts[index] {
                        if let Expr::Struct(expr_struct) = expr {
                            expr_struct
                                .fields
                                .iter()
                                .fold(TokenStream::new(), |mut acc, field| {
                                    let field_name = field.member.to_token_stream();
                                    let field_value = field.expr.to_token_stream();
                                    acc.extend(quote! {self.#field_name = #field_value;});
                                    acc
                                })
                        } else {
                            panic!("the last stmt of the fn block should be a struct expr");
                        }
                    } else {
                        panic!("the last stmt of the fn block should be a stmt expr");
                    }
                } else {
                    item_fn.block.stmts[index].to_token_stream()
                };
                tk.extend(stmt);
            }
        }

        tk
    }

    fn check_impl_ident(&self, ident: &str) -> bool {
        if let Some((_, impl_ident, is_for)) = self.trait_.as_ref() {
            return is_for.eq(&For::default())
                && impl_ident.segments.len() == 1
                && impl_ident
                    .segments
                    .first()
                    .unwrap()
                    .ident
                    .eq(&common::ident(ident));
        }
        false
    }
}

#[cfg(test)]
mod test_ext {
    use syn::ItemImpl;

    use super::ImplGetter;

    #[test]
    fn fields() {
        let item_impl = syn::parse_str::<ItemImpl>(
            "impl Default for Component { fn default() -> Self { Self{ value: 0, } } }",
        )
        .unwrap();
        dbg!(item_impl.fields());
    }
}
