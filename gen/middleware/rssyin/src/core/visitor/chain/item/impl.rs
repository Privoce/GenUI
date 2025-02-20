use crate::{
    impl_basic_chain_visitor, impl_basic_visitor,
    visitor::chain::traits::{BasicChainVisitor, BasicVisitor, VisitorImpl},
};

#[derive(Default)]
pub struct ImplVisitorChain {
    pub visitors: Vec<Box<dyn VisitorImpl>>,
}

impl ImplVisitorChain {
    pub fn clear(&mut self) {
        for visitor in &mut self.visitors {
            visitor.reset();
        }
    }
}

impl_basic_visitor!(ImplVisitorChain);
impl_basic_chain_visitor!(ImplVisitorChain: Box<dyn VisitorImpl>);

impl VisitorImpl for ImplVisitorChain {
    fn visit_item_impl_with(
        &mut self,
        item: &syn::ItemImpl,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<crate::visitor::chain::res_ty::ResultType> {
        for visitor in self.visitors.iter_mut() {
            if visitor.is_worked() {
                continue;
            } else {
                if let Ok(ty) = visitor.visit_item_impl_with(item, bridge) {
                    if ty.is_ignore() {
                        continue;
                    } else {
                        return Ok(ty);
                    }
                }
                continue;
            }
        }

        Ok(crate::visitor::chain::res_ty::ResultType::Ignore)
    }
}
