use std::collections::HashMap;

use crate::{
    builtin::BuiltinWidget,
    model::{
        role::{ForParent, Role},
        WidgetTemplate,
    },
    script::Impls,
    str_to_tk,
    traits::ToTokensExt,
};
use gen_analyzer::value::{For, IdentSplit};
use gen_utils::error::{CompilerError, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Field, Fields, ImplItem, ItemStruct};

/// for 语法糖处理器
pub struct SugarScript;

impl SugarScript {
    pub fn visit(
        prop: &mut ItemStruct,
        ptrs: &Vec<WidgetTemplate>,
        impls: &mut Impls,
    ) -> Result<(), Error> {
        // [for 所需要的组件指针] --------------------------------------------------------------------------------
        let mut for_ptrs = vec![];
        let fields = prop
            .fields
            .iter()
            .map(|f| {
                (
                    f.ident.as_ref().unwrap().to_string(),
                    f.ty.to_token_stream(),
                )
            })
            .collect();

        for index in 0..ptrs.len() {
            // [生成一个指针] ------------------------------------------------------------------------------------
            let ptr_ident = ptr_ident(index);
            for_ptrs.push(ptr_ident_field(&ptr_ident));
        }
        // [生成初始化代码] ----------------------------------------------------------------------------------
        let for_sc = Self::for_script(ptrs, fields);
        // [添加指针到prop中] ------------------------------------------------------------------------------------
        match &mut prop.fields {
            Fields::Named(fields) => {
                fields.named.extend(for_ptrs);
            }
            _ => {
                return Err(CompilerError::runtime(
                    "Makepad Compiler - Script",
                    "prop should be a struct with named fields",
                )
                .into())
            }
        }
        // [添加构建的方法到self_impl中] ----------------------------------------------------------------------------
        impls.self_impl.extend(for_sc);

        Ok(())
    }

