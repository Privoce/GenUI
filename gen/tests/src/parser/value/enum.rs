#[cfg(test)]
mod test_enum {
    use gen_parser::{Enum, EnumItem, ParseResult, ParseTarget, Value};

    #[test]
    fn parse() {
        let enum1 = "Size::Fit";
        let enum2 = "Size::Fixed(100.0)";
        let enum3 = "TextAlign::Center";
        let enum4 = "Fill";

        let res1 = Enum::parse_template(enum1).unwrap();
        let res2 = Enum::parse_template(enum2).unwrap();
        let res3 = Enum::parse_template(enum3).unwrap();
        let res4 = Enum::parse_template(enum4).unwrap();

        assert_eq!(
            res1,
            Enum {
                field_chain: vec![
                    EnumItem::Root("Size".to_string()),
                    EnumItem::Leaf("Fit".to_string(), None)
                ]
            }
        );

        assert_eq!(
            res2,
            Enum {
                field_chain: vec![
                    EnumItem::Root("Size".to_string()),
                    EnumItem::Leaf("Fixed".to_string(), Some(Value::Double(100.0)))
                ]
            }
        );

        assert_eq!(
            res3,
            Enum {
                field_chain: vec![
                    EnumItem::Root("TextAlign".to_string()),
                    EnumItem::Leaf("Center".to_string(), None)
                ]
            }
        );

        assert_eq!(
            res4,
            Enum {
                field_chain: vec![EnumItem::Leaf("Fill".to_string(), None)]
            }
        )
    }

    #[test]
    fn in_style() {
        let input = r#"
        <style>
        .app{
            height: Fill;
            width: Size::Fit;
            max_height: Size::Fixed(100.0);
            text_align: TextAlign::Center;
        }
        </style>
        "#;

        let res = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        dbg!(res);
    }
}
