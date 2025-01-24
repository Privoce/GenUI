use crate::{
    impl_basic_chain_visitor, impl_basic_visitor,
    visitor::chain::{
        res_ty::ResultType,
        traits::{BasicChainVisitor, BasicVisitor, VisitorStruct},
    },
};

#[derive(Default)]
pub struct StructVisitorChain {
    visitors: Vec<Box<dyn VisitorStruct>>,
}

impl StructVisitorChain {
    pub fn clear(&mut self) {
        for visitor in &mut self.visitors {
            visitor.reset();
        }
    }
}

impl_basic_visitor!(StructVisitorChain);

impl_basic_chain_visitor!(StructVisitorChain: Box<dyn VisitorStruct>);

impl VisitorStruct for StructVisitorChain {
    fn visit_item_struct_with(
        &mut self,
        item: &syn::ItemStruct,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        for visitor in self.visitors.iter_mut() {
            if visitor.is_worked() {
                continue;
            } else {
                if let Ok(ty) = visitor.visit_item_struct_with(item, bridge) {
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
