use quote::ToTokens;
use syn::{Ident, ItemFn};

use crate::error::{AttrMacroError, SCResult};

/// # 表示app的生命周期回调
/// see makepad::Event
/// ## 设计原则
/// 将所有的生命周期函数都放在一个结构体中，这样可以方便的管理app的生命周期，编译时确定好内存布局，性能稍优于动态调用
#[derive(Debug, Clone, Default)]
pub struct AppLifeTime {
    /// 表示app的启动函数 The application has just been created.
    /// 使用`#[startup]`标记
    /// ```
    /// #[startup]
    /// fn app_start(){
    ///     // do something
    /// }
    /// ```
    pub startup: Option<ItemFn>,
    /// 表示app的关闭函数 The application is being shut down is about to close and be destroyed.
    /// 使用`#[shutdown]`标记
    pub shutdown: Option<ItemFn>,
    /// 表示app被显示在前台 The application has been shown in the foreground and is visible to the user.
    /// 使用`#[foreground]`标记
    pub foreground: Option<ItemFn>,
    /// 表示app被隐藏在后台 The application has been hidden in the background and is no longer visible to the user.
    /// 使用`#[background]`标记
    pub background: Option<ItemFn>,
    /// 表示app正在被使用 The application is being used by the user.
    /// 使用`#[active]`标记
    pub resume: Option<ItemFn>,
    /// 表示app暂停 The application is being paused and is no longer being used by the user.
    /// 使用`#[pause]`标记
    pub pause: Option<ItemFn>,
    /// 表示app获得焦点 The application has gained focus.
    /// 使用`#[focus]`标记
    pub focus: Option<ItemFn>,
    /// 表示app失去焦点 The application has lost focus.
    /// 使用`#[focus_lost]`标记
    pub focus_lost: Option<ItemFn>,
    /// 表示app的绘制 The application is drawing.
    /// 使用`#[draw]`标记
    pub draw: Option<ItemFn>,
    /// 表示app使用了定时器 The application is using a timer.
    /// 使用`#[timer]`标记
    pub timer: Option<ItemFn>,
    // /// 表示app的网络回调 see [HttpLifeTime]
    // pub http: HttpLifeTime, // 废弃，在GenUI中AppMain是生成出来的，网络部分都是在组件内部构建的
    /// 表示app的信号回调 The application has received a signal.
    /// 使用`#[signal]`标记
    pub signal: Option<ItemFn>,
}

impl TryFrom<(&Ident, &ItemFn)> for AppLifeTime {
    type Error = crate::error::Error;

    fn try_from(value: (&Ident, &ItemFn)) -> Result<Self, Self::Error> {
        let mut res = AppLifeTime::default();

        let attr = value.0.to_string();
        match attr.as_str() {
            "startup" => {
                res.startup.replace(value.1.clone());
            }
            "shutdown" => {
                res.shutdown.replace(value.1.clone());
            }
            "foreground" => {
                res.foreground.replace(value.1.clone());
            }
            "background" => {
                res.background.replace(value.1.clone());
            }
            "resume" => {
                res.resume.replace(value.1.clone());
            }
            "pause" => {
                res.pause.replace(value.1.clone());
            }
            "focus" => {
                res.focus.replace(value.1.clone());
            }
            "focus_lost" => {
                res.focus_lost.replace(value.1.clone());
            }
            "draw" => {
                res.draw.replace(value.1.clone());
            }
            "timer" => {
                res.timer.replace(value.1.clone());
            }
            "signal" => {
                res.signal.replace(value.1.clone());
            }
            _ => {
                // check is network?
                // let _ = res.http.push(&attr, value.1)?;
                return Err(AttrMacroError::NoLifeTimeMacro.into());
            }
        }
        Ok(res)
    }
}

impl AppLifeTime {
    pub fn push(&mut self, item: &ItemFn) -> SCResult<()> {
        if item.attrs.is_empty() {
            return Err(AttrMacroError::NoLifeTimeMacro.into());
        }

        if let Some(attr) = item.attrs[0].path().get_ident() {
            let attr = attr.to_string();
            match attr.as_str() {
                "startup" => {
                    self.startup.replace(item.clone());
                }
                "shutdown" => {
                    self.shutdown.replace(item.clone());
                }
                "foreground" => {
                    self.foreground.replace(item.clone());
                }
                "background" => {
                    self.background.replace(item.clone());
                }
                "resume" => {
                    self.resume.replace(item.clone());
                }
                "pause" => {
                    self.pause.replace(item.clone());
                }
                "focus" => {
                    self.focus.replace(item.clone());
                }
                "focus_lost" => {
                    self.focus_lost.replace(item.clone());
                }
                "draw" => {
                    self.draw.replace(item.clone());
                }
                "timer" => {
                    self.timer.replace(item.clone());
                }
                "signal" => {
                    self.signal.replace(item.clone());
                }
                _ => {
                    // check is network?
                    // return self.http.push(&attr, item);
                    return Err(AttrMacroError::NoLifeTimeMacro.into());
                }
            }
            return Ok(());
        }

        Err(AttrMacroError::NoLifeTimeMacro.into())
    }
}

impl ToTokens for AppLifeTime {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(startup) = &self.startup {
            startup.to_tokens(tokens);
        }
        if let Some(shutdown) = &self.shutdown {
            shutdown.to_tokens(tokens);
        }
        if let Some(foreground) = &self.foreground {
            foreground.to_tokens(tokens);
        }
        if let Some(background) = &self.background {
            background.to_tokens(tokens);
        }
        if let Some(resume) = &self.resume {
            resume.to_tokens(tokens);
        }
        if let Some(pause) = &self.pause {
            pause.to_tokens(tokens);
        }
        if let Some(focus) = &self.focus {
            focus.to_tokens(tokens);
        }
        if let Some(focus_lost) = &self.focus_lost {
            focus_lost.to_tokens(tokens);
        }
        if let Some(draw) = &self.draw {
            draw.to_tokens(tokens);
        }
        if let Some(timer) = &self.timer {
            timer.to_tokens(tokens);
        }
        if let Some(signal) = &self.signal {
            signal.to_tokens(tokens);
        }
        // self.http.to_tokens(tokens);
    }
}
