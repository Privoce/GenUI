use quote::ToTokens;
use syn::{Ident, ItemFn};

use crate::error::{AttrMacroError, SCResult};

use super::network::HttpLifeCycle;

/// # 表示一个widget的生命周期的回调
/// 当前只有before_create被直接支持，其他生命周期需要依赖`gen_lifetime_mk`这个包实现(TODO!)
#[derive(Debug, Clone, Default)]
pub struct WidgetLifeCycle {
    /// 表示初始化的回调, 对应makepad::LiveHook::after_new_before_apply (once)
    /// 使用`#[before_create]`标记
    pub before_create: Option<ItemFn>,
    /// 表示组件正式被创建，但还没被绘制（TODO）
    /// 使用`#[created]`标记
    pub created: Option<ItemFn>,
    /// 表示组件被绘制完成（TODO）
    /// 使用`#[mounted]`标记
    pub mounted: Option<ItemFn>,
    /// 表示组件被卸载（TODO）实现Drop trait for Widget
    /// 使用`#[unmounted]`标记
    pub unmounted: Option<ItemFn>,
    /// 表示网络请求的回调
    pub http: HttpLifeCycle,
}

impl TryFrom<(&Ident, &ItemFn)> for WidgetLifeCycle {
    type Error = crate::error::Error;

    fn try_from(value: (&Ident, &ItemFn)) -> Result<Self, Self::Error> {
        let mut res = WidgetLifeCycle::default();
        let attr = value.0.to_string();
        match attr.as_str() {
            "before_create" => {
                res.before_create.replace(value.1.clone());
            }
            "created" => {
                res.created.replace(value.1.clone());
            }
            "mounted" => {
                res.mounted.replace(value.1.clone());
            }
            "unmounted" => {
                res.unmounted.replace(value.1.clone());
            }
            _ => {
                // check is network?
                let _ = res.http.push(&attr, value.1)?;
            }
        }
        Ok(res)
    }
}

impl WidgetLifeCycle {
    pub fn push(&mut self, item: &ItemFn) -> SCResult<()> {
        if item.attrs.is_empty() {
            return Err(AttrMacroError::NoLifeCycleMacro.into());
        }

        if let Some(attr) = item.attrs[0].path().get_ident() {
            let attr = attr.to_string();
            match attr.as_str() {
                "before_create" => {
                    self.before_create.replace(item.clone());
                }
                "created" => {
                    self.created.replace(item.clone());
                }
                "mounted" => {
                    self.mounted.replace(item.clone());
                }
                "unmounted" => {
                    self.unmounted.replace(item.clone());
                }
                _ => {
                    // check is network?
                    let _ = self.http.push(&attr, item)?;
                }
            }
            return Ok(());
        }

        Err(AttrMacroError::NoLifeCycleMacro.into())
    }
}

impl ToTokens for WidgetLifeCycle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(before_create) = self.before_create.as_ref() {
            before_create.to_tokens(tokens);
        }
        if let Some(created) = self.created.as_ref() {
            created.to_tokens(tokens);
        }
        if let Some(mounted) = self.mounted.as_ref() {
            mounted.to_tokens(tokens);
        }
        if let Some(unmounted) = self.unmounted.as_ref() {
            unmounted.to_tokens(tokens);
        }
        self.http.to_tokens(tokens);
    }
}