mod r#impl;
mod live_struct;

// use gen_mk_script_objs::makepad::{lifetime::LifeTime, ScriptBridger};
use gen_utils::error::{CompilerError, Error};
pub use live_struct::*;
pub use r#impl::*;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rssyin::bridger::ScriptBridger;
// use rssyin::visitor::chain::traits::ChainVisitor;
use syn::{parse_quote, Item, ItemEnum, Stmt};

use crate::{
    compiler::{Context, WidgetPoll as ScPoll},
    model::{
        traits::{ImplLiveHook, LiveHookType},
        CallbackWidget, PropBinds, TemplatePtrs, WidgetTemplate,
    },
    two_way_binding::TWBPollBuilder,
    visitor::{EventLzVisitor, FnLzVisitor, InstanceLzVisitor, LifetimeLzVisitor, PropLzVisitor},
};

/// Makepad中的Rust代码
#[derive(Debug, Clone)]
pub enum Script {
    /// rust代码, 直接使用syn::File，并按原本形式输出，用于表示与makepad无关的rust代码
    Rs(syn::File),
    /// 经过包装处理的rust，用于表示makepad中的rust代码，这些代码需要进行一些处理
    WrapRs(WrapRs)    
}

#[derive(Debug, Clone)]
pub struct WrapRs{
    /// ident of the struct
    pub ident: Option<TokenStream>,
    /// rust uses
    pub uses: Option<TokenStream>,
    /// live struct
    pub live_struct: Option<LiveComponent>,
    /// events
    pub events: Option<Vec<ItemEnum>>,
    /// impls
    pub impls: Option<Impls>,
    pub twb_poll: Option<TWBPollBuilder>,
    pub others: Option<Vec<Stmt>>,
    /// 是否为单个script(即非封装组件的脚本代码)
    is_single: bool,
}


impl Script {
    pub fn default(ident: TokenStream) -> Self {
        let live_struct = Some(LiveComponent::default(&ident));
        Self {
            ident: Some(ident),
            uses: None,
            live_struct,
            events: None,
            impls: Some(Impls::default()),
            twb_poll: None,
            others: None,
            is_single: false,
        }
    }

    pub fn new(
        sc: gen_parser::Script,
        context: &mut Context,
        template: Option<&WidgetTemplate>,
        prop_poll: PropBinds,
        callback_poll: Vec<CallbackWidget>,
        template_ptrs: &TemplatePtrs,
        sc_poll: ScPoll,
    ) -> Result<Self, Error> {
        match sc {
            gen_parser::Script::Rs(block) => {
                // get script and convert to ScirptBridger
                let _ = context.sc_visitor_chain.visit_block_with(&block);
                let mut bridge_sc = context.sc_visitor_chain.bridge.clone();
                // check script.prop is some or add default
                if bridge_sc.prop.is_none() {
                    if let Some(template) = template.as_ref() {
                        let prop = template.is_define_root_and(|widget| widget.default_prop());
                        let _ = bridge_sc
                            .set_prop(prop)
                            .map_err(|e| Error::FromDynError(e.to_string()))?;
                    }
                }
                Self::handle(
                    context,
                    bridge_sc,
                    prop_poll,
                    callback_poll,
                    template_ptrs,
                    sc_poll,
                )
            }
            gen_parser::Script::Other { lang, .. } => Err(CompilerError::runtime(
                "Makepad Compiler - Script",
                &format!("Unsupported script language: {}", lang),
            )
            .into()),
        }
    }

