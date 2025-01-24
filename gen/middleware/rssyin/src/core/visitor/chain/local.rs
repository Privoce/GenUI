use crate::{impl_basic_chain_visitor, impl_basic_visitor};

use super::{
    res_ty::ResultType,
    traits::{BasicChainVisitor, BasicVisitor, VisitorLocal},
};

#[derive(Default)]
pub struct LocalVisitorChain {
    visitors: Vec<Box<dyn VisitorLocal>>,
}

impl LocalVisitorChain {
    pub fn clear(&mut self) {
        for visitor in &mut self.visitors {
            visitor.reset();
        }
    }
}

impl_basic_visitor!(LocalVisitorChain);

impl_basic_chain_visitor!(LocalVisitorChain: Box<dyn VisitorLocal>);

impl VisitorLocal for LocalVisitorChain {
    fn visit_local_with(
        &mut self,
        local: &syn::Local,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<super::res_ty::ResultType> {
        // 每次进入访问链时，都需要从index = 0开始进行访问
        // 用循环进行访问
        for visitor in self.visitors.iter_mut() {
            if visitor.is_worked() {
                continue;
            } else {
                //  如果子访问者还能工作，则直接执行
                // 这里不用担心会Err，因为访问者的执行是不会Err的，使用SCResult是为了提升到总访问者链的处理
                if let Ok(ty) = visitor.visit_local_with(local, bridge) {
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
