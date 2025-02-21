//! 使用ra_ap_syntax进行rust的语法分析和替换
mod utils;

use std::str::FromStr;

use proc_macro2::TokenStream;
use ra_ap_syntax::{
    ast::{self, HasAttrs},
    AstNode, Edition, SourceFile, TextSize,
};
use syn::{parse_str, ItemEnum, ItemImpl, ItemStruct};
use utils::*;

use crate::{
    bridger::{Imports, ScriptBridger},
    error::Error,
};

/// # 脚本分析追踪器
/// 用于追踪脚本的分析过程，由于我们无法确定使用者是否按照顺序先定义属性再处理事件...，所以我们需要追踪整个过程
/// 这样可以自由调节那些代码需要延迟到某些代码处理完之后再进行
/// ## 需要进行的追踪
/// 1. 使用`#[prop]`宏定义的组件属性，它是整个组件的实例化的基础
/// 2. 使用`Default` trait定义的组件属性的impl部分，它是组件属性的默认值
/// 3. 使用`#[event]`宏定义的组件事件，它是组件的事件回调
/// 4. 使用`impl`定义的组件事件的impl部分，它是当前组件内部子组件的事件回调的具体实现或当前组件提供到外部的方法
///     1. 对于内部组件事件回调或外部方法，没有什么特殊的形式
///     2. impl部分还包括组件的生命周期方法，这些方法会使用`#[before_create]`, `#[created]`等宏进行定义
/// ## 不需要进行的追踪
/// 除了上述的追踪外，其他部分无需进行追踪
/// ## 为什么需要追踪
/// 1. rssyin是一个中间件，它需要分析出GenUI特殊rust代码部分并最终产生一个ScriptBridger对象
/// 2. ScriptBridger对象是一个中间对象，它存储了所有进行追踪的代码并最终在生成层转换为makepad代码，这些追踪的代码是需要在生成层进一步处理的
/// 3. ScriptBridger也会存储不需要进行追踪的代码，这部分会直接复制到makepad代码中
/// 4. 为了保证转换的正确性，我们需要追踪整个过程，确保所有的代码都被正确的转换
/// ## 脚本示例
/// ```
/// // 使用prop宏构建了组件的属性
/// #[prop]
/// struct A{
///    a: String
/// }
///
/// // 组件初始化时属性的默认值
/// impl Default for A{
///    fn default(){
///        A{a: "Hello".to_string();}
///    }
/// }
///
/// impl A{
///    // 组件中可调用的事件回调
///    fn click_btn(&self){
///      print!("clicked");
///    }
///    #[before_create]
///    fn before_create(&self){
///       print!("before create");
///    }
/// }
///
/// // 使用event宏定义了组件的事件，在外部可以由其他组件调用组件的@clicked事件
/// #[event]
/// pub enum AEvent{
///    Clicked,
/// }
///
/// fn other(){
///     // 其他代码
/// }
/// ```
pub struct ScriptAnalyzer;

