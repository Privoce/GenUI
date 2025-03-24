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
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rssyin::{analyzer::ScriptAnalyzer, bridger::ScriptBridger};
use syn::{parse_quote, ItemEnum, Stmt};
pub use live_struct::*;
pub use r#impl::*;
pub use router::*;
pub use rs::*;

/// Makepad中的Rust代码
#[derive(Debug, Clone)]
pub enum Script {
    Rust(RsScript),
    Router(RouterScript),
}

impl FromStr for Script {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}