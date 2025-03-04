mod abs;
mod handler;
pub mod role;
mod template;
mod traits;

use std::sync::{Arc, RwLock};

pub use abs::*;
pub use handler::*;
pub use template::*;
pub use traits::*;

use crate::{
    compiler::Context,
    token::{import_default_all, import_draw_shader, use_crate_all, use_default_all},
};
use gen_analyzer::{Model, Polls, Script, Style, Template};
use gen_utils::{common::Source, compiler::ToRs, error::Error};
use quote::{quote, ToTokens};

#[derive(Debug, Clone)]
pub struct Widget {
    pub source: Source,
    pub is_entry: bool,
    pub template: Option<WidgetTemplate>,
    pub template_ptrs: Option<Vec<WidgetTemplate>>,
    pub script: Option<crate::script::Script>,
    pub has_plugin: bool,
}

impl Widget {
    pub fn new(context: &mut Context, model: Model) -> Result<Self, Error> {
        let is_entry = model.is_entry;
        if is_entry {
            context
                .app_main
                .root_ref
                .source
                .replace(model.special.clone());
        }

        let widget = Widget::try_from((context, model))?;
        Ok(widget)
    }
    pub fn imports(&self) -> Option<proc_macro2::TokenStream> {
        self.script.as_ref().and_then(|sc| sc.uses())
    }
    /// default script impl for easy define widget
    pub fn patch_or_default_script(&mut self) -> Result<(), Error> {
        // 确保有template
        if let Some(template) = self.template.as_ref() {
            if let Some(crate::script::Script::ScRs(patch_sc)) = self.script.as_ref() {
                if patch_sc.live_component.is_some() {
                    return Ok(());
                }
                // 说明没有进行具体的定义，但有一些其他的代码，需要patch
                self.script = template
                    .is_define_root_and(|define_widget| {
                        let mut script = define_widget.default_script();
                        script.patch(patch_sc)?;
                        Ok::<crate::script::Script, Error>(script)
                    })
                    .transpose()?;
            }
        }

        Ok(())
    }
    pub fn uses_token_stream(&self) -> proc_macro2::TokenStream {
        let mut tk = use_default_all();
        if self.has_plugin {
            tk.extend(use_crate_all());
        }
        tk
    }
    /// 获取script的token_stream
    pub fn script_token_stream(&self) -> Option<proc_macro2::TokenStream> {
        self.script.as_ref().map(|sc| sc.to_token_stream())
    }

    pub fn is_empty(&self) -> bool {
        self.template.is_none() && self.script.is_none()
    }

    pub fn is_global(&self) -> bool {
        if let Some(template) = self.template.as_ref() {
            template.is_global()
        } else {
            false
        }
    }
}

impl TryFrom<(&mut Context, Model)> for Widget {
    type Error = Error;

    fn try_from(value: (&mut Context, Model)) -> Result<Self, Self::Error> {
        let (context, model) = value;
        // [分析Model策略] ------------------------------------------------------------------------------------
        let Model {
            special,
            template,
            script,
            style,
            is_entry,
            strategy,
            polls,
            ..
        } = model;

        // [handle commons] ----------------------------------------------------------------------------------

        let widget = match strategy {
            gen_analyzer::Strategy::SingleStyle => (special, style, is_entry).try_into(),
            gen_analyzer::Strategy::SingleTemplate => (special, template, is_entry).try_into(),
            gen_analyzer::Strategy::SingleScript => (special, script, is_entry).try_into(),
            gen_analyzer::Strategy::TemplateScript => {
                (context, special, template, script, is_entry, polls).try_into()
            }
            gen_analyzer::Strategy::TemplateStyle => {
                (special, template, style, is_entry).try_into()
            }
            gen_analyzer::Strategy::All => {
                (context, special, template, script, style, is_entry, polls).try_into()
            }
            gen_analyzer::Strategy::None => (special, is_entry).try_into(), // means no strategy, just a empty file
            _ => panic!("can not reach here"),
        }?;

        Ok(widget)
    }
}

/// 解析空文件
impl TryFrom<(Source, bool)> for Widget {
    type Error = Error;

    fn try_from(value: (Source, bool)) -> Result<Self, Self::Error> {
        let (source, is_entry) = value;
        Ok(Widget {
            source,
            is_entry,
            template: None,
            script: None,
            has_plugin: false,
            template_ptrs: None,
        })
    }
}

/// 解析单style模版
/// 处理只有单个<style>标签的情况, 这种情况需要将style转为Makepad的Global Prop即可
impl TryFrom<(Source, Option<Style>, bool)> for Widget {
    type Error = Error;

    fn try_from(value: (Source, Option<Style>, bool)) -> Result<Self, Self::Error> {
        handler::single_style(value.0, value.1, value.2)
    }
}

/// 解析单template模版
impl TryFrom<(Source, Option<Template>, bool)> for Widget {
    type Error = Error;

