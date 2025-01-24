#[cfg(test)]
mod test_color {
    use std::str::FromStr;

    use gen_parser::{
        common::{parse_hex_color, Hex, LinearGradient, Percentage, RadialGradient, Rgba},
        target::parse_style,
        ASTNodes,
    };

    #[test]
    fn test_radial() {
        let style = r#"
        .app{
            background_color : radial_gradient(#7, #3 15%, #f 24%, #d);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            ASTNodes::Tag(_) => todo!(),
            ASTNodes::Comment(_) => todo!(),
            ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.as_fn().unwrap().clone();
                let color = RadialGradient::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_linear2() {
        let style = r#"
        .app{
            background_color : linear_gradient(180deg, #7, #3 15%, #f 24%, #d);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            ASTNodes::Tag(_) => todo!(),
            ASTNodes::Comment(_) => todo!(),
            ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.as_fn().unwrap().clone();
                let color = LinearGradient::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_linear1() {
        let style = r#"
        .app{
            background_color : linear_gradient(180deg, #7 60%, #3 100%);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            ASTNodes::Tag(_) => todo!(),
            ASTNodes::Comment(_) => todo!(),
            ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.as_fn().unwrap().clone();
                let color = LinearGradient::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_rgba() {
        let style = r#"
        .app{
            background_color : rgba(44, 128, 155, 0.5);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            ASTNodes::Tag(_) => todo!(),
            ASTNodes::Comment(_) => todo!(),
            ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.as_fn().unwrap().clone();
                let color = Rgba::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_percentage() {
        let p = Percentage::from_str("11.5%").unwrap();
        let p2 = Percentage::from_str("11.5");
        assert_eq!(p.0, 11.5);
        assert!(p2.is_err());
    }
    #[test]
    fn test_hex() {
        let h = Hex::from_str("#363").unwrap();
        let h2 = Hex::from_str("#3333");
        assert_eq!(h.0, "#336633FF".to_string());
        assert!(h2.is_err());
    }

    #[test]
    fn parse_hex() {
        let h1 = parse_hex_color("#3");
        let h2 = parse_hex_color("#456");
        let h3 = parse_hex_color("#3366aa");
        let h4 = parse_hex_color("#23af453a");
        assert_eq!(h1.unwrap().1, "333333FF");
        assert_eq!(h2.unwrap().1, "445566FF");
        assert_eq!(h3.unwrap().1, "3366aaFF");
        assert_eq!(h4.unwrap().1, "23af453a");
    }
}
