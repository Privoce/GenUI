use crate::{
    impl_basic_chain_visitor, impl_basic_visitor,
    visitor::chain::{
        res_ty::ResultType,
        traits::{BasicChainVisitor, BasicVisitor, VisitorFn},
    },
};

#[derive(Default)]
pub struct FnVisitorChain {
    visitors: Vec<Box<dyn VisitorFn>>,
}

impl FnVisitorChain {
    pub fn clear(&mut self) {
        for visitor in &mut self.visitors {
            visitor.reset();
        }
    }
}

impl_basic_visitor!(FnVisitorChain);

impl_basic_chain_visitor!(FnVisitorChain: Box<dyn VisitorFn>);

impl VisitorFn for FnVisitorChain {
    fn visit_item_fn_with(
        &mut self,
        item: &syn::ItemFn,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        for visitor in self.visitors.iter_mut() {
            if visitor.is_worked() {
                continue;
            } else {
                if let Ok(ty) = visitor.visit_item_fn_with(item, bridge) {
                    if ty.is_ignore() {
                        continue;
                    } else {
                        return Ok(ty);
                    }
                }
                continue;
            }
        }

        Ok(ResultType::Ignore)
    }
}
