use super::InstanceOutput;
use crate::{
    compiler::{Context, WidgetPoll},
    model::{
        traits::{CRef, CallbackStmt, ImplLiveHook},
        CallbackWidget, PropBinds,
    },
    script::Impls,
};
use gen_analyzer::{Binds, CallbackFn, Events};
use gen_dyn_run::DynProcessor;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{parse_quote, parse_str, FnArg, ImplItem, ImplItemFn, ItemImpl};

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
/// 4. 在方法中检查是否含有`c_ref!()`宏，需要替换为makepad的组件引用（基于静态分析，通用）
/// 5. 在方法中检查是否含有编译器上下文注册的宏，需要对应替换为编译器提供的宏的转换代码（基于动态库构建，通用）
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
pub struct FnLzVisitor;
impl FnLzVisitor {
    pub fn visit(
        mut self_impl: ItemImpl,
        impls: &mut Impls,
        binds: Option<&Binds>,
        events: Option<&Events>,
        widget_poll: &WidgetPoll,
        ctx: &Context,
    ) -> Result<(), Error> {
        for impl_item in self_impl.items.iter_mut() {
            // only care about fn
            if let ImplItem::Fn(item_fn) = impl_item {
                // [功能1 + 2: 转换普通的callback fn 若不在event中则忽略] ---------------------------------------
                // if events is None, do not handle feature 1
                if let Some(events) = events {
                    Self::convert_callback(impls, item_fn, events, ctx)?;
                }
            }
        }

        Ok(())
    }

    fn convert_callback(
        impls: &mut Impls,
        item_fn: &mut ImplItemFn,
        events: &Events,
        ctx: &Context,
    ) -> Result<Step, Error> {
        // get fn name
        let fn_name = item_fn.sig.ident.to_string();
        // 标记是否需要处理事件，当可以在events中找到至少一次fn_name时，flag为true
        let mut flag = false;
        // check if fn is in event_fn
        for callback_component in events.iter().filter_map(|event| {
            event
                .callbacks
                .iter()
                .find(|(callback_fn_name, callback_fn)| callback_fn_name == &fn_name)
                .map(|(_, callback_fn)| CallbackComponent {
                    id: &event.id,
                    name: &event.name,
                    callback_fn,
                })
        }) {
            flag = true;
            // now we find all callback fn targets
            // [将当前方法的调用设置到handle_event中] --------------------------------------------------------------
            Self::has_or_set_cref(
                &callback_component,
                &mut impls.traits().widget.handle_event.c_refs,
            );
            if let Some(mut callback_stmt) = Self::get_or_create_event(
                &callback_component,
                &mut impls.traits().widget.handle_event.callbacks,
                &fn_name,
            ) {
                // 处理callback_stmt中的参数，这里需要判断是否有impl EventParam
                for p in item_fn.sig.inputs.iter() {
                    
                }
            }
        }

        if flag {
            // [检查是否含有`#[allow(unused_variables)]`，如果没有则增加] ---------------------------------------
            item_fn.attrs.push(parse_quote!(#[allow(unused_variables)]));
        }
    }

    fn has_or_set_cref(widget: &CallbackComponent, c_refs: &mut HashSet<CRef>) -> () {
        let c_ref = CRef {
            id: widget.id.to_string(),
            name: widget.name.to_string(),
        };
        if !c_refs.contains(&c_ref) {
            c_refs.insert(c_ref);
        }
    }

    fn get_or_create_event(
        widget: &CallbackComponent,
        callbacks: &mut HashSet<CallbackStmt>,
        fn_name: &str,
    ) -> Option<CallbackStmt> {
        let callback = CallbackStmt::new(
            widget.id.to_string(),
            String::new(),
            String::new(),
            fn_name.to_string(),
        );
        if let Some(callback) = callbacks.take(&callback) {
            Some(callback)
        } else {
            Some(callback)
        }
    }
}

struct CallbackComponent<'a> {
    id: &'a str,
    name: &'a str,
    callback_fn: &'a CallbackFn,
}

enum Step {
    /// 标识已经进行了回调的转换
    ConvertCallback,
    /// 标识被忽略，只有不需要被进行回调处理，生命周期处理，特殊事件的方法才会被忽略
    Ignore,
    /// 标识已经进行了生命周期的转换
    ConvertLifecycle,
    /// 标识已经进行了特殊事件的转换
    ConvertSpecialEvent,
    /// 标识处理下一个（一般用于并没有进行处理）
    /// 例如但生命周期事件并不会被在回调方法部分中进行处理，就会返回Next
    Next,
    /// 标识处理结束
    Finish,
}
