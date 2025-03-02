use std::{collections::HashSet, hash::Hash};
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, parse_str, Stmt};
use crate::{builtin::BuiltinWidget, model::CallbackComponent, str_to_tk, two_way_binding::TWBPollBuilder};

/// 对于Widget来说
/// draw_walk是必须实现的
/// 其他的方法是可选的
/// ## 必须:
/// ```rust
/// impl Widget for Easylb {
///     fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
///         self.view.draw_walk(cx, scope, walk)
///     }
///     fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
///         let _actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct WidgetTrait {
    /// 必须实现
    pub draw_walk: TokenStream,
    /// 可选实现
    pub handle_event: HandleEvent,
    pub widget: Option<TokenStream>,
    pub widgets: Option<TokenStream>,
    pub widget_id: Option<TokenStream>,
    pub widget_to_data: Option<TokenStream>,
    pub data_to_widget: Option<TokenStream>,
    pub draw: Option<TokenStream>,
    pub draw_walk_all: Option<TokenStream>,
    pub is_visible: TokenStream,
    pub draw_all: Option<TokenStream>,
    pub text: Option<TokenStream>,
    pub set_text: Option<TokenStream>,
    pub set_text_and_redraw: Option<TokenStream>,
    pub ref_cast_type_id: Option<TokenStream>,
}

impl Default for WidgetTrait {
    fn default() -> Self {
        Self {
            draw_walk: default_draw_walk(),
            handle_event: HandleEvent::default(),
            widget: Default::default(),
            widgets: Default::default(),
            widget_id: Default::default(),
            widget_to_data: Default::default(),
            data_to_widget: Default::default(),
            draw: Default::default(),
            draw_walk_all: Default::default(),
            is_visible: default_is_visible(),
            draw_all: Default::default(),
            text: Default::default(),
            set_text: Default::default(),
            set_text_and_redraw: Default::default(),
            ref_cast_type_id: Default::default(),
        }
    }
}

impl WidgetTrait {
    pub fn draw_walk_tk(&self) -> TokenStream {
        let draw_walk = &self.draw_walk;
        quote! {
            #[allow(unused_variables)]
            fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
                #draw_walk
            }
        }
    }
    pub fn handle_event_tk(&self, twb_poll: Option<&TWBPollBuilder>) -> TokenStream {
        let handle_event = self.handle_event.to_token_stream(twb_poll);
        quote! {
            #[allow(unused_variables)]
            fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                #handle_event
            }
        }
    }
    pub fn is_visible_tk(&self) -> TokenStream {
        let is_visible = &self.is_visible;
        quote! {
            #[allow(unused_variables)]
            fn is_visible(&self) -> bool {
                #is_visible
            }
        }
    }
    pub fn to_token_stream<TK>(&self, target: TK, twb_poll: Option<&TWBPollBuilder>) -> TokenStream
    where
        TK: Into<TokenStream>,
    {
        let target = target.into();
        let draw_walk = self.draw_walk_tk();
        let handle_event = self.handle_event_tk(twb_poll);
        let widget = &self.widget;
        let widgets = &self.widgets;
        let widget_id = &self.widget_id;
        let widget_to_data = &self.widget_to_data;
        let data_to_widget = &self.data_to_widget;
        let draw = &self.draw;
        let draw_walk_all = &self.draw_walk_all;
        let is_visible = &self.is_visible_tk();
        let draw_all = &self.draw_all;
        let text = &self.text;
        let set_text = &self.set_text;
        let set_text_and_redraw = &self.set_text_and_redraw;
        let ref_cast_type_id = &self.ref_cast_type_id;

        quote! {
            impl Widget for #target{
                #draw_walk
                #handle_event
                #widget
                #widgets
                #widget_id
                #widget_to_data
                #data_to_widget
                #draw
                #draw_walk_all
                #is_visible
                #draw_all
                #text
                #set_text
                #set_text_and_redraw
                #ref_cast_type_id
            }
        }
    }
}

