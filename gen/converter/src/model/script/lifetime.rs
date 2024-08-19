use proc_macro2::TokenStream;
use quote::ToTokens;
/// GenUI内置组件生命周期事件
/// 目前只设置两种事件
#[derive(Debug, Clone, Default)]
pub struct LifeTime {
    /// 启动事件
    pub startup: Option<syn::StmtMacro>,
    /// 关闭事件
    pub shutdown: Option<syn::StmtMacro>,
}

impl LifeTime {
    pub fn set_startup(&mut self, startup: syn::StmtMacro) {
        let _ = self.startup.replace(startup);
    }
    pub fn set_shutdown(&mut self, shutdown: syn::StmtMacro) {
        let _ = self.shutdown.replace(shutdown);
    }
    /// 获取启动事件中的执行的代码
    pub fn startup_token(&self) -> Option<&TokenStream> {
        match &self.startup {
            Some(startup) => Some(&startup.mac.tokens),
            None => None,
        }
    }
    pub fn shutdown_token(&self) -> Option<&TokenStream> {
        match &self.shutdown {
            Some(shutdown) => Some(&shutdown.mac.tokens),
            None => None,
        }
    }
    pub fn is_static(&self) -> bool {
        self.startup.is_none() && self.shutdown.is_none()
    }
}

impl ToTokens for LifeTime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let LifeTime { startup, shutdown } = self;

        if let Some(startup) = startup {
            tokens.extend(startup.to_token_stream());
        }
        if let Some(shutdown) = shutdown {
            tokens.extend(shutdown.to_token_stream());
        }
    }
}
