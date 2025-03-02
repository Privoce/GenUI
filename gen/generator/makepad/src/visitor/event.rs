use gen_utils::{common::camel_to_snake, error::Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, parse_str, Fields, Ident, ItemEnum, Stmt, Variant};

use crate::script::Impls;

/// 根据提供的事件生成对应外部调用方法, 例如`enum A { B, C , None }`生成:
/// 
/// - (pub) fn b(&self, actions: &Actions) -> Option<()>{}
/// - (pub) fn c(&self, actions: &Actions) -> Option<()>{}
/// 
/// 以上两个方法到ref和self中进行调用
pub struct EventLzVisitor;

impl EventLzVisitor {
    pub fn visit(events: &Vec<ItemEnum>, impls: &mut Impls) -> Result<(), Error> {
        let (mut self_fns, ref_self_fns) =
            events
                .iter()
                .fold((Vec::new(), Vec::new()), |(mut fns, mut ref_fns), event| {
                    let (f, ref_f) = event.variants.iter().fold(
                        (Vec::new(), Vec::new()),
                        |(mut fns, mut ref_fns), var| {
                            if var.ident.to_token_stream().to_string() != "None" {
                                let (self_fn, self_ref_fn) = Self::event_fn(&event.ident, var);
                                fns.push(self_fn);
                                ref_fns.push(self_ref_fn);
                            }

                            (fns, ref_fns)
                        },
                    );

                    fns.extend(f);
                    ref_fns.extend(ref_f);
                    (fns, ref_fns)
                });

        self_fns.push(Self::default_callback_fn());

        impls.self_impl.extend(self_fns);
        impls.self_ref_impl.extend(ref_self_fns);

        Ok(())
    }

    fn event_fn(ident: &Ident, var: &Variant) -> (Stmt, Stmt) {
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

    fn default_callback_fn() -> Stmt {
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
