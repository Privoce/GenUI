mod r#impl;
mod live_struct;
mod sc_rs;

use std::sync::{Arc, RwLock};

use gen_analyzer::Polls;
// use gen_mk_script_objs::makepad::{lifetime::LifeTime, ScriptBridger};
use gen_utils::error::Error;
pub use live_struct::*;
pub use r#impl::*;
pub use sc_rs::ScRs;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{compiler::Context, model::TemplatePtrs};

/// Makepad中的Rust代码
#[derive(Debug, Clone)]
pub enum Script {
    /// rust代码, 直接使用syn::File，并按原本形式输出，用于表示与makepad无关的rust代码
    Rs(syn::File),
    /// 经过包装处理的rust，用于表示makepad中的rust代码，这些代码需要进行一些处理
    ScRs(ScRs),
}

impl Script {
    pub fn handle(
        sc: gen_analyzer::Script,
        ctx: &mut Context,
        polls: Arc<RwLock<Polls>>,
        template_ptrs: &TemplatePtrs,
        ident: TokenStream,
    ) -> Result<Self, Error> {
        Ok(ScRs::new(sc, ctx, polls, template_ptrs, ident)?.into())
    }
    pub fn default(ident: TokenStream) -> Self {
        ScRs::default_sc(ident).into()
    }
}

impl From<ScRs> for Script {
    fn from(sc_rs: ScRs) -> Self {
        Script::ScRs(sc_rs)
    }
}

// only use in single_script
impl From<syn::File> for Script {
    fn from(file: syn::File) -> Self {
        Script::Rs(file)
    }
}

impl ToTokens for Script {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Script::Rs(file) => {
                file.to_tokens(tokens);
            }
            Script::ScRs(sc_rs) => {
                sc_rs.to_tokens(tokens);
            }
        }
    }
}
