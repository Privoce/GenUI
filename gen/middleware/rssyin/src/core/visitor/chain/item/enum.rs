use crate::{
    impl_basic_chain_visitor, impl_basic_visitor,
    visitor::chain::{res_ty::ResultType, traits::{BasicChainVisitor, BasicVisitor, VisitorEnum}},
};

#[derive(Default)]
pub struct EnumVisitorChain {
    visitors: Vec<Box<dyn VisitorEnum>>,
}

impl EnumVisitorChain {
    pub fn clear(&mut self) {
        for visitor in &mut self.visitors {
            visitor.reset();
        }
    }
}

impl_basic_visitor!(EnumVisitorChain);

impl_basic_chain_visitor!(EnumVisitorChain: Box<dyn VisitorEnum>);

impl VisitorEnum for EnumVisitorChain {
    fn visit_item_enum_with(
        &mut self,
        item: &syn::ItemEnum,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        for visitor in self.visitors.iter_mut() {
            if visitor.is_worked() {
                continue;
            } else {
                if let Ok(ty) = visitor.visit_item_enum_with(item, bridge) {
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
