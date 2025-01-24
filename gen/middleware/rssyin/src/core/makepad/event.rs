use gen_mk_script_objs::error::{Error, ProcMacroError};
use syn::{visit::Visit, Fields, ItemEnum};

use crate::visitor::chain::{
    res_ty::ResultType,
    traits::{BasicVisitor, VisitorEnum},
};

/// # Event Visitor
/// 专门用于访问带有`#[event]`属性宏的枚举，可多次工作
/// ```rust
/// #[event]
/// pub enum AEvent{
///    Clicked,
/// }
/// ```
#[derive(Default)]
pub struct EventVisitor {
    pub target: Option<ItemEnum>,
}

impl VisitorEnum for EventVisitor {
    fn visit_item_enum_with(
        &mut self,
        item: &syn::ItemEnum,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        if !self.is_attr_macro(item) {
            return Ok(ResultType::Ignore);
        }
        self.visit_item_enum(item);
        if self.target.is_some() {
            bridge
                .push_event(self.target.take())
                .and_then(|_| Ok(ResultType::Handled))
        } else {
            Err(Error::ProcMacro(ProcMacroError::NamedFieldEvent))
        }
    }
}

impl<'ast> Visit<'ast> for EventVisitor {
    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        if self.is_attr_macro(item) {
            // can not be Fields::Named
            let has_named = item.variants.iter().any(|var| {
                if let Fields::Named(_) = var.fields {
                    true
                } else {
                    false
                }
            });

            if !has_named {
                self.target = Some(item.clone());
            }
        }
    }
}

impl BasicVisitor for EventVisitor {
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

impl EventVisitor {
    pub fn is_attr_macro(&self, item: &ItemEnum) -> bool {
        item.attrs.iter().any(|attr| attr.path().is_ident("event"))
    }
}
