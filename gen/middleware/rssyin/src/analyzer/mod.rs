//! 使用ra_ap_syntax进行rust的语法分析和替换
mod utils;

use std::str::FromStr;

use proc_macro2::TokenStream;
use ra_ap_syntax::{
    ast::{self, HasAttrs}, AstNode, Edition, RustLanguage, SourceFile, SyntaxNode, TextSize, WalkEvent
};
use syn::{parse_str, ItemEnum, ItemImpl, ItemStruct};
use utils::*;

use crate::{
    bridger::{Import, Imports},
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
struct ScriptAnalyzer {
    // bridger: ScriptBridger,
    // current_impl_target: Option<String>,
}

impl ScriptAnalyzer {
    /// 对rust代码进行分析处理
    pub fn analyze(code: &str) -> Result<(), Error> {
       



        let source_file = SourceFile::parse(code, Edition::Edition2021).tree();

        let mut start_index = TextSize::new(0);
        let end_index = source_file.syntax().text_range().end();
        let mut import_macro = None;
        let mut prop_struct = None;
        let mut event_enum = None;
        let mut default_impl = None;
        // let mut impl_prop = None;
        // let mut others = None;
        let mut lazy: Option<Lazy> = None;

        for node in source_file.syntax().descendants() {
            // [count start index if bigger than node end index do continue] -----------------------------------
            if start_index > node.text_range().end() {
                continue;
            }
            // [if is space do continue] -----------------------------------------------------------------------
            if node.kind() == ra_ap_syntax::SyntaxKind::WHITESPACE {
                start_index = node.text_range().end();
                continue;
            }

            // [import!] ----------------------------------------------------------------------------------------
            if let Some(macro_call) = ast::MacroCall::cast(node.clone()) {
                let is_import = macro_call
                    .path()
                    .map(|path| "import".is_path_segment(&path))
                    .unwrap_or_default();

                if is_import {
                    macro_call.token_tree().map(|tree| {
                        import_macro.replace(Imports::from_str(&tree.to_string()));
                    });
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
                    prop_struct.replace(
                        parse_str::<ItemStruct>(&strt.syntax().text().to_string())
                            .map_err(|e| Error::Parse(e))?,
                    );
                    start_index = strt.syntax().text_range().end();
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
            // [default impl] -----------------------------------------------------------------------------------
            if let Some(impl_block) = ast::Impl::cast(node.clone()) {
                let is_default_impl = impl_block
                    .trait_()
                    .map(|t| "Default".is_trait(t))
                    .unwrap_or_default();

                if let Some(prop_struct) = prop_struct.as_ref() {
                    let prop_ident = prop_struct.ident.to_string();

                    // if prop ident == impl_block self ty
                    let is_prop_impl = impl_block
                        .self_ty()
                        .map(|self_ty| prop_ident.is_self_type(self_ty))
                        .unwrap_or_default();

                    if is_default_impl && is_prop_impl {
                        default_impl.replace(
                            parse_str::<ItemImpl>(&impl_block.syntax().text().to_string())
                                .map_err(|e| Error::Parse(e))?,
                        );
                        start_index = impl_block.syntax().text_range().end();
                        continue;
                    }
                } else {
                    // 这个说明在检测到impl Default for xxx, 但是没有检测到#[prop] xxx无法确定impl的目标
                    // 暂时把这部分代码放到lazy中, 等到检测到#[prop] xxx时再进行处理
                    
                }
            }
        }

        dbg!(default_impl);
        Ok(())
    }
}


struct Lazy{
    default_impls: Vec<ast::Impl>,
    impls: Vec<ast::Impl>,
    prop_ident: String,  
}

impl Lazy {
    pub fn analyze(&self) -> Result<(), Error> {

        let mut impl_default_prop = None;

        let Lazy{
            default_impls,
            impls,
            prop_ident,
        } = self;

        // [default impls] -----------------------------------------------------------------------------------
        // find default impls which self_ty is prop_ident
        for default_impl in default_impls {
            if let Some(self_ty) = default_impl.self_ty() {
                let is_prop_impl = prop_ident.is_self_type(self_ty);

                if is_prop_impl {
                    // do something
                    impl_default_prop.replace(parse_str::<syn::ItemImpl>(&default_impl.syntax().text().to_string()).map_err(|e| Error::Parse(e))?);
                    break;
                }
            }
        }




        // [impls] -------------------------------------------------------------------------------------------



        Ok(())
    }
}


struct AfterLazyAnalyze{
    default_impl: Option<syn::ItemImpl>,
    others: TokenStream
}

#[cfg(test)]
mod test_analyzer {
    #[test]
    fn test() {
        let input = r#"
        import!{
            crate::views::a::*;
        }

        #[prop]
        struct A{
            a: String
        }

        impl Default for A{
            fn default(){
                A{a: "Hello".to_string()}
            }
        }
        
        "#;

        super::ScriptAnalyzer::analyze(input);
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
        dbg!(block);
    }
}
// impl Default for A{
//     fn default(){
//         A{a: "Hello".to_string()}
//     }
// }

// impl A{
//     fn click_btn(&self){
//         print!("clicked");
//     }
//     #[before_create]
//     fn before_create(&self){
//         print!("before create");
//     }
// }

// #[event]
// pub enum AEvent{
//     Clicked,
// }

// fn other(){
//     print!("other");
// }

// #[prop]
// struct A{
//     a: String
// }

// #[event]
// pub enum AEvent{
//     Clicked,
// }
