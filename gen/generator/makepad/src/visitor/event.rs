use std::collections::HashMap;

use gen_utils::{common::camel_to_snake, error::Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, parse_str, Fields, Ident, ImplItem, ItemEnum, Stmt, Variant};

use crate::script::Impls;

/// 根据提供的事件生成对应外部调用方法, 例如`enum A { B, C , None }`生成:
///
/// - (pub) fn b(&self, actions: &Actions) -> Option<()>{}
/// - (pub) fn c(&self, actions: &Actions) -> Option<()>{}
///
/// 以上两个方法到ref和self中进行调用
pub struct EventLzVisitor;

impl EventLzVisitor {
    /// 处理事件枚举
    /// 返回：HashMap<事件名，事件类型> （可以从事件枚举的定义中得到）
    pub fn visit(
        event: &mut ItemEnum,
        impls: &mut Impls,
    ) -> Result<Option<HashMap<String, String>>, Error> {
        Self::handle_event_enum(event);

        let (mut self_fns, ref_self_fns, events) = event.variants.iter().fold(
            (Vec::new(), Vec::new(), HashMap::new()),
            |(mut fns, mut ref_fns, mut events), var| {
                if var.ident.to_token_stream().to_string() != "None" {
                    let (self_fn, self_ref_fn) = Self::event_fn(&event.ident, var);
                    fns.push(self_fn);
                    ref_fns.push(self_ref_fn);

                    events.insert(
                        var.ident.to_string(),
                        var.fields.to_token_stream().to_string(),
                    );
                }

                (fns, ref_fns, events)
            },
        );

        self_fns.push(Self::default_callback_fn());

        impls.self_impl.extend(self_fns);
        impls.self_ref_impl.extend(ref_self_fns);

        if events.is_empty() {
            return Ok(None);
        } else {
            Ok(Some(events))
        }
    }

    /// 处理事件枚举
    fn handle_event_enum(item_enum: &mut ItemEnum) -> () {
        // [remove #[event] attr] --------------------------------------------------------------------------------
        item_enum
            .attrs
            .retain(|attr| !attr.path().is_ident("event"));
        // [add #[derive(DefaultNone)] attr] ----------------------------------------------------------------------
        item_enum.attrs.push(parse_quote!(#[derive(DefaultNone)]));
        // [add None as variant] ---------------------------------------------------------------------------------
        item_enum.variants.push(parse_quote! {None});
    }

    fn event_fn(ident: &Ident, var: &Variant) -> (ImplItem, Stmt) {
        let fn_name = &var.ident;
        let snake_fn_name =
            parse_str::<TokenStream>(&camel_to_snake(&fn_name.to_string())).unwrap();
        // all the fields are unnamed
        let (field, fback) = match &var.fields {
            Fields::Named(_) => panic!("can not be here! all the fields are unnamed"),
            Fields::Unnamed(fields_unnamed) => {
                let p = fields_unnamed.unnamed.first().unwrap().to_token_stream();
                (quote! {(#p)}, quote! {#p})
            }
            Fields::Unit => (TokenStream::new(), quote! {()}),
        };

        let self_fn = parse_quote! {
            fn #snake_fn_name(&self, actions: &Actions) -> Option<#fback> {
                if !self.event_key {
                    return None;
                }

                if let #ident::#fn_name #field = actions.find_widget_action(self.widget_uid()).cast() {
                    Some(#fback)
                } else {
                    None
                }
            }
        };

        let self_ref_fn = parse_quote! {
            pub fn #snake_fn_name(&self, actions: &Actions) -> Option<#fback> {
                if let Some(c_ref) = self.borrow() {
                    return c_ref.#snake_fn_name(actions);
                }
                None
            }
        };

        (self_fn, self_ref_fn)
    }

    fn default_callback_fn() -> ImplItem {
        parse_quote! {
            fn active_event<F>(&mut self, cx: &mut Cx, f: F) -> ()
            where
                F: FnOnce(&mut Cx, WidgetUid, &HeapLiveIdPath) -> ()
            {
                if self.event_key{
                    self.scope_path.as_ref().map(|path| {
                        f(cx, self.widget_uid(), path);
                    });
                }
            }
        }
    }
}
