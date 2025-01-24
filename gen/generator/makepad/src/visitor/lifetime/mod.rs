pub mod input;

use gen_dyn_run::DynProcessor;
use gen_mk_script_objs::makepad::lifetime::{network::HttpLifeTime, WidgetLifeTime};

use gen_utils::error::{CompilerError, Error};
use input::Input;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use syn::{parse_quote, FnArg, ItemFn, Signature, Stmt, Type};

use crate::{
    compiler::WidgetPoll,
    model::{
        traits::{ImplLiveHook, LiveHookType, WidgetMatchEventType},
        PropBinds,
    },
    script::Impls,
};

use super::{r#fn::visit_builtin, FnVisitorImpl, InstanceOutput};

/// # 表示生命周期的访问者
/// 声明周期需要处理的的代码类似于fn-callback中的代码
pub struct LifetimeLzVisitor {
    input: Input,
}

impl LifetimeLzVisitor {
    pub fn new(
        widget_poll: WidgetPoll,
        instance: Option<InstanceOutput>,
        fn_names: Vec<String>,
    ) -> Self {
        Self {
            input: Input::new(widget_poll, instance, fn_names),
            // target_fn: None,
            // callback_fn: None,
            // ref_fn: None,
            // param_tk: None,
            // is_mut: false,
        }
    }
    pub fn visit(
        &mut self,
        i: &mut WidgetLifeTime,
        // traits: &mut Option<Traits>,
        impls: &mut Impls,
        prop_binds: &PropBinds,
        processor: Option<&DynProcessor>,
    ) -> Result<(), Error> {
        if let Some(before_create) = i.before_create.as_mut() {
            self.visit_before_create(before_create, impls, prop_binds, processor)?;
        }
        if let Some(_created) = i.created.as_ref() {
            todo!()
        }

        if let Some(_mounted) = i.mounted.as_ref() {
            todo!()
        }

        if let Some(_unmounted) = i.unmounted.as_ref() {
            todo!()
        }

        self.visit_http(&mut i.http, impls, prop_binds, processor)?;

        Ok(())
    }

    /// 使用ra模块检查itemfn
    fn visit_builtin(
        &mut self,
        i: &mut ItemFn,
        prop_binds: &PropBinds,
        processor: Option<&DynProcessor>,
    ) -> Result<(), Error> {
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
        .map_err(|e| {
            Error::from(CompilerError::runtime(
                "Makepad Compiler - Script",
                &e.to_string(),
            ))
        })
    }

    fn visit_before_create(
        &mut self,
        i: &mut ItemFn,
        impls: &mut Impls,
        prop_binds: &PropBinds,
        processor: Option<&DynProcessor>,
    ) -> Result<(), Error> {
        let fn_name = i.sig.ident.clone();

        // let super_tk = TokenStream::new();
        // self.visit_item_fn_mut(i, processor, &mut super_tk)?;
        self.visit_builtin(i, prop_binds, processor)?;
        i.attrs.clear();
        // 添加参数
        i.sig.inputs.insert(0, parse_quote! {&mut self});
        i.sig.inputs.insert(1, parse_quote! {cx: &mut Cx});

        impls
            .self_impl
            .extend(vec![Stmt::Item(syn::Item::Fn(i.clone()))]);
        impls.traits_impl.0.live_hook.push(
            quote! {
                self.#fn_name(cx);
            },
            LiveHookType::AfterApplyFromDoc,
        );
        Ok(())
    }

    fn visit_http(
        &mut self,
        i: &mut HttpLifeTime,
        impls: &mut Impls,
        prop_binds: &PropBinds,
        processor: Option<&DynProcessor>,
    ) -> Result<(), Error> {
        if let Some(responses) = i.responses.as_mut() {
            self.visit_http_responses(responses, impls, prop_binds, processor)?;
        }
        Ok(())
    }

    /// 访问http的响应
    /// ```rust
    /// #[http_response]
    /// fn http_response1(response: &HttpResponse) -> (){
    ///     // ...
    /// }
    ///
    /// // http_response2 ...
    /// ```
    /// ```rust
    /// fn  http_response1(response: &HttpResponse) -> (){//...}
    ///
    /// match request_id {
    ///     live_id!(http_response1) => http_response1(response),
    ///     // http_response2 ...
    ///     _ => {}
    /// }
    /// ```
    fn visit_http_responses(
        &mut self,
        responses: &mut Vec<ItemFn>,
        impls: &mut Impls,
        prop_binds: &PropBinds,
        processor: Option<&DynProcessor>,
    ) -> Result<(), Error> {
        let mut tokens = TokenStream::new();
        let super_tk = TokenStream::new();
        let mut inner_fns: Vec<Stmt> = vec![];
        for response in responses {
            let live_match_id = response.sig.ident.clone();
            // 检查方法的参数是否只有一个且类型为 &HttpResponse
            if !is_response_param(&response.sig) {
                return Err(CompilerError::runtime("Makepad Plugin - Script", "http_response method must have only one parameter and the type is &HttpResponse").into());
            }

            // self.visit_item_fn_mut(response, processor, &mut super_tk)?;
            response.attrs.clear();
            // 移除宏
            // response.attrs.clear();
            // 给方法添加&mut self, cx参数
            response.sig.inputs.insert(0, parse_quote! {&mut self});
            response.sig.inputs.insert(1, parse_quote! {cx: &mut Cx});
            self.visit_builtin(response, prop_binds, processor)?;
            inner_fns.push(Stmt::Item(syn::Item::Fn(response.clone())));
            tokens.extend(quote! {
                live_id!(#live_match_id) => self.#live_match_id(cx, response),
            });
        }

        let tokens = quote! {
            #super_tk

            match request_id {
                #tokens
                _ => {}
            }
        };

        impls.self_impl.extend(inner_fns);
        impls
            .traits_impl
            .0
            .push_widget_match_event(tokens, WidgetMatchEventType::HttpResponse);

        Ok(())
    }
}

fn is_response_param(sig: &Signature) -> bool {
    if sig.inputs.len() == 1 {
        if let FnArg::Typed(arg_ty) = &sig.inputs[0] {
            if let Type::Reference(ty_ref) = &*arg_ty.ty {
                return ty_ref.mutability.is_none()
                    && ty_ref.elem.to_token_stream().to_string() == "HttpResponse";
            }
        }
    }
    false
}

impl FnVisitorImpl for LifetimeLzVisitor {
    fn set_mut(&mut self, _is_mut: bool) {}

    fn has_widget(&self, id: &str) -> bool {
        self.input.widget_poll.contains_key(id)
    }

    fn instance(&self) -> Option<&InstanceOutput> {
        self.input.instance.as_ref()
    }

    // fn is_prop_method_and_append(&mut self, method: &mut ExprMethodCall) -> Result<bool, Error> {
    //     is_prop_method_and_append(self, method)
    // }

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

    // fn visit_local_init_mut(
    //     &mut self,
    //     i: &mut syn::LocalInit,
    //     processor: Option<&DynProcessor>,
    // ) -> Result<bool, Error> {
    //     super::visit_local_init_mut(self, i, processor)
    // }
}
