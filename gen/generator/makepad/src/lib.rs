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
        // dbg!(model);
        let mut context = context();
        let w = Widget::try_from((&mut context, model)).unwrap();
        // dbg!(&w.template);
        let content = w.content().unwrap().to_string();
        let path = PathBuf::from("/Users/shengyifei/projects/gen_ui/GenUI/gen/mini_test.rs");
        let _ = fs::write(path.as_path(), &content);
    }

}
