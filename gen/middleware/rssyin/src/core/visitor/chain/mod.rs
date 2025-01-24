mod chain_macros;
mod expr;
mod item;
mod local;
mod r#macro;
pub mod res_ty;
pub mod traits;

use gen_mk_script_objs::{error::SCResult, makepad::ScriptBridger};
use item::ItemVisitorChain;
use local::LocalVisitorChain;
use r#macro::MacroVisitorChain;
use res_ty::ResultType;

use traits::{
    BasicChainVisitor, BasicVisitor, ChainVisitor, VisitorItem, VisitorLocal, VisitorMacro,
};

/// # 访问器链(访问状态机)
#[derive(Default)]
pub struct VisitorChain {
    // // visitors: Vec<Box<dyn ChainVisitor>>,
    // current: usize,
    pub locals: LocalVisitorChain,
    pub items: ItemVisitorChain,
    // exprs: ExprVisitorChain,
    pub macros: MacroVisitorChain,
    pub bridge: ScriptBridger,
}


// 实现访问者链的访问器
impl ChainVisitor for VisitorChain {
    fn clear(&mut self) ->() {
        self.locals.clear();
        self.items.clear();
        self.macros.clear();
        self.bridge.clear();
    }
    fn is_worked(&self) -> bool {
        self.macros.is_worked()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        VisitorChain::default()
    }

    fn visit_block_with(&mut self, block: &syn::Block) -> SCResult<()> {
        for stmt in block.stmts.iter() {
            match stmt {
                syn::Stmt::Local(local) => {
                    if let Ok(ty) = self.visit_local_with(local) {
                        ty.is_ignore_then(SCResult::Ok(()), || {
                            self.bridge.push_other(&stmt);
                            SCResult::Ok(())
                        })?;
                    }
                }
                syn::Stmt::Item(item) => {
                    if let Ok(ty) = self.visit_item_with(item) {
                        ty.is_ignore_then(SCResult::Ok(()), || {
                            self.bridge.push_other(&stmt);
                            SCResult::Ok(())
                        })?;
                    }
                }
                syn::Stmt::Expr(..) => {
                    //  目前来看没有什么需要GenUI rssyin处理的，只有一个async后续可能会支持网络的异步处理
                    self.bridge.push_other(&stmt);
                },
                syn::Stmt::Macro(stmt_macro) => {
                    if let Ok(ty) = self.visit_stmt_macro_with(stmt_macro) {
                        ty.is_ignore_then(SCResult::Ok(()), || {
                            self.bridge.push_other(&stmt);
                            SCResult::Ok(())
                        })?;
                    }
                }
            }
        }
        Ok(())
    }

    fn visit_item_with(&mut self, item: &syn::Item) -> SCResult<ResultType> {
        self.items.visit_item_with(item, &mut self.bridge)
    }

    fn visit_stmt_macro_with(&mut self, mac: &syn::StmtMacro) -> SCResult<ResultType> {
        if self.macros.is_empty() {
            return Ok(ResultType::Ignore);
        }
        self.macros.visit_stmt_macro_with(mac, &mut self.bridge)
    }

    fn visit_local_with(&mut self, local: &syn::Local) -> SCResult<ResultType> {
        self.locals.visit_local_with(local, &mut self.bridge)
    }
}
