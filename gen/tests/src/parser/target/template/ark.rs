#[cfg(test)]
mod test_ark {
    use gen_parser::target::template::ark::parse_ark_template;

    #[test]
    fn test7() {
        let input = r#"
        Root(){
            Window(){
                View(){
                    Label(){}.text("Gen + Makepad + Ark")
                }
            }.id("main_window")
        }.id("ui")
        "#;
        let result = parse_ark_template(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test6() {
        let input = r#"
            Row(){
                Row(){
                    Text("Hello")
                    Text("Hello2").height("20%")
                }.width("60%")
            }.id("ui")
        "#;
        let result = parse_ark_template(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test5() {
        let input = r#"
                Column() {
                    Text("Hello world1")
                    Text("Hello world2")
                }.width("80%").height(50)
        "#;
        let result = parse_ark_template(input);
        // assert!(result.is_ok());
        assert!(result.is_ok());
    }

    #[test]
    fn test4() {
        let input = r#"
            Row(){
                Column() {}.width("80%").height(50)
                Text("Hello2")
            }
        "#;

        let result = parse_ark_template(input);
        assert!(result.is_ok());
        // assert!(result.is_ok());
    }

    #[test]
    fn test3() {
        let input = r#"
            Row(){
                Text("Hello1")
                Text("Hello2")
            }
        "#;

        let result = parse_ark_template(input);
        // dbg!(result);
        assert!(result.is_ok());
    }

    #[test]
    fn test2() {
        let input = r#"
            Text("Hello")
                .fontSize(20)
                .border({width: 1})
                .textAlign(TextAlign::Center)
        "#;

        let result = parse_ark_template(input);
        assert!(result.is_ok());
        // dbg!(result);
    }

    #[test]
    fn test1() {
        let input = r#"
        Row(){
            Text("Hello")
                .fontSize(20)
                .border({width: 1})
                .textAlign(TextAlign::Center)
        }.id("ui")
        "#;

        let result = parse_ark_template(input);
        assert!(result.is_ok());
        // assert!(result.is_ok());
    }

    #[test]
    fn test0() {
        let input = r#"
        Row()
        "#;
        let input2 = r#"
        Row(){}.width("100%")
        "#;

        let result = parse_ark_template(input);
        let result2 = parse_ark_template(input2);
        assert!(result.is_ok());
        assert!(result2.is_ok());
        // assert!(result.is_ok());
    }
}
