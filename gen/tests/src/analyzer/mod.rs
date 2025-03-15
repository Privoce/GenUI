#[cfg(test)]
mod test_analyzer{
    use std::time::Instant;

    use gen_analyzer::Model;
    use gen_utils::common::Source;

    fn handle(source: Source){
        let t = Instant::now();
        let model = Model::new(source, false);
        dbg!(t.elapsed());
        dbg!(&model);
    }

    #[test]
    fn parse_bind(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "components/hello.gen",
            "src_gen_0/src/components/hello.gen",
        );

        handle(source);
    }


    #[test]
    fn parse_style(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "components/styles.gen",
            "src_gen_0/src/components/styles.gen",
        );

        handle(source);
    }

    #[test]
    fn parse_template(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "components/easy.gen",
            "src_gen_0/src/components/easy.gen",
        );

        handle(source);
    }

    #[test]
    fn parse_all(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "views/home.gen",
            "src_gen_0/src/views/home.gen",
        );

        handle(source);
    }

    #[test]
    fn parse_script(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/made_with_GenUI/quick/hello",
            "views/mod.gen",
            "src_gen_0/src/views/mod.gen",
        );

        handle(source);
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

        dbg!(&model);
    }

    #[test]
    fn parse_root3() {
        let input = r#"
        <template>
            <view>
                <label text="'hello'"/>
                <button padding="12.0" />
                <view>
                    <button>
                        <label text="'world'" as_prop="slot" />
                    </button>
                </view>
            </view>
        </template>
        "#;

        let t = Instant::now();
        let model = input.parse::<Model>();
        dbg!(t.elapsed());
        dbg!(&model);
    }

    #[test]
    fn parse_root2() {
        let input = r#"
        <template>
            <view>
                <label text="'hello'"></label>
                <button padding="12.0"></button>
                <view>
                    <button>
                        <label text="'world'" as_prop="slot" />
                    </button>
                </view>
            </view>
        </template>
        "#;

        let t = Instant::now();
        let model = input.parse::<Model>();
        dbg!(t.elapsed());
        dbg!(&model);
    }
}