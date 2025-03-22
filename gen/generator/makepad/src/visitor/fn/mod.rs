// use super::InstanceOutput;
use crate::{
    compiler::{Context, WidgetPoll},
    model::{
        traits::{CRef, CallbackStmt, ImplLiveHook, LiveHookType, WidgetMatchEventType},
        CallbackComponent,
    },
    script::Impls,
    str_to_tk,
    two_way_binding::TWBPollBuilder,
};
use gen_analyzer::{Binds, Events};

use gen_dyn_run::DynProcessor;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use special_event::{SpecialEvent, SpecialEventVisitor};
use std::collections::HashSet;
use syn::{parse_quote, FnArg, ImplItem, ImplItemFn, ItemImpl};

// mod input;
mod replacer;
mod special_event;
// mod traits;
// mod utils;

// pub use input::Input;
pub use replacer::*;

use super::{LifeCycle, LifeCycleLzVisitor};
// pub use traits::FnVisitorImpl;
// pub use utils::*;

const LIFECYCLE: [&str; 4] = ["before_mount", "mounted", "before_update", "updated"];
const SPECIAL_EVENT: [&str; 1] = ["http_response"];

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
///     #[before_mount]
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
/// 4. 将当前方法的调用设置到handle_event中
/// ## 功能2: 中间方法
/// 对于中间方法，它不涉及到事件回调，基本只是一个普通方法，若参数中含有self|&self|&mut self，则需要增加cx: &mut Cx
/// ## 功能3: 生命周期钩子
/// 生命周期是GenUI中的一个重要概念，它是一个特殊的方法，用于在组件的生命周期中执行一些操作，如`before_mount`，`mounted`等
/// 这些操作在Makepad中也是非常重要的，因此我们需要将这些方法转为Makepad中的生命周期钩子
/// 目前提供的生命周期钩子有：
/// 1. `#[before_mount]` -> `fn after_new_from_doc(&mut self, cx: &mut Cx)` 表示组件结构已经创建，但还未应用到文档中 （makepad提供）
/// 2. `#[mounted]` -> `fn after_apply_from_doc(&mut self, cx: &mut Cx)` 表示组件已经应用到文档中并且已经渲染 （makepad提供）
/// 3. `#[before_update]` -> `fn do_before_each_upadte(&mut self, cx: &mut Cx)` 标识组件任意属性变化前触发 （由genui构建，makepad不提供）
/// 4. `#[updated]` -> `do_after_each_update(&mut self, cx: &mut Cx)` 表示组件中任意属性发生变化后触发 （由genui构建，makepad不提供）
/// ## 功能4: 特殊事件钩子
/// 在Makepad中的特殊事件目前只有一个，网络请求接收事件，使用`#[http_response]`进行方法标记，表示这个方法是一个网络请求接收事件
/// 而网络请求则是由插件提供的，插件代码是靠动态库构建的依赖上下文系统注入（see 功能1-5）
/// ## 功能5: 通用转换
/// 1. 在方法中检查是否含有`c_ref!()`宏，需要替换为makepad的组件引用（基于静态分析，通用）
/// 2. 在方法中检查是否含有编译器上下文注册的宏，需要对应替换为编译器提供的宏的转换代码（基于动态库构建，通用）
#[derive(Debug)]
pub struct FnLzVisitor;
impl FnLzVisitor {
    /// 基于编译特性，除了中间方法外，其他所有的方法都需要进行延迟处理，因为其他的方法都可能会使用到中间方法
    /// 在这里其实只需要将通用转换延后
    pub fn visit(
        mut self_impl: ItemImpl,
        impls: &mut Impls,
        twb_poll: Option<&TWBPollBuilder>,
        binds: Option<&Binds>,
        events: Option<&Events>,
        widget_poll: &WidgetPoll,
        ctx: &Context,
    ) -> Result<(), Error> {
        dbg!(&twb_poll);
        fn convert_fns(
            item_fn: &mut ImplItemFn,
            widget_poll: &WidgetPoll,
            binds: Option<&Binds>,
            signal_fns: &Vec<String>,
            processor: Option<&DynProcessor>,
            twb_poll: Option<&TWBPollBuilder>,
        ) -> Result<(), Error> {
            // [通用转化] --------------------------------------------------------------------------------
            let fields = twb_poll.as_ref().map(|x| x.fields()).unwrap_or_default();
            visit_fns(item_fn, fields, widget_poll, binds, &signal_fns, processor)
        }

        // 记录增加了cx: &mut Cx的中间方法的ident，用于在visit_fns中提供替换支持
        let mut signal_fns = vec![];
        let mut self_events = vec![];
        let mut lifecycle_events = vec![];
        let mut special_events = vec![];

        for impl_item in self_impl.items.iter_mut() {
            // only care about fn
            let (res, impl_item) = if let ImplItem::Fn(item_fn) = impl_item {
                // [功能1: 转换普通的callback fn] --------------------------------------------------------------
                // if events is None, do not handle feature 1
                let mut res = if let Some(events) = events {
                    Self::convert_callback(impls, item_fn, events, ctx, widget_poll)?
                } else {
                    ConvertResult::Ignore
                };
                if let ConvertResult::Ignore = res {
                    // [功能3: 生命周期钩子] -------------------------------------------------------------------
                    if let Some(lifecycle) = item_fn
                        .attrs
                        .iter()
                        .find(|attr| LIFECYCLE.iter().any(|l| attr.path().is_ident(l)))
                        .map(|attr| attr.path().get_ident().unwrap().to_string())
                    {
                        let lifecycle = LifeCycleLzVisitor::visit(item_fn, lifecycle)?;
                        res = ConvertResult::LifeCycle(lifecycle);
                    }

                    // 只有结果依然是ignore才能进行其他功能处理
                    if matches!(res, ConvertResult::Ignore) {
                        // [功能4: 特殊事件钩子] -------------------------------------------------------------------
                        if let Some(special_event) = item_fn
                            .attrs
                            .iter()
                            .find(|attr| SPECIAL_EVENT.iter().any(|l| attr.path().is_ident(l)))
                            .map(|attr| attr.path().get_ident().unwrap().to_string())
                        {
                            let special = SpecialEventVisitor::visit(item_fn, special_event)?;
                            res = ConvertResult::SpecialEvent(special);
                        }
                    }

                    // [功能2: 中间方法] -----------------------------------------------------------------------
                    if matches!(res, ConvertResult::Ignore) {
                        // 既不是callback fn，也不是生命周期钩子，也不是特殊事件，那么就是中间方法，检查参数中是否含有任意self的引用，如果有则添加cx: &mut Cx
                        let mut has_self = false;
                        if item_fn.sig.inputs.iter().any(|args| {
                            if let FnArg::Receiver(_receiver) = args {
                                has_self = true;
                            }
                            has_self
                        }) {
                            item_fn.sig.inputs.push(parse_quote!(cx: &mut Cx));
                            signal_fns.push(item_fn.sig.ident.to_token_stream().to_string());
                        }
                    }
                }
                (res, impl_item.clone())
            } else {
                (ConvertResult::Ignore, impl_item.clone())
            };
            // [将impl_item添加到impls.self_impl中] ---------------------------------------------------------
            match res {
                ConvertResult::SelfImpl | ConvertResult::Ignore => {
                    self_events.push(impl_item);
                }
                ConvertResult::LifeCycle(life_cycle) => {
                    lifecycle_events.push((life_cycle, impl_item));
                }
                ConvertResult::SpecialEvent(special_event) => {
                    special_events.push((special_event, impl_item));
                }
            }
        }

        for mut impl_item in self_events {
            if let ImplItem::Fn(item_fn) = &mut impl_item {
                convert_fns(
                    item_fn,
                    widget_poll,
                    binds,
                    &signal_fns,
                    ctx.dyn_processor.as_ref(),
                    twb_poll,
                )?;
            }
            impls.self_impl.push(impl_item);
        }

        for (life_cycle, mut impl_item) in lifecycle_events {
            if let ImplItem::Fn(item_fn) = &mut impl_item {
                convert_fns(
                    item_fn,
                    widget_poll,
                    binds,
                    &signal_fns,
                    ctx.dyn_processor.as_ref(),
                    twb_poll,
                )?;
            }
            Self::set_life_cycle(life_cycle, impls, impl_item)?;
        }

        for (_special_event, impl_item) in special_events.iter_mut() {
            if let ImplItem::Fn(item_fn) = impl_item {
                convert_fns(
                    item_fn,
                    widget_poll,
                    binds,
                    &signal_fns,
                    ctx.dyn_processor.as_ref(),
                    twb_poll,
                )?;
            }
        }

        Self::set_special_event(impls, special_events)?;

        Ok(())
    }