impl ScriptAnalyzer {
    /// 对rust代码进行分析处理
    pub fn analyze(code: &str) -> Result<ScriptBridger, Error> {
        let source_file = SourceFile::parse(code, Edition::Edition2021).tree();

        let mut start_index = TextSize::new(0);
        // let end_index = source_file.syntax().text_range().end();
        let mut import_macro = None;
        let mut prop_struct = None;
        let mut event_enum = None;
        let mut default_impl = None;
        let mut impl_prop = None;
        let mut others = vec![];
        let mut lazy: Option<Lazy> = None;

        for node in source_file.syntax().descendants() {
            // [count start index if bigger than node end index do continue] -----------------------------------
            if start_index >= node.text_range().end() {
                continue;
            }

            // [if is space do continue] -----------------------------------------------------------------------
            match node.kind() {
                ra_ap_syntax::SyntaxKind::WHITESPACE => {
                    start_index = node.text_range().end();
                    continue;
                }
                ra_ap_syntax::SyntaxKind::SOURCE_FILE => {
                    continue;
                }
                _ => {}
            }

            // [import!] ----------------------------------------------------------------------------------------
            if let Some(macro_call) = ast::MacroCall::cast(node.clone()) {
                let is_import = macro_call
                    .path()
                    .map(|path| "import".is_path_segment(&path))
                    .unwrap_or_default();
                if is_import {
                    if let Some(tree) = macro_call.token_tree() {
                        import_macro.replace(Imports::from_str(&tree.to_string())?);
                    }
                    // 记录结束位置
                    start_index = macro_call.syntax().text_range().end();
                    continue;
                }
            }
            // [prop macro] -------------------------------------------------------------------------------------
            if let Some(strt) = ast::Struct::cast(node.clone()) {
                let is_prop = strt.attrs().any(|attr| {
                    attr.path()
                        .map(|path| "prop".is_path_segment(&path))
                        .unwrap_or_default()
                });

                if is_prop {
                    let item_struct = parse_str::<ItemStruct>(&strt.syntax().text().to_string())
                        .map_err(|e| Error::Parse(e))?;
                    let prop_ident = item_struct.ident.to_string();
                    prop_struct.replace(item_struct);
                    start_index = strt.syntax().text_range().end();
                    // [if lazy exists do analyze] --------------------------------------------------------------
                    if let Some(lazy) = lazy.as_mut() {
                        lazy.prop_ident.replace(prop_ident);
                        let lazy_res = lazy.analyze()?;
                        // [set default impl if exists] ---------------------------------------------------------
                        lazy_res.default_impl.map(|item_impl| {
                            default_impl.replace(item_impl);
                        });
                        // [set impl prop if exists] ------------------------------------------------------------
                        lazy_res.impl_prop.map(|item_impl| {
                            impl_prop.replace(item_impl);
                        });
                        // [extend others] ----------------------------------------------------------------------
                        others.extend(lazy_res.others);
                    }
                    continue;
                }
            }
            // [event macro] ------------------------------------------------------------------------------------
            if let Some(enm) = ast::Enum::cast(node.clone()) {
                let is_event = enm.attrs().any(|attr| {
                    attr.path()
                        .map(|path| "event".is_path_segment(&path))
                        .unwrap_or_default()
                });

                if is_event {
                    event_enum.replace(
                        parse_str::<ItemEnum>(&enm.syntax().text().to_string())
                            .map_err(|e| Error::Parse(e))?,
                    );
                    start_index = enm.syntax().text_range().end();
                    continue;
                }
            }
            // [default impl or impl] ----------------------------------------------------------------------------
            if let Some(impl_block) = ast::Impl::cast(node.clone()) {
                start_index = impl_block.syntax().text_range().end();
                if let Some(prop_struct) = prop_struct.as_ref() {
                    let prop_ident = prop_struct.ident.to_string();

                    // if prop ident == impl_block self ty
                    let is_prop_impl = impl_block
                        .self_ty()
                        .map(|self_ty| prop_ident.is_self_type(self_ty))
                        .unwrap_or_default();

                    if let Some(t) = impl_block.trait_() {
                        if "Default".is_trait(t) && is_prop_impl {
                            default_impl.replace(
                                parse_str::<ItemImpl>(&impl_block.syntax().text().to_string())
                                    .map_err(|e| Error::Parse(e))?,
                            );
                        }
                    } else {
                        // no trait
                        if is_prop_impl {
                            impl_prop.replace(
                                parse_str::<ItemImpl>(&impl_block.syntax().text().to_string())
                                    .map_err(|e| Error::Parse(e))?,
                            );
                        } else {
                            // set into lazy
                            lazy.get_or_insert(Lazy::default()).impls.push(impl_block);
                        }
                    }
                } else {
                    // 这个说明在检测到impl Default for xxx, 但是没有检测到#[prop] xxx无法确定impl的目标
                    // 暂时把这部分代码放到lazy中, 等到检测到#[prop] xxx时再进行处理
                    lazy.get_or_insert(Lazy::default())
                        .default_impls
                        .push(impl_block);
                }

                continue;
            }

            // [others] -----------------------------------------------------------------------------------------
            // 由于ra_ap_syntax遍历node时会一层一层向里面遍历, 所以还是需要记录下start_index避免递归
            others.push(
                parse_str::<syn::Stmt>(&node.text().to_string()).map_err(|e| Error::Parse(e))?,
            );
            start_index = node.text_range().end();
        }

        Ok(ScriptBridger {
            imports: import_macro,
            prop: prop_struct,
            instance: default_impl,
            event: event_enum,
            impl_prop,
            others,
        })
    }
}

