mod bind;
mod r#enum;
mod function;
mod r#struct;

#[allow(unused)]
#[cfg(test)]
mod test_value {
    use gen_parser::Value;
    
    #[test]
    fn test_from_str2() {
        // ------- test ok ----------------------------------------
        let input8 = r#"Fill"#;
        let input13 = r#"A::B"#;
        let input14 = r#"A::B::C"#;
        let input15 = r#"Size::Fixed(100.0)"#;
        let input16 = r#"Position::Abs{top: 10.0, left: 20}"#;
        let input17 = r#"Position::Abs(100 12)"#;
        let input18 = r#"Position::Abs("hello")"#;
        dbg!(Value::parse_style(input8));
        dbg!(Value::parse_style(input13));
        dbg!(Value::parse_style(input14));
        dbg!(Value::parse_style(input15));
        dbg!(Value::parse_style(input16));
        dbg!(Value::parse_style(input17));
        dbg!(Value::parse_style(input18));
        let input1 = r#""hello""#;
        let input5 = r#"19"#;
        let input6 = r#"19.0"#;
        let input7 = r#"true"#;
        let input9 = r#"1 2 3 4"#;
        let input10 = r#"-12"#;
        dbg!(Value::parse_style(input1));
        dbg!(Value::parse_style(input5));
        dbg!(Value::parse_style(input6));
        dbg!(Value::parse_style(input7));
        dbg!(Value::parse_style(input9));
        dbg!(Value::parse_style(input10));
        let input2 = r#"{name:"hello"}"#;
        let input11 = r#"{name:"hello", age: 12}"#;
        let input12 = r#"{name:"hello", age: 12, children: {name: "hello", age: 12}}"#;
        dbg!(Value::parse_style(input2));
        dbg!(Value::parse_style(input11));
        dbg!(Value::parse_style(input12));
        let input3 = r#"[1,2,3]"#;
        let input4 = r#"[{"name":"hello"}]"#;
        dbg!(Value::parse_style(input3));
        dbg!(Value::parse_style(input4));
        // --------------------------------------------------------
        let input19 = r#"Size::Fixed([1, 2])"#;
        let input20 = r#"Size::Fixed(["hello", "world"])"#;
        dbg!(Value::parse_style(input19).unwrap().to_string());
        dbg!(Value::parse_style(input20).unwrap().to_string());
    }
    #[test]
    fn test_to_string() {
        // ------- test ok ----------------------------------------
        let input8 = r#"'Fill'"#;
        let input13 = r#"A::B"#;
        let input14 = r#"A::B::C"#;
        let input15 = r#"Size::Fixed(100.0)"#;
        let input16 = r#"Position::Abs{top: 10.0, left: 20}"#;
        let input17 = r#"Position::Abs(100 12)"#;
        let input18 = r#"Position::Abs("hello")"#;
        dbg!(Value::parse_template(input8).unwrap().to_string());
        dbg!(Value::parse_style(input13).unwrap().to_string());
        dbg!(Value::parse_style(input14).unwrap().to_string());
        dbg!(Value::parse_style(input15).unwrap().to_string());
        dbg!(Value::parse_style(input16).unwrap().to_string());
        dbg!(Value::parse_style(input17).unwrap().to_string());
        dbg!(Value::parse_style(input18).unwrap().to_string());
        let input1 = r#""hello""#;
        let input5 = r#"19"#;
        let input6 = r#"19.0"#;
        let input7 = r#"true"#;
        let input9 = r#"1 2 3 4"#;
        let input10 = r#"-12"#;
        dbg!(Value::parse_style(input1).unwrap().to_string());
        dbg!(Value::parse_style(input5).unwrap().to_string());
        dbg!(Value::parse_style(input6).unwrap().to_string());
        dbg!(Value::parse_style(input7).unwrap().to_string());
        dbg!(Value::parse_style(input9).unwrap().to_string());
        dbg!(Value::parse_style(input10).unwrap().to_string());
        let input2 = r#"{name:"hello"}"#;
        let input11 = r#"{name:"hello", age: 12}"#;
        let input12 = r#"{name:"hello", age: 12, children: {name: "hello", age: 12}}"#;
        dbg!(Value::parse_style(input2).unwrap().to_string());
        dbg!(Value::parse_style(input11).unwrap().to_string());
        dbg!(Value::parse_style(input12).unwrap().to_string());
        let input3 = r#"[1,2,3]"#;
        let input4 = r#"[{"name":"hello"}]"#;
        dbg!(Value::parse_style(input3).unwrap().to_string());
        dbg!(Value::parse_style(input4).unwrap().to_string());
    }
}
