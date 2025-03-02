pub mod input;

use gen_dyn_run::DynProcessor;

use gen_utils::error::{CompilerError, Error};
use input::Input;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use syn::{parse_quote, FnArg, ImplItemFn, ItemFn, Signature, Stmt, Type};

use crate::{
    compiler::WidgetPoll,
    model::{
        traits::{ImplLiveHook, LiveHookType, WidgetMatchEventType},
        PropBinds,
    },
    script::Impls,
};

/// # 表示生命周期的访问者
/// 声明周期需要处理的的代码类似于fn-callback中的代码
pub struct LifeCycleLzVisitor;

impl LifeCycleLzVisitor {
    pub fn visit<L>(item_fn: &mut ImplItemFn, life_cycle: L) -> Result<(), Error>
    where
        L: Into<LifeCycle>,
    {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LifeCycle {
    BeforeMount,
    Mounted,
    BeforeUpdate,
    Updated,
}

impl From<String> for LifeCycle {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for LifeCycle {
    fn from(value: &str) -> Self {
        match value {
            "before_mount" => LifeCycle::BeforeMount,
            "mounted" => LifeCycle::Mounted,
            "before_update" => LifeCycle::BeforeUpdate,
            "updated" => LifeCycle::Updated,
            _ => unreachable!(),
        }
    }
}
