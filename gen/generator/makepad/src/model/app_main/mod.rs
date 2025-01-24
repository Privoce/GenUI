use std::{collections::HashSet, path::PathBuf};

use gen_utils::{
    common::{ident, Source},
    compiler::ToRs,
    error::{CompilerError, Error},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use root::RootRef;
use syn::parse_str;

use crate::{
    builtin::widget::RootConf,
    compiler::Context,
    token::{import_default, use_makepad_widgets},
};

use super::{
    traits::{AppMainTrait, LiveHookTrait, MatchEventTrait},
    WidgetTemplate,
};

mod root;
mod simple;

pub use simple::*;

/// # Makepad的app main
/// 这个结构体用于处理app main的相关信息，这个app main对于原本的makepad项目来说很不相同
/// 在GenUI中它是直接生成的，使用者无需像编写widget一样编写界面代码，他依靠GenUI SFP编译器提供的外部接口来生成(main.rs)
/// 这个结构体在Makepad中会是最后才生成的，因为它需要依赖于编译器解析其他组件的信息来产生
#[derive(Debug, Clone)]
pub struct AppMain {
    /// app main的源文件地址及编译后的地址
    pub source: Source,
    /// app main的根引用
    pub root_ref: RootRef,
    /// dsl-template
    pub template: WidgetTemplate,
    /// 注册插件｜组件, 需要依赖编译器的提供
    pub registers: Option<HashSet<String>>,
    /// app main的live hook，在GenUI项目中一般来说不会有，它直接会以derive宏的形式标注在AppMain实例上
    pub live_hook: Option<LiveHookTrait>,
    /// app main的AppMain Trait
    pub app_main: AppMainTrait,
    pub match_event: MatchEventTrait,
}

impl AppMain {
    pub fn new(context: &mut Context, source: Source, conf: &RootConf) -> Result<Self, Error> {
        if !context.app_main.has_root() {
            return Err(CompilerError::runtime(
                "Makepad Compiler - AppMain",
                "AppMain must have a root widget",
            )
            .into());
        }

        Ok(Self {
            source,
            root_ref: context.app_main.root_ref.clone(),
            template: (context.app_main.root_ref.name(), conf).into(),
            registers: None,
            live_hook: None,
            app_main: AppMainTrait::default(),
            match_event: MatchEventTrait::default(),
        })
    }

    pub fn source_from_entry(entry: Option<&String>, source: &Source) -> Source {
        let app_main_path = source
            .to_path()
            .join("src")
            .join(entry.unwrap_or(&String::from("app")))
            .with_extension("rs");

        Source {
            path: source.path.to_path_buf(),
            from: PathBuf::new(),
            to: app_main_path,
        }
    }

    pub fn register_token_stream(&self) -> TokenStream {
        let registers = self.registers.as_ref().map(|registers| {
            registers.iter().fold(TokenStream::new(), |mut tk, reg| {
                let reg = parse_str::<TokenStream>(reg).unwrap();

                tk.extend(quote! {
                    crate::#reg::live_design(cx);
                });

                tk
            })
        });

        quote! {
            impl LiveRegister for App {
                fn live_register(cx: &mut Cx) {
                    crate::makepad_widgets::live_design(cx);
                    crate::gen_components::live_design(cx, None);
                    #registers
                }
            }
        }
    }
}

impl ToRs for AppMain {
    fn content(&self) -> Result<proc_macro2::TokenStream, gen_utils::error::Error> {
        let mut tk = TokenStream::new();
        // [uses] -----------------------------------------------------------------------------------
        let uses = use_makepad_widgets();
        // [imports] --------------------------------------------------------------------------------
        let imports = import_default();
        let root_import = self.root_ref.import_root();
        // [root] -----------------------------------------------------------------------------------
        // let root = parse_str::<TokenStream>(&snake_to_camel(self.root_ref.name.as_ref().unwrap()))
        //     .unwrap();
        // let root = ToTokensExt::to_token_stream(&self.template)?;
        let root = self.template.to_token_stream(None)?;
        // [live hook] ------------------------------------------------------------------------------
        let (live_hook, hook_trait) = if let Some(live_hook) = self.live_hook.as_ref() {
            let hook_trait = quote! {
                #[derive(Live)]
            };
            (
                Some(live_hook.to_token_stream(ident("App").to_token_stream())),
                hook_trait,
            )
        } else {
            let hook_trait = quote! {
                #[derive(Live, LiveHook)]
            };
            (None, hook_trait)
        };
        // [registers] -------------------------------------------------------------------------------
        let registers = self.register_token_stream();
        // [match event] -----------------------------------------------------------------------------
        let match_event = self.match_event.to_token_stream();
        // [app main] --------------------------------------------------------------------------------
        let app_main_trait = self.app_main.to_token_stream();

        tk.extend(quote! {
            #uses

            live_design!{
                #imports
                #root_import
                App = {{App}}{
                    root: #root
                }
            }

            #hook_trait
            pub struct App {
                #[live]
                root: WidgetRef,
            }

            #app_main_trait
            #registers
            #match_event
            #live_hook

            app_main!(App);
        });

        Ok(tk)
    }

    fn source(&self) -> Option<&Source> {
        Some(&self.source)
    }
}
