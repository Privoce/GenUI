#[cfg(test)]
mod test_struct {
    use gen_parser::{Struct, Value};

    #[test]
    fn parse() {
        let input1 = r#"{name: "129"}"#;
        let input2 = r#"{name: "hello", age: {a: 12, b: 1.0}}"#;
        let res1 = Struct::parse_style(input1).unwrap();
        let res2 = Struct::parse_style(input2).unwrap();

        assert_eq!(
            res1,
            Struct {
                name: None,
                fields: vec![("name".to_string(), Value::String("129".to_string()))]
                    .into_iter()
                    .collect(),
                is_anonymous: true
            }
        );

        assert_eq!(
            res2,
            Struct {
                name: None,
                fields: vec![
                    ("name".to_string(), Value::String("hello".to_string())),
                    (
                        "age".to_string(),
                        Value::Struct(Struct {
                            name: None,
                            fields: vec![
                                ("a".to_string(), Value::ISize(12)),
                                ("b".to_string(), Value::Double(1.0))
                            ]
                            .into_iter()
                            .collect(),
                            is_anonymous: true
                        })
                    )
                ]
                .into_iter()
                .collect(),
                is_anonymous: true
            }
        );
    }
}
