use gen_analyzer::Binds;
use gen_plugin::MacroContext;
use gen_utils::{
    common::string::FixedString,
    error::{CompilerError, Error},
};
use quote::ToTokens;
use ra_ap_syntax::{
    ast::{self, HasArgList, MethodCallExpr},
    AstNode, Edition, SourceFile, TextRange,
};
use std::collections::HashMap;
use syn::{parse_str, ImplItemFn};

use crate::compiler::{Context, WidgetPoll};

/// 访问双向绑定访问器结构体
#[allow(unused)]
#[derive(Debug)]
struct BindingReplacer {
    replacements: HashMap<TextRange, String>,
    fields: Vec<String>,
}

impl BindingReplacer {
    fn new(fields: Vec<String>) -> Self {
        Self {
            replacements: HashMap::new(),
            fields,
        }
    }

    fn add_replacement(&mut self, range: TextRange, new_text: String) {
        self.replacements.insert(range, new_text);
    }

    fn apply_replacements(&self, input: &str) -> String {
        let mut result = input.to_string();
        let mut offset = 0_i32;
        // 按照范围排序，确保替换的正确性
        let mut ranges: Vec<_> = self.replacements.iter().collect();
        ranges.sort_by_key(|(range, _)| range.start());

        for (range, new_text) in ranges {
            let range_start: u32 = range.start().into();
            let range_end: u32 = range.end().into();
            let start = (range_start as i32 + offset) as usize;
            let end = (range_end as i32 + offset) as usize;
            result.replace_range(start..end, new_text);

            offset += new_text.len() as i32 - (range_end - range_start) as i32;
        }

        result
    }
}

