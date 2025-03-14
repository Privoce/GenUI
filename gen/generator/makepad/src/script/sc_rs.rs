use std::sync::{Arc, RwLock};

use gen_analyzer::Polls;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rssyin::{analyzer::ScriptAnalyzer, bridger::ScriptBridger};
use syn::{parse_quote, ItemEnum, Stmt};

use crate::{
    compiler::{Context, WidgetPoll},
    model::{TemplatePtrs, WidgetTemplate, WidgetType},
    two_way_binding::TWBPollBuilder,
    visitor::{EventLzVisitor, FnLzVisitor, InstanceLzVisitor, PropLzVisitor},
};

use super::{Impls, LiveComponent};

#[derive(Debug, Clone, Default)]
pub struct ScRs {
    /// ident of the struct
    pub ident: Option<TokenStream>,
    /// rust uses
    pub uses: Option<TokenStream>,
    /// live component
    pub live_component: Option<LiveComponent>,
    /// events
    pub events: Option<Vec<ItemEnum>>,
    /// impls
    pub impls: Option<Impls>,
    pub twb_poll: Option<TWBPollBuilder>,
    pub others: Option<Vec<Stmt>>,
}

impl ScRs {
    fn handle(
        code: String,
        ctx: &mut Context,
        polls: Arc<RwLock<Polls>>,
        widget_poll: &WidgetPoll,
        template_ptrs: &TemplatePtrs,
        template: Option<&WidgetTemplate>,
    ) -> Result<Self, Error> {
        let template = template.map_or_else(
            || {
                Err(Error::from(
                    "can not find root component identifier in template",
                ))
            },
            |t| Ok(t),
        )?;
        let ident = template.root_name();
        let ScriptBridger {
            imports,
            component,
            mut props,
            mut instance,
            event,
            impl_component,
            mut others,
        } = ScriptAnalyzer::analyze(&code).map_err(|e| Error::from(e.to_string()))?;
        // [datas] -------------------------------------------------------------------------------------------
        let mut sc_rs = ScRs::default();
        // 在这里暂时不把impl_component作为构建的一部分，有助于减少后续对impl_component内ImplItem的遍历个数
        let mut impls = Impls::default(&ident, None);
        // [component, two-way-binding, live_component] -----------------------------------------------------------
        // let mut component = component.expect("component is required in component!");
        let polls = polls.read().unwrap();
        let (twb, live_component) = if let Some(mut component) = component {
            let (twb, live_component) =
                PropLzVisitor::visit(&mut component,props.as_mut() ,template_ptrs, &mut impls, polls.binds.as_ref(), &mut others)?;
            // - [twb token stream for other_stmts] --------------------------------------------------------------
            if let Some(twb) = twb.as_ref() {
                others.push(parse_quote!(#twb));
            }
            // [instance for default() in others] ----------------------------------------------------------------
            // here we need to replace the Default trait ident for component struct
            if let Some(instance) = instance.as_mut() {
                let deref_prop_ident = component.ident.to_token_stream();
                InstanceLzVisitor::visit(instance, deref_prop_ident, &mut others);
            }
            others.push(parse_quote!(#component));
            (twb, Some(live_component))
        } else {
            (None, None)
        };
        // [events] ------------------------------------------------------------------------------------------
        if let Some(mut event) = event {
            let events = EventLzVisitor::visit(&mut event, &mut impls)?;
            if let WidgetType::Define(define_widget) = &template.ty {
                let snake_name = define_widget.snake_name();
                let name = define_widget.root_name().to_string();
                ctx.push_widget(
                    snake_name,
                    crate::model::AbsWidget::Define {
                        name,
                        props: twb.as_ref().map(|build| build.0.clone()),
                        events,
                    },
                );
            }
            others.push(parse_quote!(#event));
        }
        // [处理fn-callback] ----------------------------------------------------------------------------------
        if let Some(impl_component) = impl_component {
            // 消耗impl_component，所有内部处理的方法都会被放到impls.self_impl中
            FnLzVisitor::visit(
                impl_component,
                &mut impls,
                twb.as_ref(),
                polls.binds.as_ref(),
                polls.events.as_ref(),
                widget_poll,
                &ctx,
            )?;

            // // set to impls
            // impls.self_impl = impls.self_impl.patch(impl_component);
        }
        let _ = imports.map(|imports| {
            sc_rs.uses = Some(imports.to_token_stream());
        });
        sc_rs.ident = Some(ident);
        sc_rs.impls = Some(impls);
        sc_rs.live_component = live_component;
        sc_rs.twb_poll = twb;
        sc_rs.others = Some(others);
        Ok(sc_rs)
    }
    /// 处理并生成Makepad中的Rust代码
    pub fn new(
        sc: gen_analyzer::Script,
        ctx: &mut Context,
        polls: Arc<RwLock<Polls>>,
        widget_poll: &WidgetPoll,
        template_ptrs: &TemplatePtrs,
        template: Option<&WidgetTemplate>,
    ) -> Result<Self, Error> {
        match sc {
            gen_analyzer::Script::Rs(rs) => {
                Self::handle(rs, ctx, polls, widget_poll, template_ptrs, template)
            }
            gen_analyzer::Script::Other { lang, .. } => Err(CompilerError::runtime(
                "Makepad Compiler - Script",
                &format!("Unsupported script language: {}", lang),
            )
            .into()),
        }
    }
    /// 默认生成的Makepad中的Rust代码部分，只含有最基础页面结构, 用于没有任何动态交互的页面
    pub fn default_sc(ident: TokenStream) -> Self {
        let live_component = Some(LiveComponent::default(&ident));
        let impls = Some(Impls::default(&ident, None));
        ScRs {
            ident: Some(ident),
            live_component,
            impls,
            ..Default::default()
        }
    }

    /// 合并两个ScRs
    /// 只合并: uses, others
    pub fn patch(&mut self, patcher: &ScRs) -> () {
        patcher.uses.as_ref().map(|uses| {
            self.uses.get_or_insert_default().extend(uses.clone());
        });
        patcher.others.as_ref().map(|others| {
            self.others.get_or_insert_default().extend(others.clone());
        });
    }
}

impl ToTokens for ScRs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_ident = self.ident.as_ref().unwrap();
        let live_component = self.live_component.to_token_stream();
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
            #live_component
            #events
            #impls
        });
        self.others.as_ref().map(|others| {
            tokens.extend(quote! {
                #(#others)*
            });
        });
    }
}
