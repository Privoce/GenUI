/// makepad compiler
pub mod compiler;
/// makepad builtin widgets and structs
pub mod builtin;
/// makepad model, include AppMain, Widget and Virtual
pub mod model;
/// needed traits for makepad
pub mod traits;
/// makepad props from GenUI
pub mod from_gen;
/// do token stream and makepad live design
pub mod token;
pub mod visitor;
pub mod two_way_binding;
pub mod script;


#[cfg(test)]
mod test_widget {
    // use std::{path::PathBuf, str::FromStr};

    use std::path::PathBuf;

    use gen_analyzer::Model;
    use gen_utils::{
        common::{fs, Source},
        compiler::ToRs,
    };

    use crate::{compiler::{Context, RouterBuilder}, model::{SimpleAppMain, Widget}};


    fn context() -> Context {
        Context {
            app_main: SimpleAppMain::default(),
            // sc_visitor_chain: VisitorChain::build(),
            define_widget_poll: Default::default(),
            plugins: None,
            dyn_processor: None,
            lib_content: None,
            router: Some(RouterBuilder::new("/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/router/router.toml", "").unwrap()),
        }
    }

    #[test]
    fn router_called(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/router/views/home.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "router/views/home.gen",
            "src_gen_0/src/router/views/home.rs",
        );
        handle(source);
    }

    #[test]
    fn lifecycle_home(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/lifecycle/views/home.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "lifecycle/views/home.gen",
            "src_gen_0/src/lifecycle/views/home.rs",
        );
        handle(source);
    }

    #[test]
    fn lifecycle_hello(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/lifecycle/tests/components/hello.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/lifecycle/tests",
            "components/hello.gen",
            "src_gen_0/src/components/hello.rs",
        );
        handle(source);
    }

    #[test]
    fn update_lifecycle(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/views/update.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "views/update.gen",
            "src_gen_0/src/views/update.rs",
        );
        handle(source);
    }

    #[test]
    fn if_nested(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/if_nested.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/if_nested.gen",
            "src_gen_0/src/views/if_nested.rs",
        );
        handle(source);
    }

    #[test]
    fn if_all_multi(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/if_all_multi.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/if_all_multi.gen",
            "src_gen_0/src/views/if_all_multi.rs",
        );
        handle(source);
    }

    #[test]
    fn if_else_and_if_else(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/if_else_and_if_else.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/if_else_and_if_else.gen",
            "src_gen_0/src/views/if_else_and_if_else.rs",
        );
        handle(source);
    }

    #[test]
    fn computed_with_args(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/if_multi_else_if.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/if_multi_else_if.gen",
            "src_gen_0/src/views/if_multi_else_if.rs",
        );
        handle(source);
    }

    #[test]
    fn if_computed(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/if_else_if_else.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/if_else_if_else.gen",
            "src_gen_0/src/views/if_else_if_else.rs",
        );
        handle(source);
    }

    #[test]
    fn multi_if(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/multi_if.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/multi_if.gen",
            "src_gen_0/src/views/multi_if.rs",
        );
        handle(source);
    }

    #[test]
    fn ui_root(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/root.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/root.gen",
            "src_gen_0/src/views/root.rs",
        );
        handle(source);
    }

    #[test]
    fn route_macro(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/views/router_test.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "views/router_test.gen",
            "src_gen_0/src/views/router_test.rs",
        );
        handle(source);
    }

    #[test]
    fn only_if(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/home.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/home.gen",
            "src_gen_0/src/views/home.rs",
        );
        handle(source);
    }

    #[test]
    fn if_easy(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/views/if_easy.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "views/if_easy.gen",
            "src_gen_0/src/views/if_easy.rs",
        );
        handle(source);
    }

    #[test]
    fn if_else(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else/views/if_else.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/if_else_test/if_else",
            "views/if_else.gen",
            "src_gen_0/src/views/if_else.rs",
        );

        handle(source);
    }

    #[test]
    fn vis(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/views/bind1.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "views/bind1.gen",
            "src_gen_0/src/views/bind1.rs",
        );

        handle(source);
    }

    #[test]
    fn script(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/views/sc1.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "views/sc1.gen",
            "src_gen_0/src/views/sc1.rs",
        );
        // let model = Model::new(source, true).unwrap();
        // dbg!(model);
        handle(source);
    }

    #[test]
    fn script_pure(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/views/sc_pure.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
            "views/sc_pure.gen",
            "src_gen_0/src/views/sc_pure.rs",
        );
        // let model = Model::new(source, true).unwrap();
        // dbg!(model);
        handle(source);
    }

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

    
    #[test]
    fn for_loop() {
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test/fors/components/hello.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/for_test",
            "fors/components/hello.gen",
            "src_gen_0/src/components/hello.rs",
        );

        handle(source);
    }

    #[test]
    fn setting(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/others/others/views/home.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/others",
            "others/views/home.gen",
            "src_gen_0/src/views/home.rs",
        );
        handle(source);
    }

    

    
    fn handle(source: Source) {
        let model = Model::new(source, true).unwrap();
        // dbg!(model.template);
        let mut context = context();
        let w = Widget::try_from((&mut context, model)).unwrap();
        // dbg!(w.template);
        let content = w.content().unwrap().to_string();
        let path = PathBuf::from("/Users/shengyifei/projects/gen_ui/GenUI/gen/mini_test.rs");
        let _ = fs::write(path.as_path(), &content);
    }

}
