use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse2, parse_quote, parse_str, Expr, ExprStruct, ItemStruct, Member, Pat, Stmt};

use crate::builtin::prop::err_from_to;

/// # Lazy instance(after_apply_from_doc) Visitor for Makepad
/// ```
/// let mut prop = default_prop ! { MyStruct { a : 10 , b : 12.0 , } } ;
/// ```
/// ```
/// self.set_a(10);
/// self.set_b(12.0);
/// ```
pub struct InstanceLzVisitor {
    // // 属性结构体
    // pub prop: ItemStruct,
    input: Input,
    output: Option<Output>,
    pub token: Option<Vec<Stmt>>,
}

impl InstanceLzVisitor {
    pub fn new(prop: &ItemStruct) -> Self {
        Self {
            input: Input::new(prop),
            token: None,
            output: None,
        }
    }
}

impl InstanceLzVisitor {
    pub fn set_output(&mut self, ident: String, is_mut: bool) {
        self.output.replace(Output {
            ident,
            is_mut,
            fields: None,
        });
    }
    fn push_stmt(&mut self, stmt: Stmt) {
        if let Some(token) = self.token.as_mut() {
            token.push(stmt);
        } else {
            self.token.replace(vec![stmt]);
        }
    }
    /// 访问实例并获得Output
    pub fn visit(&mut self, i: &syn::Local) -> Result<(Output, Option<Vec<Stmt>>), Error> {
        self.visit_local(i)?;

        // 延迟设置fields
        let _ = self.output.as_mut().map(|out| {
            out.fields = self.input.fields.clone();
        });

        Ok((self.output.take().unwrap(), self.token.take()))
    }

    fn visit_local(&mut self, i: &syn::Local) -> Result<(), Error> {
        // 实际上只要能访问这个访问者，那么这个local一定是有init的
        if let Some(init) = i.init.as_ref() {
            if let Expr::Macro(expr_mac) = &*init.expr {
                if expr_mac.mac.path.is_ident("default_prop") {
                    if let Pat::Ident(pat) = &i.pat {
                        self.set_output(pat.ident.to_string(), pat.mutability.is_some());
                        // do visit
                        self.visit_expr_macro(expr_mac)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn visit_expr_macro(&mut self, i: &syn::ExprMacro) -> Result<(), Error> {
        // mac的tokens是一个TokenStream, 但现在发现可以转为Expr::Struct来进行处理
        let expr_struct = parse2::<ExprStruct>(i.mac.tokens.clone())
            .map_err(|_| Error::from(err_from_to("default_prop! - TokenStream", "ExprStruct")))?;

        for field in expr_struct.fields {
            if let Member::Named(ident) = field.member {
                let set_fn = parse_str::<TokenStream>(format!("set_{}", ident).as_str()).unwrap();
                let value = field.expr.to_token_stream();
                self.push_stmt(parse_quote! {
                    self.#set_fn(cx, #value);
                });
            }
        }

        Ok(())
    }
}

/// # InstanceLzVisitor input
/// instance visitor的输入结构体，需要传入实例的定义
pub struct Input {
    /// 实例结构体的标识，例如: `#[prop] MyStruct{}`
    /// 其中的`MyStruct`就是标识
    pub ident: String,
    /// 实例结构体的字段标识，例如: `#[prop] MyStruct { a : i32 , b : f32 , }`
    /// 其中的`a , b`就是字段标识，但也可能是None
    pub fields: Option<Vec<String>>,
}

impl Input {
    pub fn new(prop_struct: &ItemStruct) -> Self {
        let ident = prop_struct.ident.to_string();
        let fields = Self::get_fields(prop_struct);
        Self { ident, fields }
    }
    // 获取属性结构体中的fields
    fn get_fields(prop: &ItemStruct) -> Option<Vec<String>> {
        let mut res = vec![];
        for field in prop.fields.iter() {
            if let Some(ident) = field.ident.as_ref() {
                let ident = ident.to_string();
                if ident != "deref_widget" {
                    res.push(ident);
                }
            }
        }

        (!(res.is_empty())).then(|| res)
    }
}

/// # InstanceLzVisitor output
/// instance visitor的输出结构体，它可能会作为后续其他Visitor的输入
/// 例如作为FnLzVisitor的输入，指明实例的信息
#[derive(Debug, Clone)]
pub struct Output {
    /// 实例的标识，例如: `let mut prop = default_prop ! { MyStruct { a : 10 , b : 12.0 , } } ;`
    /// 其中的`prop`就是标识, 在GenUI的方法中这个标识会转换为`self`因为它指向当前实例
    pub ident: String,
    /// 当前实例是否是mut，若定义非mut则后续使用时无法修改，只能访问，他会作为后续方法中的`&self`或者`&mut self`
    pub is_mut: bool,
    /// 这个fields来源于input中的fields，用于后续的访问，
    pub fields: Option<Vec<String>>,
}
