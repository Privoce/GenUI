use syn::ItemFn;

use crate::visitor::chain::traits::{BasicVisitor, VisitorImpl};

/// # impl访问器
///
#[derive(Default)]
pub struct ImplVisitor {
    pub target: Option<ItemFn>,
}

impl VisitorImpl for ImplVisitor {
    fn visit_item_impl_with(
        &mut self,
        item: &syn::ItemImpl,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<crate::visitor::chain::res_ty::ResultType> {
        // 先从script brigder中获取实例的信息，如果没有则暂缓处理
        // 访问impl检查是否为Default trait的实现
    
    }
}

impl BasicVisitor for ImplVisitor {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn is_worked(&self) -> bool {
        self.target.is_some()
    }

    fn reset(&mut self) {
        self.target = None;
    }
}
