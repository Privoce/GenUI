use std::sync::{Arc, RwLock};

use gen_analyzer::Polls;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rssyin::{analyzer::ScriptAnalyzer, bridger::ScriptBridger};
use syn::{parse_quote, ItemEnum, Stmt};

use crate::{
    compiler::Context,
    model::TemplatePtrs,
    two_way_binding::TWBPollBuilder,
    visitor::{EventLzVisitor, InstanceLzVisitor, PropLzVisitor},
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
        template_ptrs: &TemplatePtrs,
        ident: TokenStream,
    ) -> Result<Self, Error> {
        let ScriptBridger {
            imports,
            prop,
            mut instance,
            event,
            impl_prop,
            mut others,
        } = ScriptAnalyzer::analyze(&code).map_err(|e| Error::from(e.to_string()))?;
        // [datas] -------------------------------------------------------------------------------------------
        let mut sc_rs = ScRs::default();
        // 在这里暂时不把impl_prop作为构建的一部分，有助于减少后续对impl_prop内ImplItem的遍历个数
        let mut impls = Impls::default(&ident, None);
        // [prop, two-way-binding, live_component] -----------------------------------------------------------
        let mut prop = prop.expect("prop is required in component!");
        let polls = polls.read().unwrap();
        let (twb, live_component) =
            PropLzVisitor::visit(&mut prop, template_ptrs, &mut impls, polls.binds.as_ref())?;
        // - [twb token stream for other_stmts] --------------------------------------------------------------
        if let Some(twb) = twb.as_ref() {
            others.push(parse_quote!(#twb));
        }
        // [instance for default() in others] ----------------------------------------------------------------
        // here we need to replace the Default trait ident for prop struct
        if let Some(instance) = instance.as_mut() {
            let deref_prop_ident = prop.ident.to_token_stream();
            InstanceLzVisitor::visit(instance, deref_prop_ident, &mut others);
        }
        // [events] ------------------------------------------------------------------------------------------
        if let Some(event) = event {
            let _ = EventLzVisitor::visit(event, &mut impls)?;
        }
        // [处理fn-callback] ----------------------------------------------------------------------------------
        if let Some(mut impl_prop) = impl_prop {



            // set to impls
            impls.self_impl = impls.self_impl.patch(impl_prop);
        }

        others.push(parse_quote!(#prop));
        let _ = imports.map(|imports| {
            sc_rs.uses = Some(imports.to_token_stream());
        });
        sc_rs.ident = Some(ident);
        sc_rs.impls = Some(impls);
        sc_rs.live_component = Some(live_component);
        sc_rs.twb_poll = twb;
        sc_rs.others = Some(others);
        Ok(sc_rs)
    }
    /// 处理并生成Makepad中的Rust代码
    pub fn new(
        sc: gen_analyzer::Script,
        ctx: &mut Context,
        polls: Arc<RwLock<Polls>>,
        template_ptrs: &TemplatePtrs,
        ident: TokenStream,
    ) -> Result<Self, Error> {
        match sc {
            gen_analyzer::Script::Rs(rs) => Self::handle(rs, ctx, polls, template_ptrs, ident),
            gen_analyzer::Script::Other { lang, code } => Err(CompilerError::runtime(
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
}

impl ToTokens for ScRs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uses = self.uses.as_ref();
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
            #uses
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
