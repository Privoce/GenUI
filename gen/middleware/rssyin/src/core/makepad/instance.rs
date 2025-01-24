use syn::{visit::Visit, Expr, Local};

use crate::visitor::chain::{
    res_ty::ResultType,
    traits::{BasicVisitor, VisitorLocal},
};

/// 用于进行组件属性的访问者, 主要用于后续将组件的属性ident转为self
/// 这个访问者只工作一次，来访问Local，需要判断Local的init是否位Expr::Macro并使用`default_prop!`
/// ```rust
/// let mut prop = default_prop!{
///    MyStruct{
///       a: 10,
///       b: 10.0,
///  }
/// ```
#[derive(Default)]
pub struct InstanceVisitor {
    pub worked: bool,
    pub target: Option<Local>,
}

impl VisitorLocal for InstanceVisitor {
    fn visit_local_with(
        &mut self,
        local: &syn::Local,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        if bridge.instance_macro_worked() {
            return Ok(ResultType::Ignore);
        }

        self.visit_local(local);
        if self.worked {
            if let Some(instance) = self.target.take() {
                return bridge
                    .set_instance(instance)
                    .and_then(|_| Ok(ResultType::Handled));
            }
        }
        return Ok(ResultType::Ignore);
    }
}

impl<'ast> Visit<'ast> for InstanceVisitor {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        // 检查是否有init
        if let Some(init) = local.init.as_ref() {
            if let Expr::Macro(expr_macro) = &*init.expr {
                self.visit_expr_macro(expr_macro);
                if self.worked {
                    self.target.replace(local.clone());
                }
            }
        }
    }

    fn visit_expr_macro(&mut self, expr_macro: &'ast syn::ExprMacro) {
        if expr_macro.mac.path.is_ident("default_prop") {
            self.worked = true;
        }
    }
}

impl BasicVisitor for InstanceVisitor {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn is_worked(&self) -> bool {
        self.worked
    }

    fn reset(&mut self) {
        self.worked = false;
    }
}