fn default_draw_walk() -> TokenStream {
    quote! {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
}

fn default_is_visible() -> TokenStream {
    quote! {
        self.visible
    }
}

#[derive(Debug, Clone, Default)]
pub struct HandleEvent {
    pub c_refs: HashSet<CRef>,
    pub callbacks: HashSet<CallbackStmt>,
    pub widget_match_event: bool
}

impl HandleEvent {
    pub fn to_token_stream(&self, twb_poll: Option<&TWBPollBuilder>) -> TokenStream {
        let c_refs = self.c_refs.iter();
        let callbacks = self.callbacks.iter().map(|c| c.to_token_stream(twb_poll));
        let call_widget = if self.widget_match_event{
            Some(quote! {
                self.widget_match_event(cx, event, scope);
            })
        }else{
            None
        };
        quote! {
            #call_widget
            #(#c_refs)*
            #(#callbacks)*
        }
    }
}

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct CRef {
    pub id: String,
    pub name: String,
}

impl CRef {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

impl ToTokens for CRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = parse_str::<TokenStream>(&self.id).unwrap();
        let snake_name =
            parse_str::<TokenStream>(&BuiltinWidget::builtin_name_or_snake(&self.name)).unwrap();

        tokens.extend(quote! {
            let #id = self.#snake_name(id!(#id));
        });
    }
}

#[derive(Debug, Clone, Default)]
pub struct CallbackStmt {
    pub id: String,
    /// 事件回调返回的参数
    pub param: Option<String>,
    /// 绑定的变量引起的组件的事件名
    pub event: String,
    /// 绑定的变量名, 由这个变量名可以查找到对应修改的属性
    pub bind: String,
    /// 绑定的属性名
    pub prop: String,
    /// 这个回调中调用的方法
    pub fns: Vec<Stmt>,
}

impl CallbackStmt {
    pub fn new(id: String, bind: String, prop: String, event: String) -> Self {
        CallbackStmt {
            id,
            param: None,
            event,
            bind,
            prop,
            fns: vec![],
        }
    }
    pub fn fn_call_from_callback(&mut self, widget: &CallbackComponent) -> Result<(), Error>{
        let fn_name = str_to_tk!(&widget.callback_fn.func.name)?;
        let mut params = vec![];
        // [add param call] -------------------------------------------------------------------------------------------
        if self.param.is_some(){
            params.push(quote! {param});
        }
        // [add params from callback function] ------------------------------------------------------------------------
        if let Some(params_str) = widget.callback_fn.func.params_str() {
            params.push(str_to_tk!(&params_str)?);
        }
        // [add cx, widget_id as widget_ref] --------------------------------------------------------------------------
        let widget_id = str_to_tk!(widget.id)?;
        params.extend(vec![
            quote! {cx},
            // quote! {&#widget_id} // 暂时不添加类型引用
        ]);
        // [set back to self.fns] -------------------------------------------------------------------------------------
        self.fns.push(parse_quote!{
            self.#fn_name(#(#params),*);
        });
        Ok(())
    }

    pub fn to_token_stream(&self, twb_poll: Option<&TWBPollBuilder>) -> TokenStream {
        let id = parse_str::<TokenStream>(&self.id).unwrap();
        let param = if self.param.is_some() {
            quote! {param}
        } else {
            quote! {_}
        };
        let get_fn = parse_str::<TokenStream>(format!("get_{}", self.prop).as_str()).unwrap();
        let event = parse_str::<TokenStream>(&self.event).unwrap();

        let twb = if !self.bind.is_empty() {
            twb_poll
                .map(|twb_poll| {
                    if twb_poll.0.get(&self.bind).is_some() {
                        let on_change_ident = parse_str::<TokenStream>(
                            format!("on_{}_change", self.bind).as_str(),
                        ).unwrap();
                        let bind = parse_str::<TokenStream>(&self.bind).unwrap();
                        
                        Some(quote! {
                            let new_state = #id.#get_fn();
                            self.#bind = new_state.clone();
                            if let Some(on_change_callback) = self.twb_poll.#on_change_ident.as_ref(){
                                on_change_callback(cx, new_state);
                            }
                        })
                    } else {
                        None
                    }
                })
                .flatten()
        } else {
            None
        };

        let fns = &self.fns;

        quote! {
            if let Some(#param) = #id.#event(&actions) {
                #twb
                #(#fns)*
            }
        }
    }
}

impl Hash for CallbackStmt {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.event.hash(state);
        // self.bind.hash(state);
        // self.prop.hash(state);
    }
}

impl PartialEq for CallbackStmt {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.event == other.event
            // && self.bind == other.bind
            // && self.prop == other.prop
    }
}

impl Eq for CallbackStmt {}

impl From<&str> for CallbackStmt {
    fn from(value: &str) -> Self {
        CallbackStmt::new(value.to_string(), String::new(), String::new(), String::new())
    }
}

impl From<&String> for CallbackStmt {
    fn from(value: &String) -> Self {
        CallbackStmt::new(value.to_string(), String::new(), String::new(), String::new())
    }
}
