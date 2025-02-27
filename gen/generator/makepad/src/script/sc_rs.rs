use std::sync::{Arc, RwLock};

use gen_analyzer::Polls;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rssyin::{analyzer::ScriptAnalyzer, bridger::ScriptBridger};
use syn::{ItemEnum, Stmt};

use crate::{
    compiler::Context,
    model::TemplatePtrs,
    two_way_binding::TWBPollBuilder,
    visitor::{InstanceLzVisitor, PropLzVisitor},
};

use super::{Impls, LiveComponent};

#[derive(Debug, Clone, Default)]
pub struct ScRs {
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
            instance,
            event,
            impl_prop,
            others,
        } = ScriptAnalyzer::analyze(&code).map_err(|e| Error::from(e.to_string()))?;
        // [ident] -------------------------------------------------------------------------------------------
        let mut sc_rs = ScRs::default();
        let mut impls = Impls::default(&ident, impl_prop);

        sc_rs.ident = Some(ident);
        // [prop] --------------------------------------------------------------------------------------------
        let mut prop = prop.expect("prop is required in component!");
        let polls = polls.read().unwrap();
        PropLzVisitor::visit(&mut prop, template_ptrs, &mut impls, polls.binds.as_ref())?;

        // [live_struct] -------------------------------------------------------------------------------------
        sc_rs.impls = Some(impls);
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
        let live_struct = Some(LiveComponent::default(&ident));
        ScRs {
            ident: Some(ident),
            live_struct,
            impls: Some(Impls::default()),
            ..Default::default()
        }
    }
}

impl ToTokens for ScRs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uses = self.uses.as_ref();
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
        self.others.as_ref().map(|others| {
            tokens.extend(quote! {
                #(#others)*
            });
        });
    }
}
