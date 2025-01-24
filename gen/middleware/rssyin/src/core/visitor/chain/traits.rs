use gen_mk_script_objs::{error::SCResult, makepad::ScriptBridger};

use super::res_ty::ResultType;

/// # 链式访问者的trait，每个链式访问者都需要实现这个trait
pub trait ChainVisitor {
    /// 是否已经工作, 对于一些访问者，他们可能工作过一次或几次之后不需要再进行工作了
    fn is_worked(&self) -> bool;
    fn new() -> Self
    where
        Self: Sized;
    fn visit_block_with(&mut self, block: &syn::Block) -> SCResult<()>;
    fn visit_item_with(&mut self, item: &syn::Item) -> SCResult<ResultType>;
    fn visit_stmt_macro_with(&mut self, mac: &syn::StmtMacro) -> SCResult<ResultType>;
    fn visit_local_with(&mut self, local: &syn::Local) -> SCResult<ResultType>;
    /// 清空访问者链并刷新状态
    fn clear(&mut self) ->();
}

// enum -----------------------------------------------------------------------
pub trait VisitorEnum: BasicVisitor {
    fn visit_item_enum_with(
        &mut self,
        item: &syn::ItemEnum,
        bridge: &mut ScriptBridger,
    ) -> SCResult<ResultType>;
}

// struct ---------------------------------------------------------------------
pub trait VisitorStruct: BasicVisitor {
    fn visit_item_struct_with(
        &mut self,
        item: &syn::ItemStruct,
        bridge: &mut ScriptBridger,
    ) -> SCResult<ResultType>;
}

// fn ------------------------------------------------------------------------
pub trait VisitorFn: BasicVisitor {
    fn visit_item_fn_with(
        &mut self,
        item: &syn::ItemFn,
        bridge: &mut ScriptBridger,
    ) -> SCResult<ResultType>;
}

// local -----------------------------------------------------------------------
pub trait VisitorLocal: BasicVisitor {
    fn visit_local_with(
        &mut self,
        local: &syn::Local,
        bridge: &mut ScriptBridger,
    ) -> SCResult<ResultType>;
}
// item ------------------------------------------------------------------------
pub trait VisitorItem {
    fn visit_item_with(
        &mut self,
        item: &syn::Item,
        bridge: &mut ScriptBridger,
    ) -> SCResult<ResultType>;
}

// macro -----------------------------------------------------------------------
pub trait VisitorMacro: BasicVisitor {
    fn visit_stmt_macro_with(
        &mut self,
        mac: &syn::StmtMacro,
        bridge: &mut ScriptBridger,
    ) -> SCResult<ResultType>;
}

// basic -----------------------------------------------------------------------
pub trait BasicVisitor {
    fn new() -> Self
    where
        Self: Sized;
    fn is_worked(&self) -> bool;
    fn reset(&mut self);
}

pub trait BasicChainVisitor {
    /// 用于将访问者放置到链式访问者中
    type Visitor;
    fn push(&mut self, visitor: Self::Visitor);
    fn get(&self, index: usize) -> &Self::Visitor;
    fn get_mut(&mut self, index: usize) -> &mut Self::Visitor;
    fn is_empty(&self) -> bool;
}