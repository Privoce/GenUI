mod instance;

use super::{get_attr_from_field_type, lifetime::LifeTime};
use crate::error::{AttrMacroError, ImportError, SCResult};
use instance::{get_ident, proc_tk};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Display;
use syn::{parse2, parse_quote, Field, Fields, Ident, ItemEnum, ItemStruct, Local, Stmt};

/// # GenUI 脚本桥接模型
/// 用于将GenUI的属性宏，生命周期宏，事件宏，实例宏等转换为普通的RUST代码
/// 每个组件都会有一个ScriptBridger，用于存放组件的所有信息
#[derive(Debug, Default, Clone)]
pub struct ScriptBridger {
    /// 组件的导入
    /// 例如：
    /// ```rust
    /// // 表示导入my_button mod下所有的组件
    /// import!{
    ///     crate::views::my_button::*;
    /// }
    /// ```
    pub imports: Option<TokenStream>,
    /// 组件的属性
    /// 使用`#[component]`属性宏来实现
    /// 例如：
    /// ```rust
    /// #[component]
    /// pub struct AProp{
    ///    pub name: String,
    /// }
    /// ```
    pub prop: Option<ItemStruct>,
    /// 组件事件的枚举，使用`#[event]`属性宏来实现，虽然常规想法是只有一个，但其实是可以是多个
    /// 例如:
    /// ```rust
    /// #[event]
    /// pub enum AEvent{
    ///   Clicked,
    /// }
    /// ```
    pub events: Option<Vec<ItemEnum>>,
    /// # Instance实例
    /// 使用`default_prop!`宏来实现
    /// instance结构体用来表示组件的实例
    /// 该结构体会被后续用于语义分析来进行实例的作用域管理
    /// ## 语法
    /// ```rust
    /// let mut prop = default_prop!{
    ///     MyStruct{
    ///        a: 10,
    ///        b: 10.0,
    ///    }
    /// };
    /// ```
    pub instance: Option<Local>,
    /// 用于存放组件的函数，在GenUI中组件的函数可能会是闭包，也可能是普通的函数
    /// 如果是闭包则需要进行转换，转为普通的函数
    /// 例如:
    /// ```rust
    /// pub fn a_fn(){
    ///    println!("Hello World");
    /// }
    /// let mut a_clicked = ||{
    ///     println!("Clicked");
    /// };
    /// ```
    pub fns: Option<Vec<syn::ItemFn>>,
    /// 使用liftime属性宏系列的声明周期事件
    /// 分为两种大类: 1. AppMain 2. Widget
    /// 但他们都是ItemFn, 这里需要直接使用LifeTime来进行区分方便后续处理
    /// see [LifeTime]
    pub lifetimes: Option<LifeTime>,
    /// 其他的RUST代码
    /// 例如模块导入，模块声明等，与GenUI无关的代码
    pub others: Option<Vec<Stmt>>,
}

