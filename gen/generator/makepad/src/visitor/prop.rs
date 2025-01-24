use std::collections::HashMap;

use crate::{
    builtin::BuiltinWidget,
    model::{
        traits::{CRef, CallbackStmt, HandleEvent, ImplLiveHook, LiveHookType},
        PropBinds, PropWidget, TemplatePtrs,
    },
    script::Impls,
    two_way_binding::{GetSet, TWBPollBuilder},
};
use gen_utils::error::{CompilerError, Error};
use quote::ToTokens;
use syn::{Fields, ItemStruct};

use super::SugarScript;

/// # Visitor for the widget prop
/// 这个visitor主要用于处理widget的prop，当开发者定义prop中中的字段后，我们需要为这些字段生成get和set方法
/// 这非常重要，因为这是响应式双向绑定的基础
/// SugarScript也要在PropLzVisitor中进行处理
pub struct PropLzVisitor;

impl PropLzVisitor {
    pub fn visit(
        prop: &mut ItemStruct,
        binds: &PropBinds,
        template_ptrs: &TemplatePtrs,
        impls: &mut Impls,
    ) -> Result<Option<TWBPollBuilder>, Error> {
        // [生成function相关的双向绑定代码] -----------------------------------------------------------------------

        // [生成get和set方法] -----------------------------------------------------------------------------------
        let mut twb_poll = HashMap::new();
        let ident = prop.ident.to_token_stream();
        for field in prop.fields.iter() {
            let field_ident = field.ident.as_ref().unwrap().to_string();
            let ty = field.ty.to_token_stream().to_string();

            if field_ident == "deref_widget" {
                continue;
            }

            let _ = GetSet::create_get_set(&field_ident, &ty, &binds, template_ptrs, impls)?;

            // [根据binds生成相关双向绑定的代码] ------------------------------------------------------------------
            Self::handle_two_way_binding(
                &mut twb_poll,
                &binds,
                &field_ident,
                &ty,
                &mut impls.traits_impl.0.widget.handle_event,
            )?;
        }
        impls.self_ref_impl.extend(GetSet::getter_setter(&ident));
        // [双向绑定初始化相关的代码] ------------------------------------------------------------------------------
        // 初始化添加到after_apply_from_doc中的初始化双向绑定池的代码
        let twb_poll = TWBPollBuilder(twb_poll);
        let _ = twb_poll.init_tk(prop.ident.to_token_stream()).map(|tk| {
            impls
                .traits_impl
                .0
                .live_hook
                .push(tk, LiveHookType::AfterApplyFromDoc);
        });
        // [处理sugar相关的代码] ---------------------------------------------------------------------------------
        // - [通过tmeplate_ptrs给prop添加组件指针] ----------------------------------------------------------------
        Self::handle_sugar(prop, template_ptrs, impls)?;
        // [添加双向绑定池] --------------------------------------------------------------------------------------
        if twb_poll.is_empty() {
            Ok(None)
        } else {
            Self::append_twb_pool(prop)?;
            Ok(Some(twb_poll))
        }
    }

    /// 处理所有双向绑定用到的变量和组件之间的关系，生成添加到handle_event中触发组件事件的代码
    /// 例如：当使用者给checkbox到selected绑定变量时，用户点击checkbox会触发checbox的clicked事件，来更新selected的值
    /// 但实际上用户并没有显示的添加checkbox的@clicked的回调函数，这个回调函数是由双向绑定池自动生成的，属于隐式回调
    fn handle_two_way_binding(
        twb_poll: &mut HashMap<String, String>,
        binds: &PropBinds,
        field: &str,
        ty: &str,
        handle_event: &mut HandleEvent,
    ) -> Result<(), Error> {
        // 获取使用了字段的所有组件
        if let Some(widgets) = binds.get(field) {
            for widget in widgets {
                let PropWidget { id, widget, prop, .. } = widget;
                // 添加到双向绑定池中
                twb_poll.insert(field.to_string(), ty.to_string());
                // 添加到handle_event中触发组件事件的代码
                handle_event
                    .c_refs
                    .insert(CRef::new(id.to_string(), widget.to_string()));

                if let Some(event) = BuiltinWidget::twb_event(&widget, &prop) {
                    handle_event.callbacks.insert(CallbackStmt::new(
                        id.to_string(),
                        field.to_string(),
                        prop.to_string(),
                        event,
                    ));
                }
            }
        }

        Ok(())
    }

    fn handle_sugar(
        prop: &mut ItemStruct,
        ptrs: &TemplatePtrs,
        impls: &mut Impls,
    ) -> Result<(), Error> {
        SugarScript::visit(prop, ptrs, impls)
    }

    fn append_twb_pool(prop: &mut ItemStruct) -> Result<(), Error> {
        match &mut prop.fields {
            Fields::Named(fields) => {
                let field = TWBPollBuilder::field_token_stream();
                fields.named.push(field);
                return Ok(());
            }
            _ => {
                return Err(CompilerError::runtime(
                    "Makepad Compiler - Script",
                    "prop should be a struct with named fields",
                )
                .into())
            }
        }
    }
}
