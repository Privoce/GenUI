use gen_parser::{Script, Value};

use gen_utils::common::ident;
use proc_macro2::Span;
use syn::{Block, Expr, Ident, ItemImpl, Meta, Pat, Stmt, StmtMacro};

use crate::model::PropTree;

use super::{r#use::UseMod, LifeTime, PropFn, PropFnOnly};

/// # GenUI Script Model
/// Model here is used to represent the script of the component or logic code
///
/// This ScriptModel is from `gen_parser::ast::script::Script`
#[derive(Debug, Clone)]
pub enum ScriptModel {
    /// General Script Model for GenUI component Model
    Gen(GenScriptModel),
    /// Rust code
    Rs(Block),
    /// Ets code
    ETs(String),
    /// Other language code, in some framework, it may support other language code
    Other { lang: String, code: String },
}

impl ScriptModel {
    pub fn from(script: Script) -> Self {
        match script {
            Script::Rs(rs) => ScriptModel::Rs(rs),
            Script::ETs(ets) => ScriptModel::ETs(ets),
            Script::Other { lang, code } => ScriptModel::Other { lang, code },
        }
    }
    pub fn from_gen(script: Script, bind_fn_tree: &(PropTree, PropTree)) -> Self {
        if let Script::Rs(rs) = script {
            return ScriptModel::Gen(GenScriptModel::new(rs, bind_fn_tree));
        }
        panic!("Only Rs can be converted to GenScriptModel")
    }
}

impl Default for ScriptModel {
    fn default() -> Self {
        ScriptModel::Gen(GenScriptModel::default())
    }
}

impl From<Block> for ScriptModel {
    fn from(value: Block) -> Self {
        ScriptModel::Rs(value)
    }
}

impl From<GenScriptModel> for ScriptModel {
    fn from(value: GenScriptModel) -> Self {
        ScriptModel::Gen(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GenScriptModel {
    /// 使用的包,依赖
    pub uses: Option<UseMod>,
    /// 组件的导入
    /// 例如：
    /// ```rust
    /// // 表示导入my_button mod下所有的组件
    /// import!{
    ///     crate::views::my_button::*;
    /// }
    /// ```
    pub imports: Option<StmtMacro>,
    /// 组件的属性
    /// 这表示组件允许外部传入给内部的属性，需要使用GenUI的Prop宏进行标注
    /// 例如：
    /// ```rust
    /// #[derive(Debug, Clone, PartialEq, Prop)]
    /// pub struct Props{
    ///     text: String,
    ///     height: f64,
    /// }
    /// ```
    // prop_ptr: Box<dyn Prop>,
    pub prop_ptr: Option<syn::ItemStruct>,
    /// 组件事件
    /// 事件也可以被认为是组件状态
    /// 由编写者决定，所以并不一定存在，但若存在则必须要使用GenUI的Event宏进行标注
    /// 例如，一个按钮组件，它有一个点击事件，那么这个按钮被点击时，这个事件就会被触发，也就是这个按钮进入了点击状态
    /// GenuI中事件实际上是由外部影响的
    /// 例如，在组件中有一个按钮，当这个按钮被点击时，这个按钮会激发组件的点击事件并把这个事件传递给外部（连带参数）
    /// 外部可以根据这个事件来做一些事情
    /// 对于定义组件时就需要相应的使用Rust编写
    /// ```rust
    /// #[derive(Debug, Clone, PartialEq, Event)]
    /// pub enum Events{
    ///     #[name("click")]
    ///     Clicked(//内部给到外部的参数),
    /// }
    /// ```
    // event_ptr: Box<dyn Event>,
    pub event_ptr: Option<syn::ItemEnum>,
    /// 组件的生命周期
    /// 在GenUI中声明周期使用宏来进行标注
    /// 例如: on_startup! on_shutdown!
    pub lifetimes: Option<LifeTime>,
    /// 表示当前组件的内部子组件的属性绑定
    pub sub_prop_binds: Option<Vec<PropFn>>,
    /// 表示当前组件的内部子组件的事件绑定
    pub sub_event_binds: Option<Vec<PropFn>>,
    /// 当前组件的实例
    /// 该实例在组件的draw_walk中被构建, 表示当前实例调用了default()方法来构建
    /// 例如：`let instance = MyButton::default();`
    /// 常见声明如下:
    /// ```rust
    /// <script>
    /// use gen_macros::{Prop};
    /// use makepad_widget::*;
    ///
    /// #[derive(Prop)]
    /// pub struct MyButtonWidget{
    ///     text: String,
    ///     walk: Walk, //(你不该设置这个属性⛔, 该属性是自动添加的)
    /// }
    ///
    /// let mut current_instance = MyButtonWidget::default(); // 这里的
    /// current_instance.text = "Hello".to_string(); // 这不是
    /// </script>
    /// ```
    pub current_instance: Option<CurrentInstance>,
    /// 对当前组件实例处理的代码
    /// 例如上面的代码中的`current_instance.text = "Hello".to_string();`
    /// 这里应该都是Expr，但使用Stmt，因为Stmt能表示完整语句
    pub instance_opt: Option<Vec<Stmt>>,
    pub instance_default_impl: Option<(Vec<PropFnOnly>, ItemImpl)>,
    /// 其他的代码，例如一些过程代码
    pub other: Option<Vec<syn::Stmt>>,
}

#[derive(Debug, Clone)]
pub struct CurrentInstance {
    pub is_mut: bool,
    pub name: Option<Ident>,
    pub ptr: Ident,
}

impl CurrentInstance {
    pub fn name(&self) -> Option<&Ident> {
        self.name.as_ref()
    }
}

impl GenScriptModel {
    pub fn new(block: Block, bind_fn_tree: &(PropTree, PropTree)) -> Self {
        build_script(block, bind_fn_tree)
    }
    pub fn get_current_instance(&self) -> Option<&CurrentInstance> {
        self.current_instance.as_ref()
    }
    pub fn get_uses(&self) -> Option<&UseMod> {
        self.uses.as_ref()
    }
    pub fn get_prop_ptr(&self) -> Option<&syn::ItemStruct> {
        self.prop_ptr.as_ref()
    }
    pub fn get_event_ptr(&self) -> Option<&syn::ItemEnum> {
        self.event_ptr.as_ref()
    }
    pub fn get_lifetimes(&self) -> Option<&LifeTime> {
        self.lifetimes.as_ref()
    }
    pub fn get_sub_prop_binds(&self) -> Option<&Vec<PropFn>> {
        self.sub_prop_binds.as_ref()
    }
    pub fn get_sub_event_binds(&self) -> Option<&Vec<PropFn>> {
        self.sub_event_binds.as_ref()
    }
    pub fn get_other(&self) -> Option<&Vec<syn::Stmt>> {
        self.other.as_ref()
    }
    pub fn set_uses(&mut self, uses: UseMod) {
        self.uses = Some(uses);
    }
    pub fn set_imports(&mut self, imports: Option<StmtMacro>) {
        self.imports = imports;
    }
    pub fn set_prop_ptr(&mut self, prop: syn::ItemStruct) {
        if self.prop_ptr.is_none() {
            let _ = self.prop_ptr.replace(prop);
            return;
        }
        panic!("Only one struct can be derived from Prop")
    }
    pub fn set_event_ptr(&mut self, event: syn::ItemEnum) {
        if self.event_ptr.is_none() {
            let _ = self.event_ptr.replace(event);
            return;
        }
        panic!("Only one enum can be derived from Event");
    }
    pub fn set_lifetimes(&mut self, lifetimes: Option<LifeTime>) {
        self.lifetimes = lifetimes;
    }
    pub fn push_other(&mut self, stmt: syn::Stmt) {
        if self.other.is_none() {
            self.other.replace(vec![stmt]);
        } else {
            self.other.as_mut().unwrap().push(stmt);
        }
    }
    pub fn push_sub_prop_binds(
        &mut self,
        bind_tree: &PropTree,
        ident: &str,
        code: &syn::Stmt,
    ) -> bool {
        push_sub_prop_fn(
            self,
            bind_tree,
            ident,
            code,
            |v| v.is_bind_and_get().unwrap().get_normal().unwrap(),
            |target, item| {
                if target.sub_prop_binds.is_none() {
                    let _ = target.sub_prop_binds.replace(vec![]);
                }
                let _ = target.sub_prop_binds.as_mut().unwrap().push(item);
            },
        )
    }
    pub fn push_sub_fn_binds(
        &mut self,
        bind_tree: &PropTree,
        ident: &str,
        code: &syn::Stmt,
    ) -> bool {
        push_sub_prop_fn(
            self,
            bind_tree,
            ident,
            code,
            |v| v.is_fn_and_get().unwrap().get_name(),
            |target, item| {
                if target.sub_event_binds.is_none() {
                    let _ = target.sub_event_binds.replace(vec![]);
                }
                let _ = target.sub_event_binds.as_mut().unwrap().push(item);
            },
        )
    }
    pub fn push_sub_prop_fn(
        &mut self,
        bind_fn_tree: &(PropTree, PropTree),
        ident: &str,
        code: &syn::Stmt,
    ) -> bool {
        if self.push_sub_prop_binds(&bind_fn_tree.0, ident, code) {
            return true;
        }
        self.push_sub_fn_binds(&bind_fn_tree.1, ident, code)
    }
}

fn build_script(block: Block, bind_fn_tree: &(PropTree, PropTree)) -> GenScriptModel {
    let stmts = block.stmts;

    let mut model = GenScriptModel::default();
    let mut lifetimes: Option<LifeTime> = None;

    for stmt in &stmts {
        match stmt {
            syn::Stmt::Item(item) => {
                match item {
                    syn::Item::Use(use_item) => {
                        // 过滤gen中的所有的依赖
                        if model.uses.is_none() {
                            model.uses.replace(UseMod::default());
                        }

                        model.uses.as_mut().unwrap().push(use_item.clone());
                    }
                    syn::Item::Impl(impl_item) => {
                        // 判断当前impl的实现是否当前实例的Default trait的实现
                        if let Some((_, trait_ident, is_for)) = impl_item.trait_.as_ref() {
                            if trait_ident
                                .segments
                                .first()
                                .unwrap()
                                .ident
                                .eq(&ident("Default"))
                                && is_for.eq(&syn::token::For::default())
                            {
                                if model.instance_default_impl.is_none() {
                                    model
                                        .instance_default_impl
                                        .replace(PropFnOnly::filter_default_impl(impl_item, &bind_fn_tree.0));
                                } else {
                                    panic!("Only one Instance Default trait impl can be used");
                                }
                            }
                        } else {
                            model.push_other(stmt.clone());
                        }
                    }
                    syn::Item::Struct(struct_item) => {
                        if model.prop_ptr.is_some() {
                            model.push_other(stmt.clone());
                            continue;
                        }
                        // 查看是否有`#[derive(Prop)]`的属性
                        // 如果有则将其将prop设置为Some
                        // 否则放到other中
                        for attr in struct_item.clone().attrs {
                            if let Meta::List(list) = &attr.meta {
                                if list.path.is_ident("derive")
                                    && list.tokens.to_string().contains("Prop")
                                {
                                    model.current_instance.replace(CurrentInstance {
                                        name: None,
                                        is_mut: false,
                                        ptr: struct_item.ident.clone(),
                                    });
                                    model.set_prop_ptr(struct_item.clone());
                                } else {
                                    model.push_other(stmt.clone());
                                }
                            }
                        }
                    }
                    syn::Item::Enum(enum_item) => {
                        if model.event_ptr.is_some() {
                            model.push_other(stmt.clone());
                            continue;
                        }
                        // 处理带有`#[derive(Event)]`的枚举
                        // 如果有则将其将event设置为Some
                        // 否则放到other中
                        for attr in enum_item.clone().attrs {
                            if let Meta::List(list) = &attr.meta {
                                if list.path.is_ident("derive")
                                    && list.tokens.to_string().contains("Event")
                                {
                                    model.set_event_ptr(enum_item.clone());
                                } else {
                                    model.push_other(stmt.clone());
                                }
                            }
                        }
                    }
                    _ => {
                        // 其他情况也直接放到other中
                        model.push_other(stmt.clone());
                    }
                }
            }
            syn::Stmt::Macro(item) => {
                if lifetimes.is_none() {
                    lifetimes.replace(LifeTime::default());
                }
                // 处理生命周期
                // 目前只处理带有`on_startup!, on_shutdown!`标识的
                // 其他的放到other中
                if item.mac.path.is_ident("on_startup") {
                    // 处理生命周期
                    lifetimes.as_mut().unwrap().set_startup(item.clone());
                } else if item.mac.path.is_ident("on_shutdown") {
                    lifetimes.as_mut().unwrap().set_shutdown(item.clone());
                } else if item.mac.path.is_ident("import") {
                    // 处理组件导入
                    if model.imports.is_none() {
                        model.imports.replace(item.clone());
                    } else {
                        panic!("Only one import! macro can be used");
                    }
                } else {
                    model.push_other(stmt.clone());
                }
            }
            syn::Stmt::Local(local) => {
                // 处理属性绑定 和 事件绑定
                // 查看是否有init
                if let Some(init) = &local.init {
                    // 查找init中的expr是否是ptr的default方法
                    if let Expr::Call(expr_call) = &*init.expr {
                        if let Expr::Path(expr_path) = &*expr_call.func {
                            if expr_path
                                .path
                                .segments
                                .last()
                                .unwrap()
                                .ident
                                .eq(&Ident::new("default", Span::call_site()))
                                && expr_path.path.segments[0].ident.eq(&model
                                    .current_instance
                                    .as_ref()
                                    .unwrap()
                                    .ptr)
                            {
                                // 如果是default方法
                                // 则查看是否有ident
                                // 如果有则将其放到current_instance中否则继续往下走
                                if let Pat::Ident(ident) = &local.pat {
                                    model
                                        .current_instance
                                        .as_mut()
                                        .unwrap()
                                        .name
                                        .replace(ident.ident.clone());
                                    model.current_instance.as_mut().unwrap().is_mut =
                                        ident.mutability.is_some();
                                    continue;
                                }
                            }
                        }
                    }
                }

                let ident = match &local.pat {
                    Pat::Ident(ident) => Some(ident.ident.to_string()),
                    Pat::Type(ty) => {
                        if let Pat::Ident(ident) = &*ty.pat {
                            Some(ident.ident.to_string())
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(ident) = ident {
                    if model.push_sub_prop_fn(&bind_fn_tree, &ident, &stmt) {
                        continue;
                    } else {
                        model.push_other(stmt.clone());
                    }
                } else {
                    model.push_other(stmt.clone());
                }
            }
            syn::Stmt::Expr(expr, _) => {
                // 对表达式进行判断，如果左侧是以instance_ptr开头的则认为是对当前实例的操作
                // 否则放到other中
                if let Expr::Assign(assign) = expr {
                    if let Expr::Field(field) = &*assign.left {
                        if let Expr::Path(path) = &*field.base {
                            if path.path.segments.first().unwrap().ident.eq(model
                                .get_current_instance()
                                .unwrap()
                                .name()
                                .unwrap())
                            {
                                if model.instance_opt.is_none() {
                                    model.instance_opt.replace(vec![]);
                                }
                                model.instance_opt.as_mut().unwrap().push(stmt.clone());
                                continue;
                            }
                        }
                    }
                }
                model.push_other(stmt.clone());
            }
        }
    }
    model.set_lifetimes(lifetimes);
    model
}

fn push_sub_prop_fn<C, F>(
    target: &mut GenScriptModel,
    bind_tree: &PropTree,
    ident: &str,
    code: &syn::Stmt,
    condition: C,
    f: F,
) -> bool
where
    C: Fn(&Value) -> &str,
    F: Fn(&mut GenScriptModel, PropFn) -> (),
{
    let mut flag = false;
    'out: for ((widget, id), prop_fn_key) in bind_tree {
        if prop_fn_key.is_some() {
            for (k, v) in prop_fn_key.as_ref().unwrap() {
                let target_ident = condition(v);
                // dbg!(target_ident, ident);
                let is_prop = if target_ident.eq(ident) {
                    false
                } else if target_ident.starts_with(ident) {
                    true
                } else {
                    continue;
                };

                let item = PropFn {
                    widget: widget.to_string(),
                    id: id.to_string(),
                    key: k.clone(),
                    ident: v.clone(),
                    code: code.clone(),
                    is_prop,
                };
                f(target, item);
                flag = true;
                break 'out;
            }
        }
    }
    flag
}
