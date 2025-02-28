use super::InstanceOutput;
use crate::{
    compiler::WidgetPoll,
    model::{
        traits::{CRef, CallbackStmt},
        CallbackWidget, PropBinds,
    },
    script::Impls,
};
use gen_dyn_run::DynProcessor;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{parse_quote, parse_str, FnArg};

// mod input;
// mod replacer;
// mod traits;
// mod utils;

// pub use input::Input;
// pub use replacer::*;
// pub use traits::FnVisitorImpl;
// pub use utils::*;
/// # Lazy fn(event) Visitor for Makepad
/// handle convert fn to real makepad fn
/// ```
/// #[component]
/// pub struct AComponent{
///     count: i32
///     // ...
/// }
/// 
/// impl AComponent{
///     // normal callback fn for event
///     fn a_btn_clicked(&mut self){
///        // set fn can do two way binding(update count field and component)
///        self.set_count(1);
///     }
///     // lifecycle hook
///     #[before_create]
///     fn do_before_create(&mut self){
///         println!("do before create");
///     }
///     // middle fn, means this fn just for internal use, not for event
///     fn do_something(&mut self){
///         println!("do something");
///     }
/// }
/// ```
/// 上面显示的代码是GenUI中的代码，在FnLzVisitor中，我们需要将这个代码转为Makepad中的代码
/// ## 功能1: 转换普通的callback fn
/// 对于普通的callback fn，其中的代码几乎不用改动，只需要将特殊的宏进行替换和增加一些参数即可，涉及`ra_ap_syntax`
/// 1. 检查是否含有`#[allow(unused_variables)]`，如果没有则增加
/// 2. 检查是否方法参数`impl EventParam`，如果有则需要根据静态分析的结果替换为对应的回调参数类型，如按钮的回调参数类型为`GButtonEventParam`
/// 3. 在方法参数，FnArg中添加`cx: &mut Cx`
/// 4. 在方法中检查是否含有`c_ref!()`宏，需要替换为makepad的组件引用（基于静态分析）
/// 5. 在方法中检查是否含有编译器上下文注册的宏，需要对应替换为编译器提供的宏的转换代码（基于动态库构建）
/// 6. 将当前方法的调用设置到handle_event中
/// ## 功能2: 忽略中间方法
/// 对于中间方法，它不涉及到事件回调，基本只是一个普通方法，不需要改动
/// ## 功能3: 生命周期钩子
/// 生命周期是GenUI中的一个重要概念，它是一个特殊的方法，用于在组件的生命周期中执行一些操作，如`before_create`，`after_create`等
/// 这些操作在Makepad中也是非常重要的，因此我们需要将这些方法转为Makepad中的生命周期钩子
/// 目前提供的生命周期钩子有：
/// 1. `#[before_mount]` -> `fn after_new_from_doc(&mut self, cx: &mut Cx)` 表示组件结构已经创建，但还未应用到文档中 （makepad提供）
/// 2. `#[mounted]` -> `fn after_apply_from_doc(&mut self, cx: &mut Cx)` 表示组件已经应用到文档中并且已经渲染 （makepad提供）
/// 3. `#[before_update]` -> `fn do_before_each_upadte(&mut self, cx: &mut Cx)` 标识组件任意属性变化前触发 （由genui构建，makepad不提供）
/// 4. `#[updated]` -> `do_after_each_update(&mut self, cx: &mut Cx)` 表示组件中任意属性发生变化后触发 （由genui构建，makepad不提供）
/// ## 功能4: 特殊事件钩子
/// 在Makepad中的特殊事件目前只有一个，网络请求接收事件，使用`#[http_response]`进行方法标记，表示这个方法是一个网络请求接收事件
/// 而网络请求则是由插件提供的，插件代码是靠动态库构建的依赖上下文系统注入（see 功能1-5）
#[derive(Debug)]
pub struct FnLzVisitor {

}

impl FnLzVisitor {
    
}