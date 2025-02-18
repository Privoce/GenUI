mod model;
mod parse;
mod macros;

pub use model::*;
pub use parse::*;

#[cfg(test)]
mod test_analyzer{
    use std::time::Instant;

    use gen_utils::common::Source;

    use crate::Model;

    #[test]
    fn parse_template(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "components/easy.gen",
            "src_gen_0/src/components/easy.gen",
        );

        let model = Model::new(source, false);

        dbg!(model);
    }

    #[test]
    fn parse_all(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "views/home.gen",
            "src_gen_0/src/views/home.gen",
        );

        let model = Model::new(source, false);

        dbg!(model);
    }

    #[test]
    fn parse_script(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "views/mod.gen",
            "src_gen_0/src/views/mod.gen",
        );

        let t = Instant::now();
        let model = Model::new(source, false);
        dbg!(t.elapsed());
        dbg!(model);
    }

    #[test]
    fn parse_root(){
        // /Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello/views/root.gen

        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "views/root.gen",
            "src_gen_0/src/views/root.gen",
        );

        let model = Model::new(source, false);

        dbg!(model);
    }
}