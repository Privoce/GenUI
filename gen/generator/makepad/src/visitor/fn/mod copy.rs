use super::InstanceOutput;
use crate::{
    compiler::WidgetPoll,
    model::{
        traits::{CRef, CallbackStmt},
        CallbackWidget, PropBinds,
    },
    script::Impls,
};
use gen_dyn_run::DynProcessor;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{parse_quote, parse_str, FnArg};

mod input;
mod replacer;
mod traits;
// mod utils;

pub use input::Input;
pub use replacer::*;
pub use traits::FnVisitorImpl;
// pub use utils::*;
/// # Lazy fn(event) Visitor for Makepad
/// handle convert fn to real makepad fn
#[derive(Debug)]
pub struct FnLzVisitor {
    input: Input,
    /// target fn name
    pub target_fn: Option<String>,
    /// 方法的参数
    pub param_tk: Option<TokenStream>,
    /// should the method be mutable
    pub is_mut: bool,
}

impl FnLzVisitor {
    pub fn new(
        widget_poll: WidgetPoll,
        callback_widget: Option<CallbackWidget>,
        instance: Option<InstanceOutput>,
        fn_names: Vec<String>,
    ) -> Self {
        Self {
            input: Input::new(widget_poll, callback_widget, instance, fn_names),
            target_fn: None,
            param_tk: None,
            is_mut: false,
        }
    }

    fn get_or_create_event(
        widget: &CallbackWidget,
        callbacks: &mut HashSet<CallbackStmt>,
        target_fn: &str,
    ) -> Option<CallbackStmt> {
        widget.callbacks.get(target_fn).and_then(|call_fn| {
            let callback = CallbackStmt::new(
                widget.id.to_string(),
                String::new(),
                String::new(),
                call_fn.name.to_string(),
            );

            if let Some(callback) = callbacks.take(&callback) {
                Some(callback)
            } else {
                Some(callback)
            }
        })
    }

    fn has_or_set_cref(widget: &CallbackWidget, c_refs: &mut HashSet<CRef>) -> () {
        let c_ref = CRef {
            id: widget.id.to_string(),
            name: widget.name.to_string(),
        };
        if !c_refs.contains(&c_ref) {
            c_refs.insert(c_ref);
        }
    }

