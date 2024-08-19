use gen_converter::model::{
    script::{PropFn, PropFnOnly},
    PropTree,
};
use gen_parser::{PropsKey, Value};
use gen_utils::{
    common::{ident, string::FixedString, syn_ext::ClosureConverter, token_stream_to_tree},
    error::Errors,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, parse_str, visit_mut::VisitMut, Attribute, Ident, ItemStruct, Meta, Pat, Stmt,
};

use crate::{prop::builtin::MakepadValue, utils::apply_over_and_redraw};

use super::{
    model::{role::RoleType, widget::Replacer},
    BuiltIn,
};

pub fn vec_string_to_string(vec: &Vec<String>) -> String {
    format!(
        "[{}]",
        vec.iter()
            .map(|item| format!("\"{}\"", item))
            .collect::<Vec<String>>()
            .join(",")
    )
}

pub fn fn_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&str, Option<&Vec<String>>) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        f(s, None);
        Ok(())
    } else {
        value
            .is_fn_and_get()
            .map(|s| {
                f(s.get_name(), s.get_params().as_ref());
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to fn",
                    value
                )))
            })
    }
}

pub fn bool_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(bool) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<bool>() {
            Ok(b) => {
                f(b);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else {
        value
            .is_bool_and_get()
            .map(|b| {
                f(b);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn u64_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(u64) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<u64>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to u64",
                s
            ))),
        }
    } else if let Some(d) = value.is_u_int_and_get() {
        f(d as u64);
        Ok(())
    } else {
        value
            .is_i_int_and_get()
            .map(|int| {
                f(int as u64);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to u64",
                    value
                )))
            })
    }
}

pub fn usize_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(usize) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<usize>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to usize",
                s
            ))),
        }
    } else if let Some(d) = value.is_u_int_and_get() {
        f(d);
        Ok(())
    } else {
        value
            .is_i_int_and_get()
            .map(|int| {
                f(int as usize);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to usize",
                    value
                )))
            })
    }
}

pub fn i64_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(i64) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<i64>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to i64",
                s
            ))),
        }
    } else {
        value
            .is_int_and_get()
            .map(|int| {
                f(int);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to i64",
                    value
                )))
            })
    }
}

pub fn f64_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(f64) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f64>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to f64",
                s
            ))),
        }
    } else if let Some(d) = value.is_double_and_get() {
        f(d);
        Ok(())
    } else if let Some(d) = value.is_float_and_get() {
        f(d as f64);
        Ok(())
    } else {
        value
            .is_bool_and_get()
            .map(|b| {
                f(b as u8 as f64);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to f64",
                    value
                )))
            })
    }
}

pub fn f32_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(f32) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f32>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to f32",
                s
            ))),
        }
    } else if let Some(b) = value.is_float_and_get() {
        f(b);
        Ok(())
    } else if let Some(b) = value.is_double_and_get() {
        f(b as f32);
        Ok(())
    } else {
        value
            .is_bool_and_get()
            .map(|b| {
                f(b as u8 as f32);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to f32",
                    value
                )))
            })
    }
}

pub fn string_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&str) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        f(s);
        Ok(())
    } else {
        value
            .is_string_and_get()
            .map(|s| {
                f(s);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to str",
                    value
                )))
            })
    }
}

pub fn bind_prop_value(value: &Value, is_prop: bool, ident: &str) -> String {
    return if let Some(v) = value.is_bind_and_get().unwrap().get_normal() {
        if is_prop && ident.starts_with(ident) {
            // 说明这个绑定的属性是从外部传入的，需要将定义的首个prefix转为self
            v.replacen(ident, "self", 1)
        } else {
            v.to_string()
        }
    } else {
        panic!("prop value is not bind: {}", value)
    };
}

pub fn quote_prop(keys: Vec<&str>, value: &str) -> TokenStream {
    let mut result = String::new();

    // 迭代keys，逐一构建字符串
    for &key in keys.iter() {
        if !result.is_empty() {
            result.push_str(": {");
        }
        result.push_str(key);
    }

    // 添加最内层的值
    result.push_str(" : (");
    result.push_str(value);
    result.push_str("),");

    // 根据keys的数量，添加相应数量的闭括号
    for _ in 0..keys.len() - 1 {
        result.push_str("},");
    }

    result.parse().unwrap()
}

