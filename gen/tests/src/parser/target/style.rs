#[cfg(test)]
mod test_style {
    use gen_parser::target::parse_style;

    #[test]
    fn easy_identifier() {
        let style = r#"
        <style>
            @common_height : 30;
            .app{
                height : $common_height;
                width : 100;
            }
        </style>"#;
        let res = parse_style(style).unwrap();
        dbg!(res);
    }

    #[test]
    fn easy_import() {
        let style = r#"
            import "E:/Rust/try/makepad/gen/parser/s.gen";
            .app{
                height : 30;
                width : 100;
            }
        "#;
        let res = parse_style(style).unwrap();
        dbg!(res);
    }

    #[test]
    fn easy_style2() {
        let style = r#"
        .app{
            &::hover{
                start: 0.5;
                duration: 1.0;
                default: off;
                target: all;
                redraw: true;
                ease: In;
            }
        }
        "#;
        let res = parse_style(style).unwrap();
        dbg!(res);
    }
    #[test]
    fn test_style_all() {
        let style = r#"
        .app{
            // yysyd
            .ui_ui{
                height : fill;
                width : fill;
                show_bg : true;
                background_color : linear_gradient(180deg, #7, #3); 
                // background_col
                .body{
                    flow : down;
                    spacing : 20;
                    align : 0.5 0.5;
                    .button1{ }
                    .input1{
                        height : 30;
                        width : 100;
                    }
                    .label1{
                        color : #ffffff;
                    }
                }
            }
        }
        "#;

        let res = parse_style(style).unwrap();
        dbg!(res);
    }

    #[test]
    fn easy_style() {
        let style = r#"
        <style>
            .app{
                height : 30;
                width : 100;
            }
        </style>"#;
        let res = parse_style(style).unwrap();
        dbg!(res);
    }
}