    fn for_script(
        widgets: &Vec<WidgetTemplate>,
        fields: HashMap<String, TokenStream>,
    ) -> Vec<ImplItem> {
        let mut res = vec![];
        for widget in widgets {
            // 首先确定这个ptr是否是嵌套的for, 如果不是直接生成, 如果father是for, 返回None, 等待father生成
            match (widget.role.is_single_for(), widget.role.is_nested_for()) {
                (true, false) | (false, false) => {
                    let mut expr = TokenStream::new();
                    let mut sugar_fn = None;
                    let mut value_ty = None;
                    let _ = Self::nested(
                        &mut expr,
                        &mut sugar_fn,
                        &mut value_ty,
                        &widget.role,
                        widget.children.as_ref(),
                        widgets,
                        &fields,
                        true,
                    );
                    res.push(parse_quote! {
                        fn #sugar_fn(&mut self, cx: &mut Cx, value: &#value_ty) -> (){
                            #expr
                        }
                    });
                }
                (false, true) | (true, true) => {}
            };
        }
        res
    }

    /// 循环嵌套生成for语法糖
    fn nested(
        expr: &mut TokenStream,
        sugar_fn: &mut Option<TokenStream>,
        value_ty: &mut Option<TokenStream>,
        widget_role: &Role,
        widget_children: Option<&Vec<WidgetTemplate>>,
        ptrs: &Vec<WidgetTemplate>,
        fields: &HashMap<String, TokenStream>,
        is_outter: bool,
    ) -> Result<(), Error> {
        // [for sugar script] --------------------------------------------------------------------------------
        if let Role::For {
            parent,
            creditial,
            origin_pos,
            props,
            children,
            name,
            id,
        } = widget_role
        {
            // [ptr_ident] -------------------------------------------------------------------------------------
            let ptr_ident =
                if let Some(index) = ptrs.iter().position(|p| p.id.as_ref().unwrap() == id) {
                    ptr_ident(index)
                } else {
                    return Err(CompilerError::runtime(
                        "Makepad Compiler - Script",
                        "can not find the ptr in ptrs",
                    )
                    .into());
                };

            // [as_widget] -------------------------------------------------------------------------------------
            let as_widget = str_to_tk!(&format!(
                "as_{}",
                BuiltinWidget::builtin_name_or_snake(name)
            ))?;
            // [about parent] ----------------------------------------------------------------------------------
            let f_creditial = parent.creditial.as_ref();
            let is_role_for = parent.is_for();
            let ForParent {
                id, name, is_root, ..
            } = parent;
            // [len expr] --------------------------------------------------------------------------------------
            let (len_ident, len_call) = single_iter_len(creditial, f_creditial);
            let len_expr = quote! {
                let #len_ident = self.#len_call;
            };
            // [nested for arg and father] ---------------------------------------------------------------------
            let father = if is_role_for {
                quote! {widget_target}
            } else {
                let widget = str_to_tk!(&BuiltinWidget::builtin_name_or_snake(name))?;
                let id = str_to_tk!(id)?;
                quote! {
                    self.#widget(id!(#id))
                }
            };
            // [remove expr for children] --------------------------------------------------------------------
            let children_prefix = if *is_root {
                quote! {self}
            } else {
                quote! {father}
            };
            let remove_expr = quote! {
                if #len_ident > 0 && #children_prefix.children.len() > #origin_pos{
                    for _ in #origin_pos..(#origin_pos + #len_ident) {
                        #children_prefix.children.remove(#origin_pos);
                    }
                }
            };
            // [loop ident] -------------------------------------------------------------------------------------
            let loop_ident = creditial.fmt_iter_ident();
            // [redraw] ---------------------------------------------------------------------------------------
            let (redraw_expr, loop_ident) = if is_outter {
                // [sugar fn] ---------------------------------------------------------------------------------
                sugar_fn.replace(sugar_for_fn_ident(&creditial.iter_ident_as_fn()));
                // get value_ty from fields
                fields.get(&loop_ident).map(|v| value_ty.replace(v.clone()));
                (
                    Some(quote! {
                        #children_prefix.redraw(cx);
                    }),
                    quote! {value},
                )
            } else {
                (None, str_to_tk!(&loop_ident)?)
            };
            // [for loop expr] ---------------------------------------------------------------------------------
            let enumerate = str_to_tk!(&creditial.fmt_enumerate())?;
            let index = str_to_tk!(&creditial.fmt_index())?;
            let item_clone = str_to_tk!(&creditial.fmt_item_clone_tk())?;
            // 根据props生成对应需要设置的方法
            let mut set_props = props.iter().fold(TokenStream::new(), |mut tk, (k, v)| {
                let set_fn = str_to_tk!(&format!("set_{}", v)).unwrap();
                let v = str_to_tk!(k).unwrap();
                tk.extend(quote! {
                    widget_target.#set_fn(cx, #v);
                });
                tk
            });
            // 获取子组件中涉及到的绑定并生成TokenStream 代码
            if let Some(children) = widget_children {
                set_props.extend(get_children_sugar_binds(children, &creditial));
            }

            // [nested] ----------------------------------------------------------------------------------------
            let mut nested_expr = TokenStream::new();
            for child in children {
                Self::nested(
                    &mut nested_expr,
                    sugar_fn,
                    value_ty,
                    child,
                    widget_children,
                    ptrs,
                    fields,
                    false,
                )?;
            }
            // 调用as_widget
            let let_as_widget = if !set_props.is_empty() || !nested_expr.is_empty() {
                Some(quote! {
                    let widget_target = widget_ref.#as_widget();
                })
            } else {
                None
            };

            let for_loop_expr = quote! {
                for #enumerate in #loop_ident.iter().enumerate() {
                    #item_clone
                    let widget_ref = WidgetRef::new_from_ptr(cx, self.#ptr_ident);
                    #let_as_widget
                    #nested_expr
                    #set_props
                    #children_prefix.children.insert(#origin_pos + #index , (LiveId(#index as u64), widget_ref));
                }
            };

            // [if let expression for parent] ----------------------------------------------------------------
            let loop_expr = if *is_root {
                quote! {
                    #len_expr
                    #remove_expr
                    #for_loop_expr
                    #redraw_expr
                }
            } else {
                quote! {
                    #len_expr
                    if let Some(mut father) = #father.borrow_mut() {
                        #remove_expr
                        #for_loop_expr
                        #redraw_expr
                    }
                }
            };

            expr.extend(loop_expr);
            return Ok(());
        } else {
            return Err(CompilerError::runtime(
                "Makepad Compiler - Script",
                "current sugar script is only for `for`",
            )
            .into());
        }
    }
}

pub fn ptr_ident(index: usize) -> TokenStream {
    str_to_tk!(format!("item_ptr{}", index).as_str()).unwrap()
}

pub fn sugar_for_fn_ident(ident: &str) -> TokenStream {
    str_to_tk!(format!("sugar_for_{}", ident).as_str()).unwrap()
}

pub fn ptr_ident_field(ident: &TokenStream) -> Field {
    parse_quote! {
        #[live]
        #ident: Option<LivePtr>
    }
}

