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
    fn setting(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/setting_page/start/views/home.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/setting_page",
            "start/views/home.gen",
            "src_gen_0/src/views/home.rs",
        );
        handle(source);
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
    fn fors(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test/fors/components/hello.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test",
            "fors/components/hello.gen",
            "src_gen_0/src/components/hello.rs",
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
    fn nested_wrapper_for(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test/fors/views/nested.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test",
            "fors/views/nested.gen",
            "src_gen_0/src/views/nested.rs",
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
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/todo_front/todo/views/header.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/todo_front",
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
        // dbg!(model);
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
