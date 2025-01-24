use syn::{visit::Visit, ItemStruct};

use crate::visitor::chain::{
    res_ty::ResultType,
    traits::{BasicVisitor, VisitorStruct},
};

/// # PropVisitor
/// 专门用于访问带有#[prop]属性宏的结构体，只工作一次
/// ```rust
/// <script>
/// #[prop]
/// pub struct AProp{
///     pub name: String,
/// }
/// </script>
/// ```
#[derive(Default)]
pub struct PropVisitor {
    pub worked: bool,
    pub target: Option<ItemStruct>,
}

impl VisitorStruct for PropVisitor {
    fn visit_item_struct_with(
        &mut self,
        item: &syn::ItemStruct,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        self.visit_item_struct(item);
        bridge
            .set_prop(self.target.take())
            .and_then(|_| Ok(ResultType::Handled))
    }
}

impl<'ast> Visit<'ast> for PropVisitor {
    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        if self.is_attr_macro(item) {
            self.target = Some(item.clone());
            self.worked = true;
        }
    }
}

impl BasicVisitor for PropVisitor {
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

impl PropVisitor {
    pub fn is_attr_macro(&self, item: &ItemStruct) -> bool {
        item.attrs.iter().any(|attr| attr.path().is_ident("prop"))
    }
}
