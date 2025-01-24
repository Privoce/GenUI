#[cfg(test)]
mod test_shader {
    use gen_parser::{common::MakepadShader, target::parse_style, ASTNodes};

   
    #[test]
    fn shader_quad() {
        let style = r#"
        .app{
            background_color : shader(|self|{
                fn pixel(self) -> vec4{
                    return #FFF
                }
            });
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
                let shader = MakepadShader::try_from(&fn_v).unwrap();
                dbg!(shader.0.to_string());
            }
        }
    }
}