/// 将GenUI的结构体转为Makepad的属性结构体
pub fn quote_makepad_widget_struct(value: &ItemStruct) -> ItemStruct {
    let mut new_item = value.clone();

    // 遍历属性并修改,将Prop修改为Live, Widget
    // LiveHook这个trait则使用impl trait的方式实现
    for attr in new_item.attrs.iter_mut() {
        if let Meta::List(meta) = &mut attr.meta {
            if meta.path.is_ident("derive") && meta.tokens.to_string().contains("Prop") {
                // 使用parse_quote! 宏来创建新的tokens
                meta.tokens = parse_quote! { Live, Widget };
                // 将修改后的Meta赋值回Attribute
                *attr = Attribute {
                    meta: Meta::List(meta.clone()),
                    ..attr.clone()
                }
            }
        }
    }
    // 对结构体中的字段进行处理，符合的进行宏标记
    for field in new_item.fields.iter_mut() {
        let ident = field.ty.to_token_stream().to_string();
        match MakepadValue::from(&ident) {
            MakepadValue::Live(_) => {
                field.attrs.push(parse_quote! { #[live] });
            }
            MakepadValue::Rust => {
                field.attrs.push(parse_quote! { #[rust] });
            }
            _ => panic!("prop ptr field not support to convert to MakepadValue"),
        }
    }
    new_item
}

/// 根据widget的绘制函数生成对应的代码
/// 生成对应widget的绘制函数中的代码
/// 这部分很统一，所有的widget都是这样处理的(除了自动生成的IFWidget和ForWidget)
pub fn quote_draw_widget(
    draw_widget: &Option<Vec<PropFn>>,
    replacer: Option<&Replacer>,
) -> Option<TokenStream> {
    let tk = if let Some(draw_widget_tk) = draw_widget {
        let mut tk = TokenStream::new();
        for item in draw_widget_tk {
            let PropFn {
                widget,
                id,
                key,
                ident,
                code,
                is_prop,
            } = item;
            // from widget get prop value
            if key.is_builtin() {
                // here is_builtin just is gen builtin not makepad builtin
                let _ = bind_widget_prop_value(id, key, &ident.to_string(), Some(code), replacer)
                    .map(|x| tk.extend(x));
            } else {
                // 当前只考虑builtin，自定义类型组件后续增加
                let builtin = BuiltIn::from(&widget);
                let pv = builtin.prop_bind(key, ident, *is_prop, &local_ident(code));
                if !is_prop {
                    tk.extend(code.to_token_stream());
                }
                tk.extend(apply_over_and_redraw(
                    None,
                    widget,
                    id,
                    token_stream_to_tree(pv),
                ));
            }
        }
        Some(tk)
    } else {
        None
    };
    tk
}

pub trait QuoteDraw {
    /// ## draw prop depend on PropFnOnly or PropFn
    /// this fn will generate:
    /// 1. not special: `self.$widget_name(id!($widget_id)).apply_over_and_redraw(cx, live!{$prop_name: ($prop_value_ident)});`
    /// 2. is special: `self.$widget_name(id!($widget_id)).set_$set_fn($prop_name);`
    fn draw_prop(&self, replacer: Option<&Replacer>) -> Option<TokenStream>;
}

impl QuoteDraw for PropFn {
    fn draw_prop(&self, replacer: Option<&Replacer>) -> Option<TokenStream> {
        PropFnOnly::from(self).draw_prop(replacer)
    }
}

impl QuoteDraw for PropFnOnly {
    fn draw_prop(&self, replacer: Option<&Replacer>) -> Option<TokenStream> {
        let PropFnOnly {
            widget,
            id,
            key,
            ident,
        } = self;
        let mut ident = ident
            .is_bind_and_get()
            .unwrap()
            .to_string()
            .split_fixed(".");
        // change the first part to self
        ident[0] = "self".to_string();

        return if key.is_builtin() {
            // if widget/ for widget will into here
            bind_widget_prop_value(id, key, &ident.join("."), None, replacer)
        } else {
            // normal widget will into here
            let widget = parse_str::<TokenStream>(widget).unwrap();
            let id = parse_str::<TokenStream>(id).unwrap();
            let name = parse_str::<TokenStream>(key.name()).unwrap();
            let ident = parse_str::<TokenStream>(&ident.join(".")).unwrap();
            Some(quote! {
                self.#widget(id!(#id)).apply_over_and_redraw(cx, live!{ #name: (#ident) });
            })
        };
    }
}

pub fn bind_widget_prop_value(
    id: &str,
    key: &PropsKey,
    ident: &str,
    code: Option<&Stmt>,
    replacer: Option<&Replacer>,
) -> Option<TokenStream> {
    fn is_if(key: &PropsKey) -> bool {
        (key.is_bind() || key.is_fn()) && key.name() == "if"
    }

    return if is_if(key) {
        // replacer must be Some
        let replacer = replacer.unwrap().iter().find_map(|((_, r_id), ulid)| {
            if r_id == id {
                Some(ulid.to_snake())
            } else {
                None
            }
        });

        let widget_name = parse_str::<TokenStream>(&format!(
            "if_widget{}",
            replacer.expect("replacer must be Some!")
        ))
        .unwrap();
        let id = parse_str::<TokenStream>(id).unwrap();
        let func =
            parse_str::<TokenStream>(&format!("set_{}_signal({});", key.name(), ident)).unwrap();
        Some(quote! {
            #code
            self.#widget_name(id!(#id)).#func
        })
    } else {
        None
    };
}

// pub fn quote_draw_widget_define(draw_widget: &Option<Vec<PropFn>>,code: TokenStream)->Option<TokenStream>{
//     let tk = if let Some(draw_widget_tk) = draw_widget {

//     }else{
//         None
//     };
//     tk
// }

/// get local ident from stmt
fn local_ident(code: &Stmt) -> String {
    fn get(pat: &Pat) -> String {
        match pat {
            Pat::Ident(ident) => ident.ident.to_string(),
            Pat::Type(ty) => get(&*ty.pat),
            _ => panic!("local stmt must be ident|type"),
        }
    }

    if let Stmt::Local(local) = code {
        // dbg!(get(&local.pat));
        get(&local.pat)
    } else {
        panic!("local stmt must be ident|type")
    }
}

/// 根据widget的事件处理函数生成对应的代码
/// 生成出对应widget的事件处理函数
/// event: 事件处理函数
/// props: 属性绑定（通过使用这个，能够在事件中找到需要更改为self的部分）
/// instance_name: 实例名称
pub fn quote_handle_event(
    event: &Option<Vec<PropFn>>,
    binds: &PropTree,
    instance_name: Option<&Ident>,
    prop_fields: Option<&Vec<Ident>>,
) -> TokenStream {
    let target = Some(ident("deref_widget"));
    let work_tk = if let Some(event_tk) = event {
        let mut work_tk = TokenStream::new();
        for item in event_tk {
            let code = &item.code;
            //----------------------------------[work_tk]---------------------------------------
            let _ = code
                .prop_to_self_binding(instance_name, prop_fields, |props| {
                    // find item in binds depend on props and draw
                    if props.is_empty() || binds.is_empty() {
                        None
                    } else {
                        // dbg!(&props, &binds);
                        Some(props.iter().fold(
                            TokenStream::new(),
                            |mut acc, (prop, after_prop)| {
                                // dbg!(&binds);
                                let _ = binds.iter().for_each(|((w_name, w_id), tree)| {
                                    if let Some(tree) = tree {
                                        let _ = tree.iter().for_each(|(key, value)| {
                                            let _ =
                                                value.is_bind_and_get().unwrap().get_normal().map(
                                                    |v| {
                                                        if prop == v {
                                                            let _ = apply_redraw_or_set_bind(
                                                                w_name,
                                                                w_id,
                                                                key.name(),
                                                                after_prop,
                                                            )
                                                            .map(|tk| {
                                                                acc.extend(tk);
                                                            });
                                                        }
                                                    },
                                                );
                                        });
                                    }
                                });
                                acc
                            },
                        ))
                    }
                })
                .map(|code| {
                    work_tk.extend(code);
                });
        }
        Some(work_tk)
    } else {
        None
    };

    let target_handle_tk = match target {
        Some(t) => Some(quote! {self.#t.handle_event(cx, event, scope);}),
        None => None,
    };

    quote! {
        let uid = self.widget_uid();
        if let Event::Actions(actions) = event{
            #work_tk
        }
        #target_handle_tk
    }
}

/// ## generate apply over and redraw or set prop fn
/// if the widget is not virtual widget(if_widget|for_widget), do set
/// if is normal, do apply_over_and_redraw
pub fn apply_redraw_or_set_bind(
    widget_name: &str,
    widget_id: &str,
    key: &str,
    value: &str,
) -> Option<TokenStream> {
    // is virtual?
    let role = RoleType::from(key);
    let widget = parse_str::<TokenStream>(widget_name).unwrap();
    let widget_id = parse_str::<TokenStream>(widget_id).unwrap();
    if role.is_virtual() {
        if widget_name.has_ulid(role.to_prefix_snake().unwrap()) {
            let func =
                parse_str::<TokenStream>(&format!("set_{}_signal({});", key, value)).unwrap();

            Some(quote! {
                self.#widget(id!(#widget_id)).#func
            })
        } else {
            None
        }
    } else {
        let key = parse_str::<TokenStream>(key).unwrap();
        let value = parse_str::<TokenStream>(value).unwrap();
        Some(quote! {
            self.#widget(id!(#widget_id)).apply_over_and_redraw(cx, live!{ #key: (#value) });
        })
    }
}

pub fn active_macro_to_cx_widget_action(code: &mut Stmt) -> TokenStream {
    struct MacroModifier;
    impl VisitMut for MacroModifier {
        fn visit_expr_block_mut(&mut self, i: &mut syn::ExprBlock) {
            for stmt in i.block.stmts.iter_mut() {
                if let Stmt::Macro(macro_stmt) = stmt {
                    if macro_stmt.mac.path.is_ident("active") {
                        let tk = &macro_stmt.mac.tokens;
                        *stmt = parse_quote! {
                            cx.widget_action(uid, &scope.path, #tk);
                        };
                    }
                }
            }
        }
    }

    MacroModifier.visit_stmt_mut(code);
    code.to_token_stream()
}
/// 将import!宏中的TokenStream转化为live_register
/// - 例如: `crate::views::header::header::*;`
/// - 转化为: `crate::views::header::header::live_design(cx);`
/// - 例如: `crate::views::header::header::HeaderExample;`
/// - 转化为: `crate::views::header::header::live_design(cx);`
/// convert widget imports to app main live registers
pub fn imports_to_live_registers(imports: Option<TokenStream>) -> Option<Vec<String>> {
    // 找到最后一个::的位置将后面的字符替换为`live_design(cx);`
    if let Some(imports) = imports.as_ref() {
        // 由于TokenStream中内容无法直接分割为Vec<_>,所以这里需要先通过`;`进行分割，变成多个Vec<String>
        let imports = imports.to_string();
        let imports = imports
            .split(";")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        // 通过`;`分割后的Vec<String>，再将每个通过`::`进行分割，变成多个Vec<String>,最后将每个Vec<String>的最后一个元素替换为`live_design(cx);`
        let tk = imports.iter().fold(Vec::new(), |mut acc, item| {
            let mut item = item.split("::").collect::<Vec<&str>>();
            item.last_mut().map(|last| *last = "live_design(cx);");
            let item = item.join("::");
            acc.push(item);
            acc
        });
        Some(tk)
    } else {
        None
    }
}

/// combine two `Option<TokenStream>`
pub fn combine_option(l: Option<TokenStream>, r: Option<TokenStream>) -> Option<TokenStream> {
    match (l, r) {
        (Some(l_tk), Some(r_tk)) => {
            let mut tk = TokenStream::new();
            tk.extend(l_tk);
            tk.extend(r_tk);
            Some(tk)
        }
        (Some(l_tk), None) => Some(l_tk),
        (None, Some(r_tk)) => Some(r_tk),
        (None, None) => None,
    }
}

#[macro_export]
macro_rules! from_struct_to_ptr {
    ($ptr: ty, $field_ty: expr) => {
        impl From<&ItemStruct> for $ptr {
            fn from(value: &ItemStruct) -> Self {
                // 将GenUI的结构体转为Makepad的属性结构体
                let mut new_item = quote_makepad_widget_struct(value);
                // 设置#[deref]给当前的属性结构体
                if let Fields::Named(fields) = &mut new_item.fields {
                    // add view
                    fields
                        .named
                        .push(struct_field(vec!["deref"], "deref_widget", $field_ty));
                }
                Self(new_item)
            }
        }
    };
}

#[macro_export]
macro_rules! ptr_to_token {
    ($ptr: ty) => {
        impl ToToken for $ptr {
            fn to_token_stream(&self) -> TokenStream {
                self.0.to_token_stream()
            }
        }
    };
}

/// only can use for builtin prop see widget mod
#[macro_export]
macro_rules! props_to_token {
    ($ptr: ty) => {
        impl ToToken for $ptr {
            fn to_token_stream(&self) -> proc_macro2::TokenStream {
                self.to_string().parse::<TokenStream>().unwrap()
            }
        }
    };
}

#[cfg(test)]
mod test_utils {
    #[test]
    fn test_quote_prop() {
        let keys = vec!["a", "b", "c"];
        let value = "1";
        let result = super::quote_prop(keys, value);
        assert_eq!(result.to_string(), "a : { b : { c : (1) , } , } ,");
    }
}