impl ScriptBridger {
    pub fn clear(&mut self) -> () {
        self.imports = None;
        self.prop = None;
        self.events = None;
        self.instance = None;
        self.fns = None;
        self.lifetimes = None;
        self.others = None;
    }
    pub fn extend_prop_fields(&mut self, items: Vec<Field>) -> () {
        let prop = self
            .prop
            .as_mut()
            .expect("prop is none, you should use #[component] to sign a struct as prop");

        match &mut prop.fields {
            Fields::Named(fields) => {
                fields.named.extend(items);
            }
            _ => panic!("prop should be a struct with named fields"),
        }
    }
    pub fn push_lifetime(&mut self, item: &syn::ItemFn) -> SCResult<()> {
        if self.lifetimes.is_none() {
            let item = LifeTime::try_from(item)?;
            self.lifetimes.replace(item);
            return Ok(());
        }

        self.lifetimes.as_mut().unwrap().push(item)
    }
    /// ## 添加函数以及闭包，闭包需要经过ClosureVisitor转换后再添加
    pub fn push_fn(&mut self, item: syn::ItemFn) -> SCResult<()> {
        if self.fns.is_none() {
            self.fns = Some(vec![]);
        }
        self.fns.as_mut().unwrap().push(item);
        Ok(())
    }
    /// ## 设置使用`default_prop!`宏的实例
    pub fn set_instance(&mut self, instance: Local) -> SCResult<()> {
        if self.instance.is_none() {
            self.instance.replace(instance);
            return Ok(());
        }

        Err(AttrMacroError::MultiInstanceMacro.into())
    }
    /// ## 设置import!宏的导入
    pub fn set_import(&mut self, imports: Option<TokenStream>) -> SCResult<()> {
        if self.imports.is_none() {
            self.imports = imports;
            return Ok(());
        }
        Err(ImportError::MultiImportMacro.into())
    }
    /// ## 相当于`#[component]`属性宏的实现 See [impl_attr_prop]
    pub fn set_prop(&mut self, item: Option<ItemStruct>) -> SCResult<()> {
        if self.prop.is_none() {
            // 在设置之前，依据属性宏的实现来替换这个ItemStruct
            if let Some(input_struct) = item {
                let name = &input_struct.ident;
                let vis = &input_struct.vis;
                let mut struct_attrs = input_struct.attrs;
                // 首先先把`#[component]`宏去除
                struct_attrs.retain(|attr| !attr.path().is_ident("component"));
                let derives = parse_quote!(#[derive(Live, Widget)]);
                struct_attrs.push(derives);
                let mut field_tks = vec![quote! {
                    #[deref]
                    pub deref_widget: GView,
                }];

                if let syn::Fields::Named(fields) = input_struct.fields {
                    for field in fields.named {
                        let field_name = field.ident;
                        let field_type = field.ty;
                        let field_attr = get_attr_from_field_type(&field_type, &field.attrs)?;
                        let field_vis = field.vis;

                        field_tks.push(quote! {
                            #field_attr
                            #field_vis #field_name: #field_type,
                        });
                    }
                }

                let expanded = quote! {
                    #(#struct_attrs)*
                    #vis struct #name {
                        #(#field_tks)*
                    }
                };
                let prop = parse2(expanded).unwrap();
                self.prop = Some(prop);
            }
            return Ok(());
        }
        Err(AttrMacroError::MultiPropMacro.into())
    }
    pub fn push_event(&mut self, item: Option<ItemEnum>) -> SCResult<()> {
        if let Some(input_enum) = item {
            if self.events.is_none() {
                self.events = Some(vec![]);
            }
            let enum_vis = &input_enum.vis;
            let enum_name = &input_enum.ident;
            let mut enum_attrs = input_enum.attrs;
            enum_attrs.retain(|attr| !attr.path().is_ident("event"));
            enum_attrs.push(parse_quote!(#[derive(DefaultNone)]));

            let variants = input_enum.variants.iter().map(|var| {
                let variant_name = &var.ident;
                let fields = &var.fields;
                // 保持原始变体的属性
                let attrs = &var.attrs;
                quote! {
                    #(#attrs)*
                    #variant_name #fields
                }
            });

            let expanded = quote! {
                #(#enum_attrs)*
                #enum_vis enum #enum_name {
                    #(#variants,)*
                    None
                }
            };
            self.events
                .as_mut()
                .unwrap()
                .push(parse2(expanded).unwrap());
            return Ok(());
        } else {
            return Ok(());
        }
    }
    pub fn push_other(&mut self, stmt: &Stmt) {
        if self.others.is_none() {
            self.others = Some(vec![]);
        }
        self.others.as_mut().unwrap().push(stmt.clone());
    }
    /// ## 有关于过程宏的工作状态
    pub fn procedural_macro_worked(&self) -> bool {
        self.imports.is_some()
    }
    /// ## 有关于属性宏的工作状态
    pub fn attr_prop_worked(&self) -> bool {
        self.prop.is_some()
    }
    pub fn instance_macro_worked(&self) -> bool {
        self.instance.is_some()
    }
    pub fn gen_instance_tk(&self) -> TokenStream {
        self.instance
            .as_ref()
            .map_or_else(TokenStream::new, |instance| proc_tk(instance))
    }
    pub fn instance_ident(&self) -> Option<&Ident> {
        self.instance.as_ref().map(|instance| get_ident(instance))
    }
}

// 这个实现是为了在文档中显示，方便查看，目前用作测试
impl ToTokens for ScriptBridger {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // let imports = &self.imports;
        // let prop = &self.prop;
        let others = if let Some(other) = &self.others {
            let tk = other.iter().fold(TokenStream::new(), |mut tk, stmt| {
                tk.extend(quote! {#stmt});
                tk
            });
            Some(tk)
        } else {
            None
        };
        tokens.extend(quote! {
            #others
        });
    }
}

// 这个实现是为了在文档中显示，方便查看，目前用作测试
impl Display for ScriptBridger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imports = &self.imports;
        let prop = &self.prop;
        let others = if let Some(other) = &self.others {
            let tk = other.iter().fold(TokenStream::new(), |mut tk, stmt| {
                tk.extend(quote! {#stmt});
                tk
            });
            Some(tk)
        } else {
            None
        };

        let imports_str = imports
            .as_ref()
            .map_or_else(|| String::from("-"), |tk| tk.to_string());
        let prop_str = prop
            .as_ref()
            .map_or_else(|| String::from("-"), |tk| tk.to_token_stream().to_string());

        let event_str = if let Some(events) = &self.events {
            let mut event_str = String::new();
            for event in events {
                event_str.push_str(&event.to_token_stream().to_string());
            }
            event_str
        } else {
            String::from("-")
        };
        let instance_str = self
            .instance
            .as_ref()
            .map_or_else(|| String::from("-"), |tk| tk.to_token_stream().to_string());
        let others_str = others
            .as_ref()
            .map_or_else(|| String::from("-"), |tk| tk.to_string());

        let fn_str = if let Some(fns) = &self.fns {
            let mut fn_str = String::new();
            for f in fns {
                fn_str.push_str(&f.to_token_stream().to_string());
                fn_str.push('\n');
            }
            fn_str
        } else {
            String::from("-")
        };

        let lifetime_str = if let Some(lifetimes) = &self.lifetimes {
            lifetimes.to_token_stream().to_string()
        } else {
            String::from("-")
        };

        f.write_fmt(format_args!(
            "
## imports:\n```\n{}\n```\n---\n\n
## prop:\n```rust\n{}\n```\n---\n\n
## events:\n```rust\n{}\n```\n---\n\n
## instance:\n```rust\n{}\n```\n---\n\n
## fn:\n```rust\n{}\n```\n---\n\n
## lifetimes:\n```rust\n{}\n```\n---\n\n
## other:\n```rust\n{}\n```\n---\n\n
            ",
            imports_str, prop_str, event_str, instance_str, fn_str, lifetime_str, others_str
        ))
    }
}
