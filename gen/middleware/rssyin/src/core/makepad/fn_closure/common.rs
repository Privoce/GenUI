use quote::ToTokens;
use syn::{
    parse2, punctuated::Punctuated, token::Comma, visit::Visit, Expr, ExprClosure, FnArg, Ident,
    ItemFn, Pat, Signature, Visibility,
};

use crate::visitor::chain::{
    res_ty::ResultType,
    traits::{BasicVisitor, VisitorFn, VisitorLocal},
};

/// # 函数访问者
/// 用于访问函数, 可工作多次，将访问的每一个函数都存储在bridge的fns中方便后续进行ra_ap_syntax的语义分析和替换
/// 替换是最终转换到makepad代码时才进行到操作
/// 同时这个访问器也用于访问生命周期，将生命周期存储在bridge的lifetimes中
/// 这个访问器需要去调用push_lifetime方法，不需要去访问具体的生命周期
#[derive(Default)]
pub struct FnVisitor {
    pub target: Option<ItemFn>,
}

impl VisitorFn for FnVisitor {
    fn visit_item_fn_with(
        &mut self,
        item: &syn::ItemFn,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        // 优先存储生命周期
        bridge.push_lifetime(item).map_or_else(
            |_| {
                self.visit_item_fn(item);
                bridge
                    .push_fn(self.target.take().unwrap())
                    .and_then(|_| Ok(ResultType::Handled))
            },
            |_| Ok(ResultType::Handled),
        )
    }
}

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        self.target = Some(item_fn.clone());
    }
}

impl BasicVisitor for FnVisitor {
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

// --------------------------------------------------------------------------------
/// # 闭包访问者
/// 闭包和Fn是一样的，但需要转换为普通的函数
#[derive(Default)]
pub struct ClosureVisitor {
    pub target: Option<ItemFn>,
}

impl VisitorLocal for ClosureVisitor {
    fn visit_local_with(
        &mut self,
        local: &syn::Local,
        bridge: &mut gen_mk_script_objs::makepad::ScriptBridger,
    ) -> gen_mk_script_objs::error::SCResult<ResultType> {
        self.visit_local(local);
        if let Some(item) = self.target.take() {
            return bridge.push_fn(item).and_then(|_| Ok(ResultType::Handled));
        }
        return Ok(ResultType::Ignore);
    }
}

impl ClosureVisitor {
    // 将闭包转为普通函数
    pub fn convert_closure_to_fn(&mut self, fn_name: Ident, closure: &ExprClosure) -> () {
        let block = if let Expr::Block(block) = &*closure.body {
            block.block.clone()
        } else {
            panic!("ClosureVisitor: expr is not block")
        };

        let sig = Signature {
            constness: closure.constness,
            asyncness: closure.asyncness,
            unsafety: None,
            abi: None,
            fn_token: syn::token::Fn::default(),
            ident: fn_name,
            generics: Default::default(),
            paren_token: syn::token::Paren::default(),
            inputs: Self::convert_inputs(&closure.inputs),
            variadic: Default::default(),
            output: closure.output.clone(),
        };

        let item_fn = syn::ItemFn {
            attrs: closure.attrs.clone(),
            vis: Visibility::Inherited,
            sig,
            block: Box::new(block),
        };

        self.target = Some(item_fn);
    }
    fn convert_inputs(closure: &Punctuated<Pat, Comma>) -> Punctuated<FnArg, Comma> {
        closure
            .iter()
            .map(|pat| {
                parse2::<FnArg>(pat.to_token_stream()).expect("ClosureVisitor: pat to fnarg err")
            })
            .collect()
    }
}

impl<'ast> Visit<'ast> for ClosureVisitor {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        if let Some(init) = &local.init {
            if let syn::Expr::Closure(closure) = &*init.expr {
                let fn_name = if let Pat::Ident(ident) = local.pat.clone() {
                    ident.ident
                } else {
                    panic!("ClosureVisitor: local is not ident")
                };
                self.convert_closure_to_fn(fn_name, closure);
            }
        }
    }
}

impl BasicVisitor for ClosureVisitor {
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
