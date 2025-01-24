#[cfg(test)]
mod test_bind {
    use std::str::FromStr;

    use gen_parser::{Bind, For, ForItem, Ident};

    #[test]
    fn style() {
        let input1 = r#"$A"#;
        let res = Bind::parse_style(input1).unwrap();
        assert_eq!(res, Bind::Normal(vec![Ident::new("A")]));
    }

    #[test]
    fn from_str_in_template() {
        let bind = Bind::from_str("item in iter_ident").unwrap();
        assert_eq!(
            bind,
            Bind::For(For {
                iter_ident: vec![Ident::new("iter_ident")],
                index: None,
                item: ForItem::Ident("item".to_string())
            })
        );
    }

    #[test]
    fn bind_normal(){
        let b = Bind::Normal(vec![Ident::new("A"), Ident::new("B")]);
        dbg!(b.to_string());
    }

    #[test]
    fn for_ident(){
        let input = "iter_ident.0";
        let idents = Ident::parse_idents(input).unwrap();
        dbg!(idents);
    }

    #[test]
    fn for_in_template_nested() {
        let (input, bind) = For::parser("(index, item) in iter_ident.0").unwrap();
        assert_eq!(input, "");
        assert_eq!(
            bind.iter_ident,
            vec![Ident::new("iter_ident"), Ident::dot("0")]
        );
        assert_eq!(bind.index, Some("index".to_string()));
        assert_eq!(bind.item, ForItem::Tuple(vec![ForItem::Ident("item".to_string())]));
    }

    #[test]
    fn for_in_template() {
        let (input, bind) = For::parser("(index, item) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, vec![Ident::new("iter_ident")]);
        assert_eq!(bind.index, Some("index".to_string()));
        assert_eq!(bind.item, ForItem::Ident("item".to_string()));

        let (input, bind) = For::parser("item in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, vec![Ident::new("iter_ident")]);
        assert_eq!(bind.index, None);
        assert_eq!(bind.item, ForItem::Ident("item".to_string()));

        let (input, bind) = For::parser("(item1, (item2, item3)) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, vec![Ident::new("iter_ident")]);
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(
            bind.item,
            ForItem::Tuple(vec![
                ForItem::Ident("item2".to_string()),
                ForItem::Ident("item3".to_string())
            ])
        );

        let (input, bind) = For::parser("(item1, _) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, vec![Ident::new("iter_ident")]);
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(bind.item, ForItem::None);

        let (input, bind) = For::parser("(item1, ()) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, vec![Ident::new("iter_ident")]);
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(bind.item, ForItem::Tuple(vec![]));

        let (input, bind) = For::parser("(item1, (_, _)) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, vec![Ident::new("iter_ident")]);
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(
            bind.item,
            ForItem::Tuple(vec![ForItem::None, ForItem::None])
        );
    }
}