/// ## 访问fuction并进行替换
/// 以下内容需要进行处理:
/// 1. c_ref!宏 (转为self.#widget(id!(#id)))
/// 2. active!宏 (转为self.active_event(cx, |cx, uid, path| {cx.widget_action(uid, path, #param);}))
/// 3. get_和set_方法 (转为self.#field_name()和self.#field_name(#param))
/// 4. signal_fns中的方法 (在参数列表最后添加cx)
/// 5. 当方法中含有set_方法时, 最终需要增加一行重新绘制的代码 (self.redraw(cx);) 来触发重绘
///
/// is_special: 标记当前方法是否是特殊的访问器，例如生命周期就无需进行redraw
pub fn visit_fns(
    input: &mut ImplItemFn,
    fields: &Vec<String>,
    computeds: &Vec<String>,
    widgets: &WidgetPoll,
    prop_binds: Option<&Binds>,
    signal_fns: &Vec<String>,
    ctx: &Context,
    is_special: bool,
) -> Result<(), Error> {
    let processor = ctx.dyn_processor.as_ref();
    let router = ctx.router.as_ref();
    let input_str = input.to_token_stream().to_string();
    let source_file = SourceFile::parse(&input_str, Edition::Edition2021);
    let syntax = source_file.tree();
    // 记录需要检查并调用get|set的组件，当使用者调用c_ref!时需要将组件id记录到这里，然后在get|set访问时进行替换
    let mut addition_widgets = HashMap::new();
    // 记录是否需要增加重绘
    let mut redraw = false;
    // 创建替换器
    let mut replacer = BindingReplacer::new(fields.clone());
    // 记录方法中访问到的路由组件，路由组件需要在其nav_to和nav_back方法中添加cx参数
    let mut router_widget = None;
    // [visit_two_way_binding] -------------------------------------------------------------------------------
    // 遍历语法树
    for node in syntax.syntax().descendants() {
        // [c_ref!, active!] ---------------------------------------------------------------------------------------------------
        // c_ref宏一定是let语句中的MacroCall
        if let Some(let_stmt) = ast::LetStmt::cast(node.clone()) {
            for node in let_stmt.syntax().descendants() {
                if let Some(macro_call) = ast::MacroCall::cast(node) {
                    if let Some(path) = macro_call.path() {
                        let ident = path.syntax().text().to_string();
                        if ident == "c_ref" {
                            // [replace c_ref!() => self.#widget(id!(#id))] ------------------------------------------------------------
                            if let Some(tt) = macro_call.token_tree() {
                                // remove `{}` or `()`
                                let id = inner_tt(tt);
                                // 记录id
                                let let_ident = let_stmt
                                    .pat()
                                    .and_then(|pat| {
                                        ast::IdentPat::cast(pat.syntax().clone()).map(|ident_pat| {
                                            ident_pat
                                                .syntax()
                                                .last_token()
                                                .unwrap()
                                                .text()
                                                .to_string()
                                        })
                                    })
                                    .ok_or(CompilerError::runtime(
                                        "Makepad Compiler - Script",
                                        "c_ref! macro should has let statement",
                                    ))?;
                                // 这里需要将id记录到addition_widgets中
                                addition_widgets.insert(let_ident.to_string(), id.to_string());

                                let widget = widgets.get(&id).map_or_else(
                                    || {
                                        Err(Error::from(CompilerError::runtime(
                                            "Makepad Compiler - Script",
                                            "can not find id in template, please check!",
                                        )))
                                    },
                                    |widget| Ok(widget.snake_name()),
                                )?;

                                let new_expr = format!("self.{}(id!({}))", &widget, id);
                                let full_range = macro_call.syntax().text_range();
                                replacer.add_replacement(full_range, new_expr);

                                // 尝试获取路由组件
                                if let Some(router) = router {
                                    // 这里需要根据组件名字的缩写来判断是否是路由组件
                                    if widget == router.name.camel_to_snake() {
                                        router_widget.replace(UsedRouter {
                                            id: id.to_string(),
                                            name: widget.to_string(),
                                            ident: let_ident.to_string(),
                                        });
                                    }
                                }
                            } else {
                                return Err(CompilerError::runtime(
                                    "Makepad Compiler - Script",
                                    "c_ref! macro should has widget id as token",
                                )
                                .into());
                            }
                        }
                    }
                }
            }
        }

        if let Some(macro_call) = ast::MacroCall::cast(node.clone()) {
            if let Some(path) = macro_call.path() {
                let ident = path.syntax().text().to_string();
                if ident == "active" {
                    // [replace active!() => self.active_event(cx, |cx, uid, path| {cx.widget_action(uid, path, #param);})] -------
                    if let Some(tt) = macro_call.token_tree() {
                        let param = inner_tt(tt);
                        let new_expr = format!(
                            "self.active_event(cx, |cx, uid, path| {{cx.widget_action(uid, path, {});}})",
                            param
                        );
                        let full_range = macro_call.syntax().text_range();
                        replacer.add_replacement(full_range, new_expr);
                    } else {
                        return Err(CompilerError::runtime(
                            "Makepad Compiler - Script",
                            "active! macro should has param as token",
                        )
                        .into());
                    }
                } else if ident == "nav_to" {
                    if let Some(tt) = macro_call.token_tree() {
                        let tt = inner_tt(tt);
                        if !tt.is_empty() {
                            // add cx, self.widget_uid(), &mut Scope::empty() as param
                            let new_expr = format!(
                                "nav_to!({}, cx, self.widget_uid(), &mut Scope::empty());",
                                tt
                            );
                            let full_range = macro_call.syntax().text_range();
                            replacer.add_replacement(full_range, new_expr);
                        } else {
                            return Err(CompilerError::runtime(
                                "Makepad Compiler - Script",
                                "nav_to! macro should has param, param is the id of the page you registered in router toml",
                            )
                            .into());
                        }
                    }
                } else if ident == "nav_back" {
                    if let Some(tt) = macro_call.token_tree() {
                        let tt = inner_tt(tt);
                        // nav_back should have no tt, so tt should be empty
                        if tt.is_empty() {
                            // add cx, self.widget_uid(), &mut Scope::empty() as param
                            let new_expr =
                                format!("nav_back!(cx, self.widget_uid(), &mut Scope::empty());");
                            let full_range = macro_call.syntax().text_range();
                            replacer.add_replacement(full_range, new_expr);
                        } else {
                            return Err(CompilerError::runtime(
                                "Makepad Compiler - Script",
                                "nav_back! macro should has no param",
                            )
                            .into());
                        }
                    }
                } else {
                    if let Some(processor) = processor {
                        let tokens = if let Some(tt) = macro_call.token_tree() {
                            inner_tt(tt)
                        } else {
                            String::new()
                        };
                        let mut mac_context = MacroContext { ident, tokens };

                        let is_replace = unsafe {
                            processor.process_macro(&mut mac_context).map_err(|e| {
                                CompilerError::runtime("Makepad Compiler - Script", &e.to_string())
                            })?
                        };

                        if is_replace {
                            let new_expr =
                                format!("{}!({})", mac_context.ident, mac_context.tokens);
                            let full_range = macro_call.syntax().text_range();
                            replacer.add_replacement(full_range, new_expr);
                        }
                    }
                }
            }
        }

        // get and set method call
        if let Some(method_call) = ast::MethodCallExpr::cast(node.clone()) {
            if let Some(receiver) = method_call.receiver() {
                let receiver_text = receiver.syntax().text().to_string();

                let from_widget = method_call
                    .syntax()
                    .first_child()
                    .and_then(|first| addition_widgets.get_key_value(&first.text().to_string()));

                // 检查是否是目标属性访问
                if receiver_text == "self" || from_widget.is_some() {
                    // dbg!(method_call.syntax().text());
                    if let Some(name_ref) = method_call.name_ref() {
                        let method_name = name_ref.syntax().text().to_string();
                        if method_name.starts_with("get_") || method_name.starts_with("set_") {
                            let field_name = method_name
                                .trim_start_matches("get_")
                                .trim_start_matches("set_")
                                .to_string();
                            // dbg!(&fields, &field_name);
                            let is_computed = computeds.contains(&field_name);
                            // 检查字段是否在目标列表中
                            if fields.contains(&field_name) || from_widget.is_some() || is_computed
                            {
                                let prefix = if let Some((w, _)) = from_widget {
                                    w.to_string()
                                } else {
                                    "self".to_string()
                                };

                                let is_setter = method_name.starts_with("set_");
                                // 获取完整的方法调用范围
                                let full_range = method_call.syntax().text_range();

                                // 构建新的调用表达式
                                let new_expr = if is_setter {
                                    let mut redraw_cref = None;
                                    // computed不需要重绘
                                    // if !is_computed {
                                    //     redraw = true;
                                    // }
                                    redraw = true && !is_special;

                                    let mut new_call = String::new();
                                    // 如果from_widget则需要反向绑定到父组件中完成双向绑定
                                    if let Some((widget_ident, widget_id)) = from_widget {
                                        // 获取function中的参数
                                        let param = method_call.arg_list().map_or_else(
                                            || Err(Error::from("set prop need a param!")),
                                            |arg_list| Ok(arg_list.syntax().text().to_string()),
                                        )?;
                                        // 通过field_name获取父组件中绑定的字段名
                                        // 没有找到的话可能是因为并没有采取双向绑定的方式，而是c_ref的直接内部访问，这里就不需要处理
                                        if let Some(prop_binds) = prop_binds {
                                            let _ = prop_binds
                                                .iter()
                                                .find(|(_, v)| {
                                                    v.iter().any(|widget| {
                                                        &widget.id == widget_id
                                                            && widget.prop.as_str() == field_name
                                                    })
                                                })
                                                .map(|(bind_field, _)| {
                                                    new_call.push_str(
                                                        format!(
                                                            "self.{} = {}.clone();",
                                                            bind_field,
                                                            remove_holder(&param)
                                                        )
                                                        .as_str(),
                                                    );
                                                });
                                        }
                                        // c_ref!调用后使用了set_方法则需要对这个组件进行重绘
                                        redraw_cref.replace(widget_ident);
                                    }

                                    // 对于setter，需要添加cx参数
                                    new_call.push_str(&format!("{}.", prefix));
                                    new_call.push_str(&method_name);

                                    // 检查是否已经有cx参数
                                    if let Some(arg_list) = method_call.arg_list() {
                                        let args = arg_list.syntax().text().to_string();
                                        if !args.contains("cx") {
                                            // 在参数列表开始位置插入cx
                                            let mut args = args.to_string();
                                            if args == "()" {
                                                args = "(cx)".to_string();
                                            } else {
                                                args.insert_str(1, "cx, ");
                                            }
                                            new_call.push_str(&args);
                                        } else {
                                            new_call.push_str(&args);
                                        }
                                    }

                                    if let Some(widget) = redraw_cref {
                                        new_call.push_str(&format!("; {}.redraw(cx);", widget));
                                    }

                                    new_call
                                } else {
                                    // 对于getter，直接替换接收者
                                    format!("{}.{}()", prefix, method_name)
                                };

                                replacer.add_replacement(full_range, new_expr);
                            }
                        } else {
                            // 检查是否在signal_fns中
                            if signal_fns.contains(&method_name) {
                                // 这里只需要为方法调用的参数中最后一个参数添加cx即可
                                if let Some(arg_list) = method_call.arg_list() {
                                    let args = arg_list.syntax().text().to_string();
                                    if !args.contains("cx") {
                                        // 在参数列表最后添加cx
                                        let mut args = args.to_string();
                                        if args == "()" {
                                            args = "(cx)".to_string();
                                        } else {
                                            args.insert(args.len() - 1, ',');
                                            args.push_str("cx");
                                        }
                                        let full_range = method_call.syntax().text_range();
                                        let new_expr =
                                            format!("{}.{}{}", receiver_text, method_name, args);
                                        replacer.add_replacement(full_range, new_expr);
                                    }
                                }
                            } else {
                                if let Some(router_widget) = router_widget.as_ref() {
                                    if router_widget.ident == receiver_text {
                                        // 这里说明当前方法调用了路由组件的nav_to或nav_back方法，需要添加cx参数
                                        match RouterCalled::from(method_name.as_str()) {
                                            RouterCalled::NavTo => {
                                                let args =
                                                    handle_router_args(&method_call, 1, "nav_to")?;
                                                let full_range = method_call.syntax().text_range();
                                                let new_expr = format!(
                                                    "{}.{}({});",
                                                    receiver_text,
                                                    method_name,
                                                    args.join(",")
                                                );
                                                replacer.add_replacement(full_range, new_expr);
                                            }
                                            RouterCalled::NavBack => {
                                                let args = handle_router_args(
                                                    &method_call,
                                                    0,
                                                    "nav_back",
                                                )?;
                                                let full_range = method_call.syntax().text_range();
                                                let new_expr = format!(
                                                    "{}.{}({});",
                                                    receiver_text,
                                                    method_name,
                                                    args.join(",")
                                                );
                                                replacer.add_replacement(full_range, new_expr);
                                            }
                                            RouterCalled::Unknown => {}
                                        }

                                        // let new_expr =format!("{}.{}")
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 应用所有替换
    let modified_code = replacer.apply_replacements(&input_str);

    // 解析回ImplItemFn
    match parse_str::<ImplItemFn>(&modified_code) {
        Ok(mut new_fn) => {
            // 如果有需要重绘的情况，需要在最后添加self.redraw(cx);
            if redraw {
                new_fn
                    .block
                    .stmts
                    .push(parse_str("self.redraw(cx);").unwrap());
            }

            *input = new_fn;
            Ok(())
        }
        Err(e) => Err(Error::from(format!("Failed to parse modified code: {}", e))),
    }
    // [visit c_ref!]
}

fn inner_tt(tt: ast::TokenTree) -> String {
    let param = tt.syntax().text().to_string();
    remove_holder(&param).to_string()
}

/// 去除花括号和括号, 只去除一层
fn remove_holder(input: &str) -> &str {
    if (input.starts_with('(') && input.ends_with(')'))
        || (input.starts_with('{') && input.ends_with('}'))
        || (input.starts_with('[') && input.ends_with(']'))
    {
        &input[1..input.len() - 1]
    } else {
        input
    }
}

fn handle_router_args(
    method_call: &MethodCallExpr,
    arg_num: usize,
    method_name: &str,
) -> Result<Vec<String>, Error> {
    method_call.arg_list().map_or_else(
        || {
            Err(Error::from(format!(
                "can not find args in {}()",
                method_name
            )))
        },
        |arg_list| {
            // 这里还需要继续判断，args的数量
            if arg_list.args().count() == arg_num {
                // 在参数列表末尾添加cx
                let mut args = arg_list.args().fold(Vec::new(), |mut acc, arg| {
                    acc.push(arg.syntax().text().to_string());
                    acc
                });
                args.push("cx".to_string());
                Ok(args)
            } else {
                Err(Error::from(format!(
                    "{}(), should has only {} arg",
                    method_name, arg_num
                )))
            }
        },
    )
}

/// 用于存储路由被调用的结构体
/// 用于在访问中替换nav_to和nav_back方法
#[allow(unused)]
#[derive(Debug, Clone)]
struct UsedRouter {
    pub id: String,
    pub name: String,
    pub ident: String,
}

enum RouterCalled {
    NavTo,
    NavBack,
    Unknown,
}

impl From<&str> for RouterCalled {
    fn from(value: &str) -> Self {
        match value {
            "nav_to" => RouterCalled::NavTo,
            "nav_back" => RouterCalled::NavBack,
            _ => RouterCalled::Unknown,
        }
    }
}