    fn set_special_event(
        impls: &mut Impls,
        events: Vec<(SpecialEvent, ImplItem)>,
    ) -> Result<(), Error> {
        fn handle_http_response(
            responses: Vec<ImplItemFn>,
            impls: &mut Impls,
        ) -> Result<(), Error> {
            let mut tk = TokenStream::new();
            let push_trait = responses.len() > 0;
            for response in responses {
                let live_match_id = response.sig.ident.clone();

                tk.extend(quote! {
                    live_id!(#live_match_id) => self.#live_match_id(cx, response),
                });

                impls.self_impl.push(ImplItem::Fn(response));
            }

            if push_trait {
                let tk = quote! {
                    match request_id {
                        #tk
                        _ => {}
                    }
                };

                impls
                    .traits()
                    .push_widget_match_event(tk, WidgetMatchEventType::HttpResponse);
            }

            Ok(())
        }

        let mut http_responses = vec![];
        // [sort events] ---------------------------------------------------------------------------------------
        for (special, item) in events {
            if let ImplItem::Fn(item_fn) = item {
                match special {
                    SpecialEvent::HttpResponse => {
                        http_responses.push(item_fn);
                    }
                }
            } else {
                return Err(CompilerError::runtime(
                    "Makepad Compiler - Script",
                    "special event must be a fn",
                )
                .into());
            }
        }

        handle_http_response(http_responses, impls)?;

        Ok(())
    }

    fn set_life_cycle(
        life_cycle: LifeCycle,
        impls: &mut Impls,
        item: ImplItem,
    ) -> Result<(), Error> {
        if let ImplItem::Fn(item_fn) = item {
            let block = item_fn.block.to_token_stream();
            match life_cycle {
                LifeCycle::BeforeMount => {
                    impls
                        .traits()
                        .live_hook
                        .push(block, LiveHookType::AfterNewFromDoc);
                }
                LifeCycle::Mounted => {
                    impls
                        .traits()
                        .live_hook
                        .push(block, LiveHookType::AfterApplyFromDoc);
                }
                LifeCycle::BeforeUpdate => {}
                LifeCycle::Updated => {}
            }

            Ok(())
        } else {
            Err(
                CompilerError::runtime("Makepad Compiler - Script", "life_cycle must be a fn")
                    .into(),
            )
        }
    }

    fn convert_callback(
        impls: &mut Impls,
        item_fn: &mut ImplItemFn,
        events: &Events,
        _ctx: &Context,
        widget_poll: &WidgetPoll,
    ) -> Result<ConvertResult, Error> {
        // get fn name
        let fn_name = item_fn.sig.ident.to_string();
        // 标记是否需要处理事件，当可以在events中找到至少一次fn_name时，flag为true
        let mut flag = false;
        // check if fn is in event_fn
        for callback_component in events.iter().filter_map(|event| {
            event
                .callbacks
                .iter()
                .find(|(callback_fn_name, _callback_fn)| *callback_fn_name == &fn_name)
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
            ) {
                // 处理callback_stmt中的参数，这里需要判断是否有impl EventParam
                for p in item_fn.sig.inputs.iter_mut() {
                    if let FnArg::Typed(ty) = p {
                        if let syn::Type::ImplTrait(impl_trait) = &*ty.ty {
                            if let syn::TypeParamBound::Trait(trait_bound) = &impl_trait.bounds[0] {
                                if trait_bound.path.is_ident("EventParam") {
                                    // 获取真实的参数返回值的类型
                                    let callback_ty =
                                        callback_component.callback_ty(widget_poll).ok_or(
                                            Error::Compiler(CompilerError::runtime(
                                                "Makepad Compiler - Script",
                                                "can not find target param type",
                                            )),
                                        )?;
                                    let callback_ty_tk = str_to_tk!(&callback_ty)?;
                                    ty.ty = parse_quote!(#callback_ty_tk);
                                    // 给callback_stmt添加参数
                                    callback_stmt.param.replace(callback_ty);
                                }
                            }
                        }
                    }
                }
                // [为callback_stmt添加对应的方法调用] --------------------------------------------------------------
                callback_stmt.fn_call_from_callback(&callback_component)?;
                // [将callback_stmt设置回] ------------------------------------------------------------------------
                impls
                    .traits()
                    .widget
                    .handle_event
                    .callbacks
                    .insert(callback_stmt);
            }
        }

        if flag {
            // [检查是否含有`#[allow(unused_variables)]`，如果没有则增加] -----------------------------------------
            item_fn.attrs.push(parse_quote!(#[allow(unused_variables)]));
            // [添加参数cx: &mut Cx] ---------------------------------------------------------------------------
            item_fn.sig.inputs.push(parse_quote!(cx: &mut Cx));
            // 暂时不添加类型引用 --------------------------------------------------------------------------------
            // ❗️[添加类型引用] ---------------------------------------------------------------------------------
            Ok(ConvertResult::SelfImpl)
        } else {
            Ok(ConvertResult::Ignore)
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
    ) -> Option<CallbackStmt> {
        let fn_name = widget.callback_fn.event.to_string();
        let callback =
            CallbackStmt::new(widget.id.to_string(), String::new(), String::new(), fn_name);
        if let Some(callback) = callbacks.take(&callback) {
            Some(callback)
        } else {
            Some(callback)
        }
    }
}

enum ConvertResult {
    SelfImpl,
    LifeCycle(LifeCycle),
    SpecialEvent(SpecialEvent),
    Ignore,
}

#[cfg(test)]
mod test {
    #[test]
    fn try_find() {
        let a = vec!["a", "b", "c"];
        let target = vec!["a", "c", "d"];
        let res = a.iter().find(|x| target.iter().any(|y| *x == y));
        println!("{:?}", res);
    }
}
