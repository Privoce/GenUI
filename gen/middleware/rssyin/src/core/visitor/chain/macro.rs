use gen_mk_script_objs::error::SCResult;

use crate::{impl_basic_chain_visitor, impl_basic_visitor};

use super::{
    res_ty::ResultType,
    traits::{BasicChainVisitor, BasicVisitor, VisitorMacro},
};

#[derive(Default)]
pub struct MacroVisitorChain {
    visitors: Vec<Box<dyn VisitorMacro>>,
}

impl MacroVisitorChain {
    pub fn clear(&mut self) {
        for visitor in &mut self.visitors {
            visitor.reset();
        }
    }
}

impl_basic_chain_visitor!(MacroVisitorChain: Box<dyn VisitorMacro>);

impl_basic_visitor!(MacroVisitorChain);

impl VisitorMacro for MacroVisitorChain {
    fn visit_stmt_macro_with(
        &mut self,
        mac: &syn::StmtMacro,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> SCResult<ResultType> {
        // // 如果bridge已经完成了所有的macro访问，那么就直接返回
        if bridge.procedural_macro_worked() {
            return Ok(ResultType::Ignore);
        }

        for visitor in self.visitors.iter_mut() {
            if visitor.is_worked() {
                continue;
            } else {
                if let Ok(ty) = visitor.visit_stmt_macro_with(mac, bridge) {
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
