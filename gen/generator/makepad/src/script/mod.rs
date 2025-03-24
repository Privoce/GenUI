mod r#impl;
mod live_struct;
mod router;
mod rs;

use gen_analyzer::Polls;
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};
// use gen_mk_script_objs::makepad::{lifetime::LifeTime, ScriptBridger};
use crate::{
    compiler::{Context, WidgetPoll},
    model::{TemplatePtrs, WidgetTemplate, WidgetType},
    token::use_default_all,
    two_way_binding::TWBPollBuilder,
    visitor::{EventLzVisitor, FnLzVisitor, InstanceLzVisitor, PropLzVisitor},
};
use gen_utils::error::Error;
pub use live_struct::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
pub use r#impl::*;
pub use router::*;
pub use rs::*;
use rssyin::{analyzer::ScriptAnalyzer, bridger::ScriptBridger};
use syn::{parse_quote, ItemEnum, Stmt};

/// Makepad中的Rust代码
#[derive(Debug, Clone)]
pub enum Script {
    Rust(RsScript),
    Router(RouterScript),
}

impl Script {
    /// only for Rust script
    pub fn new(
        sc: gen_analyzer::Script,
        ctx: &mut Context,
        polls: Arc<RwLock<Polls>>,
        widget_poll: &WidgetPoll,
        template_ptrs: &TemplatePtrs,
        template: Option<&WidgetTemplate>,
    ) -> Result<Self, Error> {
        Ok(Self::Rust(RsScript::new(
            sc,
            ctx,
            polls,
            widget_poll,
            template_ptrs,
            template,
        )?))
    }
    pub fn default(ident: TokenStream) -> Self {
        Self::Rust(RsScript::default(ident))
    }
    pub fn uses(&self) -> Option<TokenStream> {
        match self {
            Self::Rust(sc) => sc.uses.clone(),
            Self::Router(_) => None,
        }
    }
    pub fn patch(&mut self, patcher: &RsScript) -> () {
        if let Self::Rust(sc) = self {
            sc.patch(patcher);
        }
    }
}

impl TryFrom<(String, &mut Context)> for Script {
    type Error = Error;

    fn try_from(value: (String, &mut Context)) -> Result<Self, Self::Error> {
        let script_bridger =
            ScriptAnalyzer::analyze(&value.0).map_err(|e| Error::from(e.to_string()))?;

        if let Some(router) = script_bridger.router {
            Ok(Self::Router((router, value.1).try_into()?))
        } else {
            Ok(Self::Rust(script_bridger.try_into()?))
        }
    }
}

impl ToTokens for Script {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Rust(sc) => sc.to_tokens(tokens),
            Self::Router(sc) => sc.to_tokens(tokens),
        }
    }
}