#[derive(Debug, Default)]
struct Lazy {
    default_impls: Vec<ast::Impl>,
    impls: Vec<ast::Impl>,
    prop_ident: Option<String>,
}

impl Lazy {
    pub fn analyze(&self) -> Result<LazyAnalyzeResult, Error> {
        let handle = |impls: &Vec<ast::Impl>,
                      prop_ident: &Option<String>,
                      target: &mut Option<ItemImpl>,
                      others: &mut Vec<syn::Stmt>|
         -> Result<(), Error> {
            for impl_block in impls {
                if let Some(self_ty) = impl_block.self_ty() {
                    let is_prop_impl = prop_ident
                        .as_ref()
                        .map_or(false, |prop_ident| prop_ident.is_self_type(self_ty));

                    if is_prop_impl {
                        target.replace(
                            parse_str::<syn::ItemImpl>(&impl_block.syntax().text().to_string())
                                .map_err(|e| Error::Parse(e))?,
                        );
                    } else {
                        others.push(
                            parse_str::<syn::Stmt>(&impl_block.syntax().text().to_string())
                                .map_err(|e| Error::Parse(e))?,
                        );
                    }
                }
            }
            Ok(())
        };

        let mut impl_default_prop = None;
        let mut impl_prop = None;
        let mut others = vec![];

        let Lazy {
            default_impls,
            impls,
            prop_ident,
        } = self;

        // [default impls] -----------------------------------------------------------------------------------
        // find default impls which self_ty is prop_ident
        handle(
            default_impls,
            prop_ident,
            &mut impl_default_prop,
            &mut others,
        )?;
        // [impls] -------------------------------------------------------------------------------------------
        handle(impls, prop_ident, &mut impl_prop, &mut others)?;

        Ok(LazyAnalyzeResult {
            default_impl: impl_default_prop,
            impl_prop,
            others,
        })
    }
}

struct LazyAnalyzeResult {
    default_impl: Option<syn::ItemImpl>,
    impl_prop: Option<syn::ItemImpl>,
    others: Vec<syn::Stmt>,
}

#[cfg(test)]
mod test_analyzer {
    use quote::ToTokens;

    #[test]
    fn test() {
        let input = r#"
        import!{
            crate::views::a::*;
        }

        impl Default for A{
            fn default(){
                A{a: "Hello".to_string()}
            }
        }

        #[prop]
        struct A{
            a: String
        }

        #[event]
        pub enum AEvent{
            Clicked,
        }

        fn other(){
            print!("other");
        }

        impl A{
            fn click_btn(&self){
                print!("clicked");
            }
            #[before_create]
            fn before_create(&self){
                print!("before create");
            }
        }
        "#;

        let res = super::ScriptAnalyzer::analyze(input).unwrap();
        dbg!(&res.to_token_stream().to_string());
    }
    #[test]
    fn test2() {
        let input = r#"
        {
            impl Default for A{
                fn default(){
                    A{a: "Hello".to_string()}
                }
            }
        }
        "#;

        let block = syn::parse_str::<syn::Block>(input);
        dbg!(&block);
    }
}
