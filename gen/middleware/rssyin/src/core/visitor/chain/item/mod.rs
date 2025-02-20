mod r#enum;
mod r#fn;
mod r#impl;
mod r#struct;

use r#enum::EnumVisitorChain;
use r#fn::FnVisitorChain;
use r#impl::ImplVisitorChain;
use r#struct::StructVisitorChain;

use super::{
    res_ty::ResultType,
    traits::{BasicChainVisitor, VisitorEnum, VisitorFn, VisitorImpl, VisitorItem, VisitorStruct},
};

#[derive(Default)]
pub struct ItemVisitorChain {
    pub enums: EnumVisitorChain,
    pub structs: StructVisitorChain,
    pub fns: FnVisitorChain,
    pub impls: ImplVisitorChain,
}

impl ItemVisitorChain {
    pub fn clear(&mut self) {
        self.enums.clear();
        self.structs.clear();
        // self.fns.clear();
        self.impls.clear();
    }
}

impl VisitorItem for ItemVisitorChain {
    fn visit_item_with(
        &mut self,
        item: &syn::Item,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<super::res_ty::ResultType> {
        match item {
            syn::Item::Enum(item_enum) => {
                if self.enums.is_empty() {
                    return Ok(ResultType::Ignore);
                }
                self.enums.visit_item_enum_with(item_enum, bridge)
            }
            syn::Item::Impl(item_impl) => {
                if self.impls.is_empty() {
                    return Ok(ResultType::Ignore);
                }
                self.impls.visit_item_impl_with(item_impl, bridge)
            }
            syn::Item::Fn(item_fn) => {
                if self.fns.is_empty() {
                    return Ok(ResultType::Ignore);
                }
                self.fns.visit_item_fn_with(item_fn, bridge)
            }
            syn::Item::Struct(item_struct) => {
                if self.enums.is_empty() {
                    return Ok(ResultType::Ignore);
                }
                self.structs.visit_item_struct_with(item_struct, bridge)
            }
            _ => Ok(ResultType::Ignore),
        }
    }
}
