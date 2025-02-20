mod import;
mod lifecycle;
use std::collections::HashMap;

pub use import::{Import, Imports};
use lifecycle::LifeCycle;
use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct ScriptBridger {
    pub imports: Option<Imports>,
    pub prop: Option<syn::ItemStruct>,
    /// default impl
    pub instance: Option<syn::ItemImpl>,
    pub event: Option<syn::ItemEnum>,
    // lifecycles: LifeCycle,
    pub impl_prop: Option<syn::ItemImpl>,
    // 非追踪部分
    pub others: TokenStream,
}
