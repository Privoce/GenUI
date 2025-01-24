use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, parse_str, Stmt};

use crate::{
    builtin::BuiltinWidget,
    model::{PropBinds, TemplatePtrs},
    script::Impls,
    visitor::sugar_for_fn_ident,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GetSet {
    Get,
    Set,
    UnMatch,
}

impl GetSet {
    #[allow(dead_code)]
    pub fn is_get(&self) -> bool {
        matches!(self, GetSet::Get)
    }
    pub fn is_set(&self) -> bool {
        matches!(self, GetSet::Set)
    }
    pub fn is_unmatch(&self) -> bool {
        matches!(self, GetSet::UnMatch)
    }

    /// 生成组件双向绑定需要的get和set方法
    /// 这个方法一般由PropLzVisitor调用(通过传入的ItemStruct生成所有的get和set方法)
    /// ⚠️ set方法需要注意：需要添加组件对数据的绑定和重绘
    pub fn create_get_set(
        field: &str,
        ty: &str,
        binds: &PropBinds,
        // is_for: bool,
        ptrs: &TemplatePtrs,
        impls: &mut Impls,
    ) -> Result<(), Error> {
        let mut bind_and_redraw = if let Some(binds) = binds.get(field) {
            binds.iter().fold(TokenStream::new(), |mut tk, widget| {
                let widget_name = parse_str::<TokenStream>(&widget.widget_name()).unwrap();
                let widget_id = parse_str::<TokenStream>(&widget.id).unwrap();
                let set_prop_fn =
                    parse_str::<TokenStream>(&format!("set_{}", &widget.prop)).unwrap();
                let set_prop = if let Some((prop_widget, as_prop)) = widget.as_prop.as_ref() {
                    let prop_widget = BuiltinWidget::builtin_name_or_snake(prop_widget);
                    let as_prop_widget =
                        parse_str::<TokenStream>(&format!("as_{}", prop_widget)).unwrap();
                    let as_prop = parse_str::<TokenStream>(as_prop).unwrap();

                    quote! {
                        if let Some(mut c_ref) = self.#widget_name(id!(#widget_id)).borrow_mut(){
                            let slot_widget = c_ref.#as_prop.#as_prop_widget();
                            slot_widget.#set_prop_fn(cx, value.clone());
                        }

                    }
                } else {
                    quote! {
                        let widget = self.#widget_name(id!(#widget_id));
                        widget.#set_prop_fn(cx, value.clone());
                    }
                };

                tk.extend(set_prop);
                tk
            })
        } else {
            // 有可能是这个变量并没有绑定到组件上, 但其实也需要生成get和set方法，只是没有绑定部分的代码
            TokenStream::new()
        };

        // let field_tk = parse_str::<TokenStream>(field).unwrap();
        let sugar_for_ident = sugar_for_fn_ident(field);
        if let Some(_) = ptrs.iter().find(|ptr| {
            if let Some(f) = ptr.role.for_field() {
                f == field
            } else {
                false
            }
        }) {
            // sugar_fn_call
            bind_and_redraw.extend(quote! {
                self.#sugar_for_ident(cx, &value);
            });
        }

        let (self_get, self_get_ref) = Self::create_get_fn(field, ty);
        let (self_set, self_set_ref) = Self::create_set_fn(field, ty, bind_and_redraw);

        impls.self_impl.extend(vec![self_get, self_set]);
        impls.self_ref_impl.extend(vec![self_get_ref, self_set_ref]);
        Ok(())
    }

    pub fn getter_setter(ident: &TokenStream) -> Vec<Stmt> {
        vec![
            parse_quote! {
                fn setter<F>(&self, cx: &mut Cx, f: F) -> ()
                where
                    F: FnOnce(&mut std::cell::RefMut<'_, #ident>, &mut Cx),
                {
                    if let Some(mut c_ref) = self.borrow_mut() {
                        f(&mut c_ref, cx);
                    }
                }
            },
            parse_quote! {
                fn getter<T, F>(&self, f: F) -> T
                where
                    F: Fn(&std::cell::Ref<'_, #ident>) -> T,
                    T: Default,
                {
                    if let Some(c_ref) = self.borrow() {
                        f(&c_ref)
                    } else {
                        T::default()
                    }
                }
            },
        ]
    }
    /// 生成双向绑定的get方法
    fn create_get_fn(field: &str, ty: &str) -> (Stmt, Stmt) {
        let fn_name = parse_str::<TokenStream>(&format!("get_{}", field)).unwrap();
        let field = parse_str::<TokenStream>(field).unwrap();
        let ty = parse_str::<TokenStream>(ty).unwrap();
        (
            parse_quote! {
                fn #fn_name(&self) -> #ty {
                    self.#field.clone()
                }
            },
            parse_quote! {
                pub fn #fn_name(&self) -> #ty{
                    self.getter(|c_ref| c_ref.#field.clone())
                }
            },
        )
    }
    /// 生成双向绑定的set方法
    fn create_set_fn(field: &str, ty: &str, bind_and_redraw: TokenStream) -> (Stmt, Stmt) {
        let fn_set = parse_str::<TokenStream>(&format!("set_{}", field)).unwrap();
        let field = parse_str::<TokenStream>(field).unwrap();
        let ty = parse_str::<TokenStream>(ty).unwrap();
        (
            parse_quote! {
                fn #fn_set(&mut self, cx: &mut Cx, value: #ty) -> (){
                    #bind_and_redraw
                    self.#field = value.clone();
                }
            },
            parse_quote! {
                pub fn #fn_set(&self, cx: &mut Cx, value: #ty) -> () {
                    self.setter(cx, |c_ref, cx| {c_ref.#fn_set(cx, value);});
                }
            },
        )
    }
}

impl From<&str> for GetSet {
    fn from(s: &str) -> Self {
        return if s.starts_with("get_") {
            GetSet::Get
        } else if s.starts_with("set_") {
            GetSet::Set
        } else {
            GetSet::UnMatch
        };
    }
}
