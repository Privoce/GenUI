#[cfg(test)]
mod test_func {
    use gen_parser::Function;

    #[test]
    fn template() {
        let easy = r#"easy()"#;
        let easy_param_single = r#"easy(1)"#;
        let easy_param_multi = r#"easy(1, #fff)"#;
        let easy_s = r#"easy_s(1)"#;
        let easy_with_bind = r#"easy_s(bind1)"#;
        let easy_string_param = r#"easy_s(bind1,"test_hello")"#;

        let easy = Function::parse(easy, false).unwrap();
        let easy_param_single = Function::parse(easy_param_single, false).unwrap();
        let easy_param_multi = Function::parse(easy_param_multi, false).unwrap();
        let easy_s = Function::parse(easy_s, false).unwrap();
        let easy_with_bind = Function::parse(easy_with_bind, false).unwrap();
        let easy_string_param = Function::parse(easy_string_param, false).unwrap();

        dbg!(easy.to_string());
        dbg!(easy_param_single.to_string());
        dbg!(easy_param_multi.to_string());
        dbg!(easy_s.to_string());
        dbg!(easy_with_bind.to_string());
        dbg!(easy_string_param.to_string());
    }

    #[test]
    fn style() {
        let easy = r#"easy()"#;
        let easy_param_single = r#"easy(1)"#;
        let easy_param_multi = r#"easy(1, #fff)"#;
        let easy_s = r#"easy_s(1)"#;
        let easy_with_bind = r#"easy_s($bind1)"#;
        let easy_string_param = r#"easy_s($bind1,"test_hello")"#;

        let easy = Function::parse(easy, true).unwrap();
        let easy_param_single = Function::parse(easy_param_single, true).unwrap();
        let easy_param_multi = Function::parse(easy_param_multi, true).unwrap();
        let easy_s = Function::parse(easy_s, true).unwrap();
        let easy_with_bind = Function::parse(easy_with_bind, true).unwrap();
        let easy_string_param = Function::parse(easy_string_param, true).unwrap();

        dbg!(easy.to_string());
        dbg!(easy_param_single.to_string());
        dbg!(easy_param_multi.to_string());
        dbg!(easy_s.to_string());
        dbg!(easy_with_bind.to_string());
        dbg!(easy_string_param.to_string());
    }
}
