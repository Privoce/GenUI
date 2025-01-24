use gen_mk_script_objs::error::SCResult;
use proc_macro2::TokenStream;
use syn::visit::Visit;

use crate::visitor::chain::{
    res_ty::ResultType,
    traits::{BasicVisitor, VisitorMacro},
};

/// Imports visitor
/// 用于访问导入的组件, 例如:
/// ```
/// import! {
///   crate::views::my_button::*;
/// }
/// ```
/// 该访问器会查找import!宏中的所有导入的组件，然后将其放置到ScriptBridger中, 以便后续的代码生成
/// 并且该访问器只会工作一次，因为import!宏只会出现一次，我们不允许多个import!宏出现
#[derive(Default, Debug)]
pub struct ImportVisitor {
    pub worked: bool,
    pub target: Option<String>,
}

impl VisitorMacro for ImportVisitor {
    fn visit_stmt_macro_with(
        &mut self,
        mac: &syn::StmtMacro,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> SCResult<ResultType> {
        if !mac.mac.path.is_ident("import") {
            return Ok(ResultType::Ignore);
        }
        self.visit_stmt_macro(mac);
        bridge
            .set_import(self.target.take().map(|s| s.parse().unwrap()))
            .and_then(|_| Ok(ResultType::Handled))
    }
}

impl BasicVisitor for ImportVisitor {
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

impl<'ast> Visit<'ast> for ImportVisitor {
    fn visit_stmt_macro(&mut self, i: &'ast syn::StmtMacro) {
        if i.mac.path.is_ident("import") {
            self.worked = true;
            // 获取所有的导入
            self.parse_import_macro(&i.mac.tokens);
            return;
        }
    }
}

impl ImportVisitor {
    fn parse_import_macro(&mut self, tk: &TokenStream) {
        // 首先我们需要去除`{}`，因为我们只需要里面的内容
        let tk_str = tk.to_string();
        let content = tk_str.trim().trim_matches(|c| c == '{' || c == '}').trim();
        if content.is_empty() {
            return;
        }
        // 将content使用`;`进行分割, 每个分割的部分就是一个导入, 并为这些导入增加use关键字
        let imports = content
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|s| format!("use {};", s.trim()))
            .collect::<Vec<String>>()
            .join("\n");

        self.target = Some(imports);
    }
}

#[cfg(test)]
mod test_import_visitor {
    use gen_mk_script_objs::makepad::ScriptBridger;
    use syn::Stmt;

    use super::*;
    #[test]
    fn easy_import() {
        let import = r#"
        import! {
            crate::views::my_button::*;
        }
        "#;
        let mac = syn::parse_str::<syn::Stmt>(import).unwrap();
        let mut visitor = ImportVisitor::default();
        let mut bridge = ScriptBridger::default();
        if let Stmt::Macro(mac) = mac {
            let _ = VisitorMacro::visit_stmt_macro_with(&mut visitor, &mac, &mut bridge);
        }
        assert_eq!(
            visitor.target,
            Some("crate :: views :: my_button ::*;".to_string())
        );
    }
}
