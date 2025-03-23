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

    use crate::{compiler::Context, model::{SimpleAppMain, Widget}};


    fn context() -> Context {
        Context {
            app_main: SimpleAppMain::default(),
            // sc_visitor_chain: VisitorChain::build(),
            define_widget_poll: Default::default(),
            plugins: None,
            dyn_processor: None,
            lib_content: None,
            routers: None,
        }
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
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/tests/views/if_else.gen
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/tests",
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
