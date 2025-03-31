use std::collections::HashMap;

use crate::{
    model::traits::{ImplLiveHook, LiveHookType},
    script::Impls,
    str_to_tk,
    traits::MakepadExtComponent,
};
use gen_analyzer::{
    value::{Bind, Function, Value},
    Binds, SugarIf,
};
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, ExprArray, ImplItem, ImplItemFn, Stmt};

pub struct ComputedVisitor;

/// ## 计算属性 (e.g.: #[computed(a, b, c)] )
/// 计算属性是GenUI中的一个重要概念，它是一个特殊的方法，使用`#[computed(${link_arg}...)]`进行方法标记，表示这个方法是一个计算属性
/// 计算属性需要关联到组件属性上，这样在组件属性进行变化时，计算属性会自动更新。对计算属性我们需要生成：
/// 1. 去除方法上的`#[computed(${link_arg}...)]`标记
/// 2. 生成`update_${computed_fn_name}`方法, 该方法用于更新计算属性和对应组件
/// 3. 找到宏中与计算属性关联的组件属性的setter方法，将更新方法添加到setter方法中
impl ComputedVisitor {
    pub fn visit(
        item_fn: &mut ImplItemFn,
        impls: &mut Impls,
        binds: Option<&Binds>,
        elses: &mut HashMap<String, Function>,
    ) -> Result<(), Error> {
        fn build_fn_args(function: &Function) -> TokenStream {
            function
                .params_str()
                .map(|args| {
                    let mut args = str_to_tk!(&args).unwrap();
                    args.extend(quote! {, cx});
                    args
                })
                .unwrap_or(quote! {cx})
        }
        // dbg!(binds);

        // [去除方法上的属性宏] ------------------------------------------------------------------------------------
        item_fn
            .attrs
            .retain(|attr| !attr.path().is_ident("computed"));

        // [为方法添加cx参数] ------------------------------------------------------------------------------------
        item_fn.sig.inputs.push(parse_quote! {cx: &mut Cx});
        // [生成更新方法] ------------------------------------------------------------------------------------------
        let fn_name = item_fn.sig.ident.to_token_stream();
        let update_fn_name = str_to_tk!(&format!("update_{}", fn_name.to_string()))?;

        let bind_components = binds
            .map(|binds| binds.get(&fn_name.to_string()).map(|bind| bind.clone()))
            .flatten()
            .ok_or(CompilerError::runtime(
                "Makepad Plugin - Script",
                "can not find bind for computed method",
            ))?;

        for bind_component in bind_components {
            let widget_id = str_to_tk!(&bind_component.id)?;
            let widget = str_to_tk!(&bind_component.name())?;
            let set_fn = str_to_tk!(&format!(
                "set_{}",
                if SugarIf::SUGAR_SIGNS.contains(&bind_component.prop.as_str()) {
                    "visible"
                } else {
                    bind_component.prop.as_str()
                }
            ))?;

            // 如果是Else的话，需要对value进行累加
            let new_value_fn = match bind_component.prop {
                gen_analyzer::Prop::Value(prop_kv) => {
                    if let Value::Function(function) = prop_kv.value {
                        let fn_args = build_fn_args(&function);
                        quote! {self.#fn_name(#fn_args)}
                    } else if let Value::Bind(Bind::Fn(function)) = prop_kv.value {
                        let fn_args = build_fn_args(&function);
                        quote! {self.#fn_name(#fn_args)}
                    } else {
                        quote! {self.#fn_name(cx)}
                    }
                }
                gen_analyzer::Prop::Else(items) => {
                    let len = items.len();
                    let mut pre: Option<String> = None;
                    str_to_tk!(&items
                        .iter()
                        .enumerate()
                        .map(|(index, item)| {
                            if index == len - 1 {
                                if let Value::Bind(Bind::Fn(function)) = &item.value {
                                    if let Some(pre) = pre.as_ref() {
                                        elses.insert(pre.to_string(), function.clone());
                                    }
                                }
                                Ok("!new_value".to_string())
                            } else {
                                let fn_ident = item.value.as_bind()?.ident();
                                if index == len - 2
                                    && SugarIf::SUGAR_SIGNS.contains(&item.key.as_str())
                                {
                                    pre.replace(fn_ident.to_string());
                                }

                                let args = if let Value::Function(function) = &item.value {
                                    build_fn_args(function)
                                } else if let Value::Bind(Bind::Fn(function)) = &item.value {
                                    build_fn_args(function)
                                } else {
                                    TokenStream::new()
                                };

                                Ok(format!("!self.{}({})", fn_ident, args.to_string()))
                            }
                        })
                        .collect::<Result<Vec<String>, Error>>()?
                        .join(" && "))?
                }
            };

            let fn_block = quote! {
                let new_value = #new_value_fn;
                let widget = self.#widget(id!(#widget_id));
                widget.#set_fn(cx, new_value)?;
            };

            // 如果之前已经存在更新方法，则需要将代码附加到更新方法中，否则创建新的更新方法
            if let Some(update_fn) = impls.self_impl.get_mut_fn(&update_fn_name.to_string()) {
                let mut index = update_fn.block.stmts.len() - 1;
                let stmts: Vec<Stmt> = parse_quote!(#fn_block);
                for stmt in stmts {
                    update_fn.block.stmts.insert(index, stmt);
                    index += 1;
                }
            } else {
                let update_fn: ImplItem = parse_quote! {
                    fn #update_fn_name(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
                        #fn_block
                        Ok(())
                    }
                };

                impls.self_impl.push(update_fn);
            }
        }

        Ok(())
    }

    pub fn handle_update(
        arg_map: HashMap<String, ExprArray>,
        impls: &mut Impls,
        fields: &Vec<String>,
        computeds: &mut Vec<String>,
        elses: &HashMap<String, Function>,
    ) -> Result<(), Error> {
        fn patch_update(set_fn: &str, impls: &mut Impls, update_fn_name: &TokenStream) {
            if let Some(item_fn) = impls.self_impl.get_mut_fn(set_fn) {
                let index = item_fn.block.stmts.len() - 1;
                item_fn.block.stmts.insert(
                    index,
                    parse_quote! {
                        self.#update_fn_name(cx)?;
                    },
                );
            }
        }

        // [从args中获取绑定的组件属性并将更新方法添加到对应的setter中] -------------------------------------------------
        for (fn_name, args) in arg_map {
            let update_fn_name = str_to_tk!(&format!("update_{}", fn_name))?;
            for arg in args.elems.iter() {
                let arg = arg.to_token_stream();
                let arg_str = arg.to_string();
                let set_fn_str = format!("set_{}", arg.to_string());

                // 如果arg在fields中已经存在，则跳过，否则在AfterNewFromDoc中添加初始化方法
                if !computeds.contains(&arg_str) && !fields.contains(&arg_str) {
                    computeds.push(arg_str);
                    let set_fn = str_to_tk!(&set_fn_str)?;
                    impls.traits().live_hook.push(
                        quote! {
                            self.#set_fn(cx, deref_prop.#arg);
                        },
                        LiveHookType::AfterNewFromDoc,
                    );
                }

                patch_update(&set_fn_str, impls, &update_fn_name);
            }
            // 查找elses中是否有与fn_name相同的函数，如果有则需要添加一个update方法
            if let Some(else_fn) = elses.get(&fn_name) {
                let update_fn_name = str_to_tk!(&format!("update_{}", else_fn.ident()))?;
                for arg in args.elems.iter() {
                    let set_fn_str = format!("set_{}", arg.to_token_stream().to_string());
                    patch_update(&set_fn_str, impls, &update_fn_name);
                }
            }
        }
        Ok(())
    }
}
