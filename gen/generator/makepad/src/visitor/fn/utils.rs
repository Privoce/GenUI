use gen_dyn_run::DynProcessor;
use gen_plugin::MacroContext;
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use syn::{parse_quote, parse_str, Expr, ExprMethodCall};

use crate::two_way_binding::GetSet;

use super::FnVisitorImpl;

/// 检查追加prop的get|set方法
pub fn is_prop_method_and_append<V>(
    visitor: &mut V,
    method: &mut ExprMethodCall,
) -> Result<bool, Error>
where
    V: FnVisitorImpl,
{
    fn is_method_and_mut(
        is_mut: bool,
        get_set: GetSet,
        method: &str,
        fields: &Vec<String>,
    ) -> Result<bool, Error> {
        if !is_mut && get_set.is_set() {
            // can not call set_xxx
            return Err(CompilerError::runtime(
                "Makepad Compiler - Script",
                "can not call set_xxx in immutable instance!",
            )
            .into());
        }
        // check if method_without_prefix is in fields
        let method_without_prefix = match get_set {
            GetSet::Get => method.trim_start_matches("get_"),
            GetSet::Set => method.trim_start_matches("set_"),
            GetSet::UnMatch => method,
        }
        .to_string();

        Ok(fields.contains(&method_without_prefix))
    }
    let method_name = method.method.to_string();
    let method_prefix = GetSet::from(method_name.as_str());
    visitor.set_mut(method_prefix.is_set());
    if method_prefix.is_unmatch() {
        return Ok(false);
    }
    if let Expr::Path(path) = &*method.receiver {
        let ident = path.path.segments[0].ident.to_string();
        // instance不存在或者fields不存在，也可能是直接使用了内置组件的方法（c_ref）
        if let Some(instance) = visitor.instance() {
            if let Some(fields) = instance.fields.as_ref() {
                if ident == instance.ident {
                    if is_method_and_mut(instance.is_mut, method_prefix, &method_name, fields)? {
                        // // 替换
                        // method.receiver = parse_quote! {self};
                        // // 如果是set方法需要在方法的参数中增加一个cx参数
                        // if method_prefix.is_set() {
                        //     method.args.insert(0, parse_quote! {cx});
                        // }
                        return Ok(true);
                    }
                }
            }
        }

        // #[deprecated]
        // // 这里需要处理代码中调用其他内置组件或自定义组件的方法
        // // 这里只需要检查self.input中的widget_poll中是否存在这个ident即可
        // if visitor.has_widget(&ident) && method_prefix.is_set() {
        //     // if method_name == "set_text" {
        //     //     method.method = gen_utils::common::ident("set_text_and_redraw");
        //     //     method.args[0] = Expr::Reference(ExprReference {
        //     //         attrs: vec![],
        //     //         and_token: And::default(),
        //     //         mutability: None,
        //     //         expr: method.args[0].clone().into(),
        //     //     });
        //     // }
        //     // 如果是set方法需要在方法的参数中增加一个cx参数
        //     method.args.insert(0, parse_quote! {cx});
        // }
    }
    // else if let Expr::MethodCall(method_call) = &*method.receiver {

    // }

    Ok(false)
}

pub fn visit_local_init_mut<V>(
    visitor: &mut V,
    i: &mut syn::LocalInit,
    processor: Option<&DynProcessor>,
) -> Result<bool, Error>
where
    V: FnVisitorImpl,
{
    match &mut *i.expr {
        Expr::Macro(expr_macro) => {
            if expr_macro.mac.path.is_ident("c_ref") {
                // let widget_name = parse_str::<TokenStream>(&self.widget.name).unwrap();
                // let widget_id = expr_macro.mac.tokens.clone();
                let widget_id = expr_macro.mac.tokens.to_string();
                // 通过widget_id从widgets中获取widget的name进行组合
                let widget_name =
                    parse_str::<TokenStream>(&visitor.get_widget_name(&widget_id)?).unwrap();
                let widget_id = parse_str::<TokenStream>(&widget_id).unwrap();

                i.expr = parse_quote! {self.#widget_name(id!(#widget_id))};
                return Ok(false);
            } else if expr_macro.mac.path.is_ident("active") {
                let param = &expr_macro.mac.tokens;

                i.expr = parse_quote! {
                    self.active_event(cx, |cx, uid, path| {
                        cx.widget_action(
                            uid,
                            path,
                            #param,
                        );
                    })
                };
                return Ok(false);
            } else {
                // do visit mac mut
                return visit_mac_mut(&mut expr_macro.mac, processor);
            }
        }
        _ => {}
    }
    Ok(false)
}

pub fn visit_mac_mut(i: &mut syn::Macro, processor: Option<&DynProcessor>) -> Result<bool, Error> {
    if let Some(processor) = processor {
        let mut mac_context = MacroContext::from(i.clone());

        unsafe {
            processor
                .process_macro(&mut mac_context)
                .map_err(|e| CompilerError::runtime("Makepad Compiler - Script", &e.to_string()))?;
        }

        i.path = parse_str(&mac_context.ident).unwrap();
        i.tokens = parse_str::<TokenStream>(&mac_context.tokens).unwrap();
    }

    Ok(false)
}