/// 通过father for来获取子组件中涉及到的绑定并生成TokenStream 代码
fn get_children_sugar_binds(children: &Vec<WidgetTemplate>, father_for: &For) -> TokenStream {
    let mut tokens = TokenStream::new();
    for child in children {
        // 遍历binds过滤出使用了father for的index或item
        if let Some(binds) = child.binds.as_ref() {
            binds.iter().enumerate().for_each(|(i, (k, v))| {
                // 比较特殊，需要找v中是否包含father_for的index或item
                if father_for.is_use_index(&k) || father_for.is_use_item(&k) {
                    let name = child.ty.snake_name();
                    let widget = str_to_tk!(&name).unwrap();
                    let widget_id =
                        str_to_tk!(&child.id.as_ref().unwrap_or(&format!("{}{}", &name, i)))
                            .unwrap();
                    let set_fn = str_to_tk!(&format!("set_{}", v)).unwrap();
                    let value = str_to_tk!(&k).unwrap();
                    tokens.extend(quote! {
                        widget_target.#widget(id!(#widget_id)).#set_fn(cx, #value);
                    });
                }
            })
        }
    }

    tokens
}

/// 嵌套访问for role, 目的是为了生成需要的arg len的参数
pub fn visit_for_args(role: &Role, tk: &mut Vec<TokenStream>, calls: &mut Vec<TokenStream>) -> () {
    if let Role::For {
        creditial,
        children,
        ..
    } = role
    {
        let suffix = creditial.iter_ident_as_fn();
        let len_ident = str_to_tk!(&format!("len_{}", suffix)).unwrap();
        tk.push(quote! {
            #len_ident: usize
        });
        calls.push(len_ident);
        for child in children {
            if child.is_for() {
                visit_for_args(&*child, tk, calls);
            }
        }
    }
}

fn in_father_and_replace(
    tk: &mut TokenStream,
    ident: &str,
    father: Option<&For>,
) -> Option<String> {
    if let Some(father) = father {
        if father.is_use_index(ident) || father.is_use_item(ident) {
            let prefix = father.fmt_iter_ident();
            *tk = str_to_tk!(&prefix).unwrap();
            return Some(father.fmt_index());
        }
    }

    None
}

/// 访问for role, 目的是为了生成需要的len的参数, 生成的代码如下:
/// - `let len_ident = self.iter_ident.len();`
/// - `let len_ident = self.iter_ident.get(index).len();`
/// - `let len_ident = self.iter_ident.map_or(0, |v| v.len());`
fn single_iter_len(creditial: &For, f_creditial: Option<&For>) -> (TokenStream, TokenStream) {
    let suffix = str_to_tk!(&creditial.iter_ident_as_fn()).unwrap();
    let len_ident = str_to_tk!(&format!("len_{}", suffix)).unwrap();
    let iter_len = creditial.iter_ident.len();
    let mut last = None;
    let mut len_call = if iter_len == 1 {
        let mut ident = creditial
            .iter_ident
            .iter()
            .next()
            .unwrap()
            .to_token_stream()
            .unwrap();
        let ident_str = ident.to_string();
        if let Some(index) = in_father_and_replace(&mut ident, &ident_str, f_creditial) {
            let index = str_to_tk!(&index).unwrap();
            last = Some(IdentSplit::Holder);
            quote! {
                #ident.get(#index)
            }
        } else {
            quote! {
                #ident.len()
            }
        }
    } else if iter_len > 1 {
        let first = creditial.iter_ident.first().unwrap().name.to_string();
        let mut ident = str_to_tk!(&first).unwrap();
        let suffix = creditial
            .iter_ident
            .iter()
            .skip(1)
            .fold(TokenStream::new(), |mut tk, i| {
                match i.split {
                    gen_analyzer::value::IdentSplit::None
                    | gen_analyzer::value::IdentSplit::Dot => {
                        tk.extend(i.to_token_stream());
                    }
                    gen_analyzer::value::IdentSplit::Holder => {
                        // [index] -> get(index)
                        let index = str_to_tk!(&i.name).unwrap();
                        tk.extend(quote! {
                            .get(#index)
                        });
                    }
                }
                last.replace(i.split);
                tk
            });
        let _ = in_father_and_replace(&mut ident, &first, f_creditial);
        ident.extend(suffix);
        ident
    } else {
        panic!("iter_ident len should be greater than 0")
    };

    if let Some(last) = last {
        match last {
            gen_analyzer::value::IdentSplit::None | gen_analyzer::value::IdentSplit::Dot => {
                len_call.extend(quote! { .len() })
            }

            gen_analyzer::value::IdentSplit::Holder => len_call.extend(quote! {
                .map_or(0, |v| v.len())
            }),
        }
    }

    (len_ident, len_call)
}