    pub fn handle(
        context: &mut Context,
        bridger: ScriptBridger,
        prop_binds: PropBinds,
        callback_poll: Vec<CallbackWidget>,
        template_ptrs: &TemplatePtrs,
        sc_poll: ScPoll,
    ) -> Result<Script, Error> {
        let ScriptBridger {
            imports,
            prop,
            events,
            instance,
            mut fns,
            mut lifetimes,
            mut others,
        } = bridger;
        let mut prop = prop.expect("prop must be exist!");
        let ident = prop.ident.to_token_stream();
        let mut impls = Impls::default();
        // [处理组件实例初始化的代码] -------------------------------------------------------------------------------------
        let instance_output = if let Some(init) = instance.as_ref() {
            let mut lz_visitor = InstanceLzVisitor::new(&prop);
            let (output, tk) = lz_visitor.visit(init)?;
            if let Some(tk) = tk {
                let tk = quote! {
                    #(#tk)*
                };
                impls
                    .traits_impl
                    .0
                    .live_hook
                    .push(tk, LiveHookType::AfterApplyFromDoc);
            }
            Some(output)
        } else {
            None
        };
        // [使用PropLzVisitor处理prop] --------------------------------------------------------------------------------
        let twb_poll = PropLzVisitor::visit(&mut prop, &prop_binds, template_ptrs, &mut impls)?;
        if let Some(twb_poll) = twb_poll.as_ref() {
            if others.is_none() {
                others = Some(vec![]);
            }
            others.as_mut().unwrap().push(parse_quote! {
                #twb_poll
            });
        }
        // [处理event] ------------------------------------------------------------------------------------------------
        if let Some(events) = events.as_ref() {
            let _ = EventLzVisitor::visit(events, &mut impls)?;
        }
        // [处理fn-callback] ------------------------------------------------------------------------
        let fn_names = if let Some(mut fns) = fns.take() {
            // 获取所有fn的名称，目的是若其他方法中调用了这些fn，我们需要将其改为self.fn_name()的形式
            // 过滤出组件调用的回调方法，去除一些过程方法(即在组件中使用@xxx="fn_name"的方法才是后续处理的目标)
            let fn_names = fns
                .iter()
                .filter(|f| {
                    !callback_poll
                        .iter()
                        .any(|item| item.has(f.sig.ident.to_string().as_str()))
                })
                .map(|f| f.sig.ident.to_string())
                .collect::<Vec<_>>();

            for f in fns.iter_mut() {
                let callback_widget = callback_poll
                    .iter()
                    .find(|item| item.has(f.sig.ident.to_string().as_str()))
                    .cloned();
                // 构建出FnLzVisitor
                let mut fn_visitor = FnLzVisitor::new(
                    sc_poll.clone(),
                    callback_widget,
                    instance_output.clone(),
                    fn_names.clone(),
                );
                // 转换函数
                fn_visitor.visit(
                    &context.define_widget_poll,
                    &prop_binds,
                    f,
                    context.dyn_processor.as_ref(),
                    &mut impls,
                )?;
            }

            impls
                .self_impl
                .extend(fns.into_iter().map(|f| Stmt::Item(Item::Fn(f))).collect());
            fn_names
        } else {
            vec![]
        };
        // [处理lifetime] ---------------------------------------------------------------------------
        if let Some(lifetime) = lifetimes.as_mut() {
            if let LifeTime::Widget(widget_lifetime) = lifetime {
                let mut lifetime_visitor =
                    LifetimeLzVisitor::new(sc_poll.clone(), instance_output.clone(), fn_names);

                lifetime_visitor.visit(
                    widget_lifetime,
                    &mut impls,
                    &prop_binds,
                    context.dyn_processor.as_ref(),
                )?;
            }
        }

        // [clear bridge] --------------------------------------------------------------------------
        let _ = context.sc_visitor_chain.clear();

        Ok(Script {
            ident: Some(ident),
            uses: imports,
            live_struct: Some(prop.into()),
            events,
            impls: Some(impls),
            twb_poll,
            others,
            is_single: false,
        })
    }
}

pub fn handle_script(
    script: Option<gen_parser::Script>,
    context: &mut Context,
    template: Option<&WidgetTemplate>,
    prop_poll: PropBinds,
    callback_poll: Vec<CallbackWidget>,
    template_ptrs: &TemplatePtrs,
    sc_poll: ScPoll,
) -> Result<Option<Script>, Error> {
    if let Some(sc) = script {
        let sc = Script::new(
            sc,
            context,
            template,
            prop_poll,
            callback_poll,
            template_ptrs,
            sc_poll,
        )?;
        Ok(Some(sc))
    } else {
        return Ok(None);
    }
}

impl ToTokens for Script {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uses = self.uses.as_ref();
        if !self.is_single {
            let struct_ident = self.ident.as_ref().unwrap();
            let live_struct = self.live_struct.to_token_stream();
            let impls = self
                .impls
                .as_ref()
                .map(|impls| impls.to_token_stream(struct_ident, self.twb_poll.as_ref()));
            let events = self.events.as_ref().map(|events| {
                quote! {
                    #( #events )*
                }
            });
            tokens.extend(quote! {
                #uses
                #live_struct
                #events
                #impls
            });
        } else {
            tokens.extend(quote! {
                #uses
            });
        }

        self.others.as_ref().map(|others| {
            tokens.extend(quote! {
                #(#others)*
            });
        });
    }
}

// only use in single_script
// impl From<ScriptBridger> for Script {
//     fn from(value: ScriptBridger) -> Self {
//         let ScriptBridger {
//             imports,
//             events,
//             others,
//             ..
//         } = value;

//         Script {
//             ident: None,
//             uses: imports,
//             live_struct: None,
//             events,
//             impls: None,
//             others,
//             is_single: true,
//             twb_poll: None,
//         }
//     }
// }

impl From<syn::File> for Script{
    fn from(value: syn::File) -> Self {
        Script::Rs(value)
    }
}