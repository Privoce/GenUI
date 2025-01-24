#[cfg(test)]
mod test_converter{
    use gen_converter::Model;
    use gen_utils::common::Source;

    #[test]
    fn test1(){
        let source = Source::new(
            "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1",
            "hello/views/multi_fns.gen",
            "src_gen_0/src/views/multi_fns.rs",
        );

        let model = Model::new(source, false).unwrap();
        println!("{:?}", model);
    }
}