    fn push_fn_call(
        &self,
        callback_stmt: &mut CallbackStmt,
        widget: &CallbackWidget,
        target_fn: &str,
    ) -> Result<(), Error> {
        let target_fn = parse_str::<TokenStream>(target_fn).unwrap();
        let mut params = vec![];
        // let _ = callback_stmt.param.as_ref().map(|x| {
        //     params.push(parse_str::<TokenStream>(x).unwrap());
        // });
        if callback_stmt.param.is_some() {
            params.push(quote! {param});
        }
        // args
        let _ = widget
            .get(self.target_fn.as_ref().unwrap())
            .ok_or(Error::Compiler(CompilerError::runtime(
                "Makepad Compiler - Script",
                "CallbackWidget::get_event_tk: event is not valid",
            )))?
            .func
            .params_str()
            .map(|x| {
                params.push(parse_str::<TokenStream>(&x).unwrap());
            });

        let widget_id_tk = widget.id_tk();
        params.extend(vec![quote! {cx}, quote! {&#widget_id_tk}]);

        callback_stmt
            .fns
            .push(parse_quote! {self.#target_fn(#(#params),*);});

        Ok(())
    }

    pub fn visit(
        &mut self,
        widget_poll: &WidgetPoll,
        prop_binds: &PropBinds,
        i: &mut syn::ItemFn,
        processor: Option<&DynProcessor>,
        impls: &mut Impls,
    ) -> Result<(), Error> {
        // [获取当前方法名字] --------------------------------------------------------------------------------
        let target_fn = i.sig.ident.to_string();
        self.target_fn.replace(target_fn.to_string());
        // [尝试到impls的traits的handle_event中找到对应的方法] ------------------------------------------------
        if let Some(widget) = self.input.callback_widget.as_ref() {
            Self::has_or_set_cref(widget, &mut impls.traits_impl.0.widget.handle_event.c_refs);
            if let Some(mut callback_stmt) = Self::get_or_create_event(
                widget,
                &mut impls.traits_impl.0.widget.handle_event.callbacks,
                &target_fn,
            ) {
                // [属于callback fn] -----------------------------------------------------------------------
                // add #[allow(unused_variables)] ---------------------------------------------------------
                i.attrs.push(parse_quote! {#[allow(unused_variables)]});
                // [进行一次遍历找到类型是: impl EventParam的参数] ---------------------------------------------
                for p in i.sig.inputs.iter_mut() {
                    if let FnArg::Typed(ty) = p {
                        if let syn::Type::ImplTrait(impl_trait) = &*ty.ty {
                            if let syn::TypeParamBound::Trait(trait_bound) = &impl_trait.bounds[0] {
                                if trait_bound.path.is_ident("EventParam") {
                                    // 获取真实的参数返回值的类型
                                    let event_ty = widget
                                        .event_ty(widget_poll, self.target_fn.as_ref().unwrap())
                                        .ok_or(Error::Compiler(CompilerError::runtime(
                                            "Makepad Compiler - Script",
                                            "can not find target param type",
                                        )))?;

                                    let event_ty_tk = parse_str::<TokenStream>(&event_ty).unwrap();
                                    ty.ty = parse_quote! {#event_ty_tk};
                                    // 给callback_stmt添加参数
                                    callback_stmt.param.replace(event_ty);
                                    self.param_tk.replace(ty.pat.to_token_stream());
                                    break;
                                }
                            }
                        }
                    }
                }
                // [将当前方法的调用设置到callback_stmt中] -------------------------------------------------------
                self.push_fn_call(&mut callback_stmt, widget, &target_fn)?;
                // [将callback_stmt设置回] -------------------------------------------------------------------
                impls
                    .traits_impl
                    .0
                    .widget
                    .handle_event
                    .callbacks
                    .insert(callback_stmt);
            }
        }
        // [使用ra模块检查itemfn] --------------------------------------------------------------------------
        let (prop, fields) = if let Some(instance) = self.input.instance.as_ref() {
            if let Some(fields) = instance.fields.as_ref() {
                (&instance.ident, fields)
            } else {
                (&instance.ident, &vec![])
            }
        } else {
            (&String::new(), &vec![])
        };
        visit_builtin(
            i,
            &prop,
            fields,
            &self.input.widget_poll,
            prop_binds,
            processor,
        )
        .map_err(|e| CompilerError::runtime("Makepad Compiler - Script", &e.to_string()))?;
        // [在FnArg中添加cx和actions] ---------------------------------------------------------------------
        let arg = parse_quote! {&mut self};
        i.sig.inputs.insert(0, arg);
        if let Some(widget) = self.input.callback_widget.as_ref() {
            i.sig.inputs.push(parse_quote! {cx: &mut Cx});
            let widget_id = widget.id_tk();
            let widget_ref = widget.widget_ref_tk();
            i.sig.inputs.push(parse_quote! {#widget_id: &#widget_ref});
        }

        Ok(())
    }
    #[allow(dead_code)]
    fn is_prop_field(&self, ident: &str, field: &String) -> bool {
        if let Some(instance) = self.input.instance.as_ref() {
            if let Some(fields) = instance.fields.as_ref() {
                return ident == instance.ident && fields.contains(field);
            }
        }
        false
    }
}

impl FnVisitorImpl for FnLzVisitor {
    fn set_mut(&mut self, is_mut: bool) {
        if self.is_mut == false {
            self.is_mut = is_mut;
        }
    }

    fn has_widget(&self, id: &str) -> bool {
        self.input.widget_poll.contains_key(id)
    }

    fn instance(&self) -> Option<&InstanceOutput> {
        self.input.instance.as_ref()
    }

    fn get_widget_name(&self, id: &str) -> Result<String, Error> {
        self.input.get_widget_name(id).map_or_else(
            || {
                Err(CompilerError::runtime(
                    "Makepad Compiler - Script",
                    &format!("can not find widget by id: {}", id),
                )
                .into())
            },
            |widget| Ok(widget.snake_name()),
        )
    }
}
