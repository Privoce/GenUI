mod define;
mod ty;

use std::collections::HashMap;
pub use define::*;
use crate::{
    builtin::{
        widget::{Root, RootConf, Window},
        BuiltinWidget,
    },
    traits::ToTokensExt,
    visitor::ptr_ident,
};

use gen_utils::common::{punct_alone, snake_to_camel};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;
use crate::token::ToLiveDesign;
use gen_utils::error::Error;
pub use ty::WidgetType;
use super::role::Role;

/// # 通用Widget模型
/// 这个Widget模型主要用于抽象整个Makepad Widget结构
#[derive(Debug, Clone)]
pub struct WidgetTemplate {
    /// 虽然这里是Option，但是在生成代码的时候，这个id是自动填充的，见[utils::Ulid]
    pub id: Option<String>,
    pub is_root: bool,
    pub as_prop: Option<String>,
    pub is_static: bool,
    pub ty: WidgetType,
    pub children: Option<Vec<WidgetTemplate>>,
    pub role: Role,
    pub binds: Option<HashMap<String, String>>,
}

impl WidgetTemplate {
    pub fn is_global(&self) -> bool {
        matches!(self.ty, WidgetType::Global(_))
    }
    pub fn root_name(&self) -> TokenStream {
        self.ty.root_name()
    }
    pub fn is_define_root_and<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&DefineWidget) -> R,
    {
        if let WidgetType::Define(define) = &self.ty {
            if self.is_root {
                return Some(f(define));
            }
        }

        None
    }
    pub fn to_token_stream(
        &self,
        ptrs: Option<&Vec<WidgetTemplate>>,
    ) -> Result<TokenStream, Error> {
        if let WidgetType::Global(globals) = &self.ty {
            return ToTokensExt::to_token_stream(globals);
        } else {
            return Ok(self.live_node(ptrs));
        }
    }
    pub fn ptr_to_token_stream(ptrs: Option<&Vec<WidgetTemplate>>) -> Option<TokenStream> {
        ptrs.map(|ptrs| {
            ptrs.iter()
                .enumerate()
                .fold(TokenStream::new(), |mut tk, (index, item)| {
                    // let ptr_ident =
                    //     parse_str::<TokenStream>(format!("item_ptr{}", index).as_str()).unwrap();
                    let ptr_ident = ptr_ident(index);
                    let widget_name = item.ty.name();
                    let widget_props = item.ty.props();
                    let children = item.children.as_ref().map(|children| {
                        children.iter().fold(TokenStream::new(), |mut tk, child| {
                            tk.extend(child.to_token_stream(None).unwrap());
                            tk
                        })
                    });

                    let item = quote! {
                        <#widget_name>{
                            #widget_props
                            #children
                        },
                    };
                    tk.extend(quote! {
                       #ptr_ident: #item
                    });

                    tk
                })
        })
    }

    pub fn live_node(&self, ptrs: Option<&Vec<WidgetTemplate>>) -> TokenStream {
        let id = self.id.as_ref();
        let as_prop = self.as_prop.as_ref();
        let widget = &self.ty;
        let children = self.children.as_ref();
        let is_root = self.is_root;
        // [id, signal: `:` or `=`, name] ----------------------------------------------------------------------
        let widget_name = widget.name();
        let (id, sig, widget_name) = if is_root {
            if widget.is_define(){
                let id = widget_name.clone();
                // 如果是define的root节点，那么name属性就是下面需要的id
                let name = quote! {
                    {{#widget_name}}
                };
                (id, Some(punct_alone('=')), name)
            }else{
                // 不是define, 那么就是id的大写形式
                let id = parse_str::<TokenStream>(&snake_to_camel(id.unwrap())).unwrap();
                let name = quote! {
                    <#widget_name>
                };
                (id, Some(punct_alone('=')), name)
            }
        }else{
            // 不是root节点, 那么就需要对组件类型进行检查
            // 如果是as_prop, 那么id就是as_prop的值, sig则是`:`, name则是组件名
            if let Some(prop_slot) = as_prop{
                let id = parse_str::<TokenStream>(&prop_slot).unwrap();
                let name = quote! {
                    <#widget_name>
                };

                (id, Some(punct_alone(':')), name)
            }else{
                // 不是as_prop, 不是root, 那么就是最普通的组件情况, 直接使用id, 也无需大写
                let name = quote! {
                    <#widget_name>
                };
                let id = parse_str::<TokenStream>(id.unwrap()).unwrap();
                (id, Some(punct_alone('=')), name)
            }
        };

        // [widget props] -------------------------------------------------------------------------------------
        let widget_props = widget.props();
        // [children] -----------------------------------------------------------------------------------------
        let children = children.map(|children| {
            children.iter().fold(TokenStream::new(), |mut tk, child| {
                // here child widget must be static and use live_node_static!
                // tk.extend(
                //     ToTokensExt::to_token_stream(child)
                //         .expect("if here has error, check ast! cause it should never exist error!"),
                // );
                tk.extend(child.to_token_stream(None));
                tk
            })
        });
        // [widget ptrs] --------------------------------------------------------------------------------------
        let widget_ptrs = WidgetTemplate::ptr_to_token_stream(ptrs);

        quote! {
            #id #sig #widget_name{
                #widget_ptrs
                #widget_props
                #children
            }
        }
    }
}

/// generate root widget
/// ```
/// <Root> {
///     <GWindow> {
///         height: Fill, width: Fill, window: {inner_size: vec2(1080, 720)}, flow: Down
///     }
/// }
/// ```
impl From<(String, &RootConf)> for WidgetTemplate {
    fn from(value: (String, &RootConf)) -> Self {
        let (root_name, conf) = value;

        let root_ty = WidgetType::Builtin(BuiltinWidget::Root(Root));
        let window_ty = WidgetType::Builtin(BuiltinWidget::Window(Window {
            prop: Some(conf.window.clone()),
        }));
        let root = WidgetType::Define(DefineWidget {
            name: root_name,
            prop: None,
            role: Role::Normal,
        });

        Self {
            id: None,
            is_root: true,
            as_prop: None,
            is_static: true,
            ty: root_ty,
            role: Role::default(),
            binds: None,
            children: Some(vec![WidgetTemplate {
                id: None,
                is_root: false,
                as_prop: None,
                is_static: true,
                ty: window_ty,
                role: Role::default(),
                binds: None,
                children: Some(vec![WidgetTemplate {
                    id: Some("body".to_string()),
                    is_root: false,
                    as_prop: None,
                    is_static: true,
                    ty: root,
                    children: None,
                    role: Role::default(),
                    binds: None,
                }]),
            }]),
        }
    }
}
