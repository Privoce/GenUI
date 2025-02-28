use proc_macro2::TokenStream;

use syn::{parse_quote, ItemImpl, Stmt};

/// # Lazy instance(after_apply_from_doc) Visitor for Makepad
/// 在这里我们需要将impl Default for xxx的 ident: xxx 修改为传入的deref_prop_ident, 并且添加到others中
pub struct InstanceLzVisitor;

impl InstanceLzVisitor {
    pub fn visit(default_impl: &mut ItemImpl, ident: TokenStream, others: &mut Vec<Stmt>) -> () {
        // [替换ident] -----------------------------------------------------------------------------------------
        default_impl.self_ty = parse_quote!(#ident);
        // [添加到others] ---------------------------------------------------------------------------------------
        others.push(parse_quote!(#default_impl));
    }
}
