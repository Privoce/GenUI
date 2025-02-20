use quote::{quote, ToTokens};
use syn::ItemFn;

use crate::error::{AttrMacroError, SCResult};

/// # 表示网络的生命周期的回调枚举
/// 当前只支持Http协议
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NetwrokLifeCycle {
    Http(HttpLifeCycle),
    // WebSocket(WebSocketLifeCycle), TODO!("makepad 未支持WebSocket")
}

impl TryFrom<(&str, &ItemFn)> for NetwrokLifeCycle {
    type Error = crate::error::Error;

    fn try_from(value: (&str, &ItemFn)) -> Result<Self, Self::Error> {
        let mut http = HttpLifeCycle::default();
        match value.0 {
            "http_response" => {
                if http.responses.is_none() {
                    http.responses.replace(vec![value.1.clone()]);
                } else {
                    http.responses.as_mut().unwrap().push(value.1.clone());
                }
            }
            "request_error" => {
                http.request_error.replace(value.1.clone());
            }
            "progress" => {
                http.progress.replace(value.1.clone());
            }
            "stream_response" => {
                http.stream_response.replace(value.1.clone());
            }
            "stream_complete" => {
                http.stream_complete.replace(value.1.clone());
            }
            _ => {
                return Err(AttrMacroError::LifeCycleConflict(value.0.to_string()).into());
            }
        }
        Ok(http.into())
    }
}

impl From<HttpLifeCycle> for NetwrokLifeCycle {
    fn from(http: HttpLifeCycle) -> Self {
        NetwrokLifeCycle::Http(http)
    }
}
/// # 表示Http协议网络的生命周期回调
#[derive(Debug, Clone, Default)]
pub struct HttpLifeCycle {
    /// 表示响应的回调
    /// 使用`#[http_response]`标记
    pub responses: Option<Vec<ItemFn>>,
    /// 表示错误的回调
    /// 使用`#[request_error]`标记
    pub request_error: Option<ItemFn>,
    /// 进度回调
    /// 使用`#[progress]`标记
    pub progress: Option<ItemFn>,
    /// 流响应回调
    /// 使用`#[stream_response]`标记
    pub stream_response: Option<ItemFn>,
    /// 完成回调
    /// 使用`#[stream_complete]`标记
    pub stream_complete: Option<ItemFn>,
}

impl HttpLifeCycle {
    pub fn push(&mut self, attr: &str, item: &ItemFn) -> SCResult<()> {
        match attr {
            "http_response" => {
                if let Some(responses) = self.responses.as_mut() {
                    responses.push(item.clone());
                } else {
                    self.responses.replace(vec![item.clone()]);
                }
            }
            "request_error" => {
                self.request_error.replace(item.clone());
            }
            "progress" => {
                self.progress.replace(item.clone());
            }
            "stream_response" => {
                self.stream_response.replace(item.clone());
            }
            "stream_complete" => {
                self.stream_complete.replace(item.clone());
            }
            _ => {
                return Err(AttrMacroError::LifeCycleConflict(attr.to_string()).into());
            }
        }
        Ok(())
    }
}

impl ToTokens for HttpLifeCycle {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(response) = self.responses.as_ref() {
            tokens.extend(quote! {
                #(#response)*
            });
        }
        if let Some(error) = self.request_error.as_ref() {
            tokens.extend(error.to_token_stream());
        }
        if let Some(progress) = self.progress.as_ref() {
            tokens.extend(progress.to_token_stream());
        }
        if let Some(stream) = self.stream_response.as_ref() {
            tokens.extend(stream.to_token_stream());
        }
        if let Some(stream_complete) = self.stream_complete.as_ref() {
            tokens.extend(stream_complete.to_token_stream());
        }
    }
}

// pub struct WebSocketLifeCycle{
//     pub on_open: Option<ItemFn>,
//     pub on_message: Option<ItemFn>,
//     pub on_close: Option<ItemFn>,
//     pub on_error: Option<ItemFn>,
// }