    fn try_from(value: (Source, Option<Template>, bool)) -> Result<Self, Self::Error> {
        handler::single_template(value.0, value.1, value.2)
    }
}

/// 解析单script模版
/// 处理只有单个<script>标签的情况,
impl TryFrom<(Source, Option<Script>, bool)> for Widget {
    type Error = Error;

    fn try_from(value: (Source, Option<Script>, bool)) -> Result<Self, Self::Error> {
        handler::single_script(value.0, value.1, value.2)
    }
}

/// 解析template + style模版
impl TryFrom<(Source, Option<Template>, Option<Style>, bool)> for Widget {
    type Error = Error;

    fn try_from(
        value: (Source, Option<Template>, Option<Style>, bool),
    ) -> Result<Self, Self::Error> {
        handler::template_style(value.0, value.1, value.2, value.3)
    }
}

/// 解析template + script模版
impl
    TryFrom<(
        &mut Context,
        Source,
        Option<Template>,
        Option<Script>,
        bool,
        Arc<RwLock<Polls>>,
    )> for Widget
{
    type Error = Error;

    fn try_from(
        value: (
            &mut Context,
            Source,
            Option<Template>,
            Option<Script>,
            bool,
            Arc<RwLock<Polls>>,
        ),
    ) -> Result<Self, Self::Error> {
        handler::template_script(value.0, value.1, value.2, value.3, value.4, value.5)
    }
}

/// 解析template + script + style模版
impl
    TryFrom<(
        &mut Context,
        Source,
        Option<Template>,
        Option<Script>,
        Option<Style>,
        bool,
        Arc<RwLock<Polls>>,
    )> for Widget
{
    type Error = Error;

    fn try_from(
        value: (
            &mut Context,
            Source,
            Option<Template>,
            Option<Script>,
            Option<Style>,
            bool,
            Arc<RwLock<Polls>>,
        ),
    ) -> Result<Self, Self::Error> {
        handler::all(
            value.0, value.1, value.2, value.3, value.4, value.5, value.6,
        )
    }
}

// 实现最终的ToRs，将Widget最终能够输出为rs文件
impl ToRs for Widget {
    fn source(&self) -> Option<&Source> {
        Some(&self.source)
    }

    fn content(&self) -> Result<proc_macro2::TokenStream, Error> {
        let mut tokens = proc_macro2::TokenStream::new();
        // [如果是空文件, 直接返回] ----------------------------------------------------------------------------
        if self.is_empty() {
            return Ok(quote! {});
        }

        // [template] ---------------------------------------------------------------------------------------
        let template = if let Some(template) = self.template.as_ref() {
            // [引入依赖] -------------------------------------------------------------------------------------
            let uses = self.uses_token_stream();
            // [引入Makepad的全局依赖] -------------------------------------------------------------------------
            let mut imports = if self.is_global() {
                import_draw_shader()
            } else {
                import_default_all()
            };
            let component_imports = self.imports();
            if let Some(tk) = component_imports.as_ref() {
                imports.extend(tk.clone());
            }

            let template = template.to_token_stream(self.template_ptrs.as_ref())?;
            let pub_sign = if template.is_empty() {
                None
            } else {
                Some(quote! {pub})
            };

            Some(quote! {
                #uses
                #component_imports
                live_design!{
                    #imports
                    #pub_sign #template
                }
            })
        } else {
            None
        };
        // [script] -----------------------------------------------------------------------------------------
        let script = self.script_token_stream();
        // [合并] --------------------------------------------------------------------------------------------
        tokens.extend(quote! {
            #template
            #script
        });

        Ok(tokens)
    }
}

#[cfg(test)]
mod test_widget {
    // use std::{path::PathBuf, str::FromStr};

    use std::path::PathBuf;

    use gen_analyzer::Model;
    use gen_utils::{
        common::{fs, Source},
        compiler::ToRs,
    };
    use quote::ToTokens;
    // use rssyin::{makepad::MakepadChainExpand, visitor::chain::VisitorChain};

    use crate::{compiler::Context, model::SimpleAppMain};

    use super::Widget;
    fn context() -> Context {
        Context {
            app_main: SimpleAppMain::default(),
            // sc_visitor_chain: VisitorChain::build(),
            define_widget_poll: Default::default(),
            plugins: None,
            dyn_processor: None,
            lib_content: None,
        }
    }

