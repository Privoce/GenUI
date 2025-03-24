use gen_utils::error::{CompilerError, Error};
use quote::ToTokens;
use rssyin::bridger::RouterTk;

use crate::compiler::Context;

#[derive(Debug, Clone)]
pub struct RouterScript(Router);

impl RouterScript {}

impl TryFrom<(RouterTk, &mut Context)> for RouterScript {
    type Error = Error;

    fn try_from(value: (RouterTk, &mut Context)) -> Result<Self, Self::Error> {
        // 从context中获取对应的router
        if let Some(routers) = value.1.routers.as_ref() {
            routers.get(&value.0.0)

        }
        Err(CompilerError::Conf(format!("{} router can not found in context, please check!", value.0.0)).into())
    }
}

impl ToTokens for RouterScript {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
