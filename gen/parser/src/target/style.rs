use std::collections::HashMap;

use gen_utils::{error::Error, parser::{parse_value, trim}};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_until1},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, pair},
    IResult,
};

use crate::{
    ast::{ASTNodes, PropertyKeyType, PropsKey, Style}, common::{parse_comment as parse_common_comment, Special}, Bind, Value 
};

use gen_utils::common::tokenizer::{
    HOLDER_END, HOLDER_START, STYLE_CLASS, STYLE_END, STYLE_ID, STYLE_PESUDO, STYLE_START,
};

#[allow(dead_code)]
pub fn parse_style_tag(input: &str) -> IResult<&str, &str> {
    let (input, _) = trim(tag(STYLE_START))(input)?;
    let (_, input) = take_until(STYLE_END)(input)?;
    Ok((input, "style"))
}

/// ## parser ident
/// - class
/// - id
/// - pesudo
fn parse_ident(input: &str) -> IResult<&str, ASTNodes> {
    let (input, style_type) = alt((
        trim(tag(STYLE_CLASS)),
        trim(tag(STYLE_ID)),
        trim(tag(STYLE_PESUDO)),
    ))(input)?;
    let (input, name) = parse_value(input)?;
    let style = Style::new_style_start(name, style_type.into());
    Ok((input, style.into()))
}

fn parse_property_key(input: &str) -> IResult<&str, &str> {
    parse_value(input)
}

// begin $ `(input , (sign,name))`
fn bind(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
    let (input, (sign, name)) = pair(tag("$"), parse_property_key)(input)?;
    Ok((input, (sign, (name, "", None))))
}

/// end () `(type, (name,params))`
pub fn function(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
    fn normal_fn(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
        let (input, (name, params)) = pair(
            parse_property_key,
            recognize(delimited(tag("("), take_until(")"), tag(")"))),
        )(input)?;

        Ok((input, ("()", (name, params, Some(true)))))
    }
    alt((Special::makepad_shader_parser, normal_fn))(input)
}

fn normal(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
    // TODO:
    // 增加解析对象类型 `{}`
    // 增加解析数组类型 `[]` (solve 使用` `分割)
    // let (input, value) = (input)?;
    Ok(("", ("", (input, "", None))))
}

/// ## parse style property
/// - normal : `xxx:zzz;`
/// - bind : `xxx:$zzz;`
/// - function : `xxx:zzz();`
fn parse_property(input: &str) -> IResult<&str, (PropsKey, Value)> {
    let (input, key) = parse_property_key(input)?;
    let (input, _) = trim(tag(":"))(input)?;
    let (input, value) = take_until1(";")(input)?;
    //remove `;`
    let (input, _) = trim(tag(";"))(input)?;
    let (remain, (sign, (name, params, is_style))) = alt((bind, function, normal))(value)?;
    //check remain is empty ,or should panic
    return if remain.is_empty() {
        // match sign
        let (key, value) = match sign {
            "" => (
                PropsKey::new(key, true, PropertyKeyType::Normal),
                Value::UnKnown(name.trim_matches('"').to_string()),
            ),
            "()" => (
                PropsKey::new(key, true, PropertyKeyType::Function),
                Value::Function((name, params, is_style.unwrap()).into()),
            ), //do not afraid to panic cause only function has is_style
            "$" => (
                PropsKey::new(key, true, PropertyKeyType::Bind),
                Value::Bind(Bind::Normal(name.to_string())),
            ),
            _ => panic!("Invalid Value:{}", sign),
        };

        Ok((input, (key, value)))
    } else {
        panic!("parse remain:{}", remain);
    };
}

#[allow(dead_code)]
fn parse_comment(input: &str) -> IResult<&str, ASTNodes> {
    match parse_common_comment(input) {
        Ok((input, comment)) => Ok((input, comment.into())),
        Err(e) => Err(e),
    }
}

fn parse_single(input: &str) -> IResult<&str, ASTNodes> {
    let (input, mut ast) = trim(alt((parse_ident, parse_comment)))(input)?;
    return if ast.is_style() {
        // find open `{`
        let (input, _) = trim(tag(HOLDER_START))(input)?;

        let (input, children, properties) = match trim(tag(HOLDER_END))(input) {
            Ok((input, _)) => (input, None, None), //end
            Err(_) => {
                // parse property
                let (input, properties) = many0(trim(parse_property))(input)?;
                let properties = if properties.is_empty() {
                    None
                } else {
                    Some(properties)
                };
                // nesting parse
                let (input, mut children) = many0(parse_single)(input)?;
                // set parent
                children
                    .iter_mut()
                    .for_each(|child| child.set_parent(ast.clone()));
                // remove end `)`
                let (input, _) = many0(trim(tag(HOLDER_END)))(input)?;
                (input, Some(children), properties)
            }
        };
        //set properties
        match properties {
            Some(p) => ast.set_properties(Some(HashMap::from_iter(p.into_iter()))),
            None => {}
        };
        // set children
        match children {
            Some(c) => ast.set_children(c),
            None => {}
        }
        Ok((input, ast))
    } else {
        Ok((input, ast))
    };
}

/// ## parse styleⓂ️
/// main style parser
#[allow(dead_code)]
pub fn parse_style(input: &str) -> Result<Vec<ASTNodes>, Error> {
    match many1(parse_single)(input) {
        Ok((remain, asts)) => {
            if remain.is_empty() {
                return Ok(asts);
            }
            Err(Error::template_parser_remain(remain))
        }
        Result::Err(_) => Err(Error::new("error parsing style")),
    }
}

#[cfg(test)]
mod test_style {

    use crate::ast::{ASTNodes, Style};

    use super::{function, parse_style, parse_style_tag};
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
        // let st = res
        //     .into_iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>()
        //     .join("\n");
        // // "E:/Rust/try/makepad/gen/parser/c.css"
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/gen/parser/c.css").unwrap();
        // let _ = f.write(st.as_bytes());
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

    #[test]
    fn test_function() {
        let func1 = r#"linear_gradient(180deg, #7, #3)"#;
        let res = function(func1).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_tag() {
        let tag = r#"
            <style></style>
        "#;
        let res = parse_style_tag(tag).unwrap();
        assert_eq!(res, ("", "style",));
    }

    #[test]
    fn test_ident() {
        let ident1 = ".app{}";
        let ident2 = "#app1{}";
        let ident3 = "&::hover{}";
        let res1 = parse_style(ident1).unwrap();
        let res2 = parse_style(ident2).unwrap();
        let res3 = parse_style(ident3).unwrap();
        assert_eq!(
            res1,
            vec![ASTNodes::Style(Box::new(Style::new_style_start(
                "app",
                ".".into()
            )))]
        );
        assert_eq!(
            res2,
            vec![ASTNodes::Style(Box::new(Style::new_style_start(
                "app1",
                "#".into()
            )))]
        );
        assert_eq!(
            res3,
            vec![ASTNodes::Style(Box::new(Style::new_style_start(
                "hover",
                "&::".into()
            )))]
        );
    }
}