    #[test]
    fn root() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quickstart/hello/views/root.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quickstart",
            "hello/views/root.gen",
            "src_gen_0/src/views/root.rs",
        );
        handle(source);
    }

    #[test]
    fn call_define() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quickstart/hello/views/home.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quickstart",
            "hello/views/home.gen",
            "src_gen_0/src/views/home.rs",
        );
        handle(source);
    }

    #[test]
    fn nested_for() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test",
            "fors/components/easy.gen",
            "src_gen_0/src/components/easy.rs",
        );

        handle(source);
    }

    // /Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test/fors/views/home.gen
    #[test]
    fn for_test() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test",
            "fors/views/home.gen",
            "src_gen_0/src/views/home.rs",
        );

        handle(source);
    }

    // /Users/shengyifei/projects/gen_ui/made_with_GenUI/start/start/components/easy.gen
    #[test]
    fn start() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick_start/quick_start",
            "components/easy.gen",
            "src_gen_0/src/components/easy.rs",
        );

        handle(source);
    }

    // /Users/shengyifei/projects/gen_ui/made_with_GenUI/c_ref_test/c_ref/components/header.gen
    #[test]
    fn c_ref2() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/c_ref_test",
            "c_ref/components/header.gen",
            "src_gen_0/src/components/header.rs",
        );

        handle(source);
    }

    // /Users/shengyifei/projects/gen_ui/made_with_GenUI/c_ref_test/c_ref/views/root.gen
    #[test]
    fn c_ref() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/c_ref_test",
            "c_ref/views/hello.gen",
            "src_gen_0/src/views/hello.rs",
        );

        handle(source);
    }

    // /Users/shengyifei/projects/gen_ui/made_with_GenUI/todo/others/t1/views/two_way.gen
    #[test]
    fn todo3() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/todo/others",
            "t1/views/two_way.gen",
            "src_gen_0/src/views/two_way.rs",
        );

        handle(source);
    }

    #[test]
    fn todo2() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/todo/todo_front",
            "todo/views/header.gen",
            "src_gen_0/src/views/header.rs",
        );

        handle(source);
    }

    #[test]
    fn todo() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/todo/todo_front",
            "todo/views/todo_list.gen",
            "src_gen_0/src/views/todo_list.rs",
        );

        handle(source);
    }

    #[test]
    fn for_loop() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1",
            "hello/views/for.gen",
            "src_gen_0/src/views/for.rs",
        );

        handle(source);
    }

    #[test]
    fn multi_fns() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1",
            "hello/views/multi_fns.gen",
            "src_gen_0/src/views/multi_fns.rs",
        );

        handle(source);
    }

    #[test]
    fn root_script() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello/views/root.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick",
            "hello/views/root.gen",
            "src_gen_0/src/views/root.rs",
        );

        handle(source);
    }
    #[test]
    fn template_script_ref() {
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1",
            "hello/views/hello5.gen",
            "src_gen_0/src/views/hello5.rs",
        );

        handle(source);
    }

    #[test]
    fn template_script() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello/components/template_sc.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick",
            "hello/components/template_sc.gen",
            "src_gen_0/src/components/template_sc.rs",
        );

        handle(source);
    }

    #[test]
    fn template_style() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello/components/template_style.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick",
            "hello/components/template_style.gen",
            "src_gen_0/src/components/template_style.rs",
        );

        handle(source);
    }

    #[test]
    fn single_script() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello/components/single_sc.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick",
            "hello/components/single_sc.gen",
            "src_gen_0/src/components/single_sc.rs",
        );

        handle(source);
    }

    #[test]
    fn define_template() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello/components/define_component.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick",
            "hello/components/define_component.gen",
            "src_gen_0/src/components/define_component.rs",
        );

        handle(source);
    }

    // #[test]
    // fn empty() {
    //     let source =
    //         PathBuf::from_str("/Users/shengyifei/projects/gen_ui/GenUI/examples/new_gen").unwrap();
    //     let file_path = source.join("scripts/empty.gen");
    //     let model = Model::new(&file_path, &source, false);
    //     let w = Widget::try_from(model.unwrap()).unwrap();
    //     let content = ToRs::content(&w).unwrap().to_string();
    //     let target = source.join("scripts/result/empty.rs");
    //     std::fs::write(target, content).unwrap();
    // }
    #[test]
    fn single_template() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello/components/easy.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick",
            "hello/components/easy.gen",
            "src_gen_0/src/components/easy.rs",
        );

        handle(source);
    }

    fn handle(source: Source) {
        let model = Model::new(source, true).unwrap();
        let mut context = context();
        let w = Widget::try_from((&mut context, model)).unwrap();
        // dbg!(&w.template);
        let content = w.content().unwrap().to_string();
        let path = PathBuf::from("/Users/shengyifei/projects/gen_ui/GenUI/gen/generator/makepad/src/compiler/conf/mini_test.rs");
        let _ = fs::write(path.as_path(), &content);
    }

    // #[test]
    // fn single_style() {
    //     let source =
    //         PathBuf::from_str("/Users/shengyifei/projects/gen_ui/GenUI/examples/new_gen").unwrap();
    //     let file_path = source.join("scripts/style_theme.gen");
    //     let model = Model::new(&file_path, &source, false);
    //     // dbg!(&model);
    //     let w = Widget::try_from(model.unwrap()).unwrap();
    //     // write to new_gen/scripts/result/style_theme.rs
    //     let content = ToRs::content(&w).unwrap().to_string();

    //     let target = source.join("scripts/result/style_theme.rs");
    //     std::fs::write(target, content).unwrap();
    // }
}
