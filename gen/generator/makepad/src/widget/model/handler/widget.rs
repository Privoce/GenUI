use core::panic;

use gen_converter::model::script::{ScriptModel, UseMod};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{parse_quote, Attribute, Ident, ItemEnum, ItemStruct, Meta};

use crate::widget::BuiltIn;

pub struct WidgetHandler;

impl WidgetHandler {
    /// ## Build Widget Struct Name
    /// 1. A: 没有使用`gen_macros::Prop`标注的且没有声明id
    /// 2. B: 没有使用`gen_macros::Prop`标注但声明id
    /// 3. C: 使用`gen_macros::Prop`不声明id
    ///
    /// > Note!:
    /// >   1. 同时使用`gen_macros::Prop`且声明id时，若`${prop_struct_name} == ${id}`，属于C类
    /// >   2. 但若`${prop_struct_name} != ${id}`时，panic!
    /// ### A
    /// `${source_name}_${inherits}`
    /// ### B
    /// `${id}`
    /// ### C
    /// `${prop_struct_name}`
    /// ### Return
    /// (widget_struct_name, builtin_inherits, is_builtin_widget, is_static)
    pub fn build_widget_struct_name(
        source_name: &str,
        widget_name: &str,
        id: Option<&String>,
        is_component: bool,
        inherits: Option<&String>,
        is_root: bool,
        script: Option<&ScriptModel>,
    ) -> (String, Option<BuiltIn>, bool, bool) {
       
        // only root widget can define as a component
        if is_component && is_root {
            let inherits =
                BuiltIn::try_from(inherits.unwrap_or(&BuiltIn::default().to_string())).unwrap();
            let name = match (id, script) {
                (None, None) => format!("{}_{}", source_name, inherits.to_string()), // type A
                (None, Some(ScriptModel::Gen(sc))) => {
                    // sc only can be Gen
                    if let Some(prop_struct) = &sc.prop_ptr {
                        prop_struct.ident.to_string()
                    } else {
                        panic!("Component must have a prop struct if is type C")
                    }
                } // type C
                (Some(id), None) => id.to_string(),                                  // type B
                (Some(id), Some(ScriptModel::Gen(sc))) => {
                    if let Some(prop_struct) = &sc.prop_ptr {
                        if prop_struct.ident.to_string().eq(id) {
                            id.to_string()
                        } else {
                            panic!("Component id must be same as prop struct name if is type C")
                        }
                    } else {
                        panic!("Component must have a prop struct if is type C")
                    }
                } // type C
                _ => panic!("Can not be handled See Dyn Widget Rule"),
            };

            return (name, Some(inherits), false, script.is_none());
        } else {
            // here means current widget is not root widget, directly return widget name
            // and the builtin inherits is self
            return (
                widget_name.to_string(),
                BuiltIn::try_from(widget_name).ok(),
                true,
                script.is_none(),
            );
        }
    }

    pub fn uses(uses: &UseMod) -> Option<TokenStream> {
        let mut tk = TokenStream::new();
        let UseMod { widget, other, .. } = uses;

        if let Some(widget) = widget {
            for item in widget.iter() {
                tk.extend(item.to_token_stream());
            }
        }

        if let Some(other) = other {
            for item in other.iter() {
                tk.extend(item.to_token_stream());
            }
        }

        if tk.is_empty() {
            None
        } else {
            Some(tk)
        }
    }
    /// convert GenUI property struct to Makepad property struct
    pub fn prop_ptr(prop_ptr: &ItemStruct, inherit: &BuiltIn) -> TokenStream {
        // 将GenUI的结构体转为Makepad的属性结构体
        inherit.to_token_stream(prop_ptr)
    }
    /// convert GenUI event enum to Makepad event enum
    pub fn event_ptr(event_ptr: &ItemEnum) -> TokenStream {
        // 将GenUI的结构体转为Makepad的事件枚举
        // 将GenUI标记的#[derive(Event)]修改为Makepad的#[derive(DefaultNone)]
        let mut new_item = event_ptr.clone();
        for attr in new_item.attrs.iter_mut() {
            if let Meta::List(meta) = &mut attr.meta {
                if meta.path.is_ident("derive") && meta.tokens.to_string().contains("Event") {
                    // 将Event修改为DefaultNone，其他不变
                    let mut new_tk = TokenStream::new();
                    let _ = meta.tokens.clone().into_iter().for_each(|token| {
                        if let TokenTree::Ident(ident) = token {
                            let new_ident = if ident.to_string() == "Event" {
                                Ident::new("DefaultNone", Span::call_site())
                            } else {
                                ident
                            };
                            new_tk.extend(vec![TokenTree::Ident(new_ident)]);
                        } else {
                            new_tk.append(token);
                        }
                    });

                    meta.tokens = new_tk;

                    // 将修改后的Meta赋值回Attribute
                    *attr = Attribute {
                        meta: Meta::List(meta.clone()),
                        ..attr.clone()
                    }
                }
            }
        }
        // 检查是否有Event::None，没有则添加
        if !(new_item
            .variants
            .iter()
            .any(|var| var.ident.to_string().eq("None")))
        {
            new_item.variants.push(parse_quote! { None });
        }

        new_item.to_token_stream()
    }
}
