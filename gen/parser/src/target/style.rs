use std::collections::HashMap;

use gen_utils::{
    common::tokenizer::{FUNCTION_SIGN, IMPORT},
    error::{Error, ParseError},
    parser::{parse_value, trim},
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_until1},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, pair},
    IResult,
};

use crate::{
    ast::{ASTNodes,  PropsKey, Style},
    common::{parse_comment as parse_common_comment, Special},
    Value,
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
/// - import
/// - identifier
fn parse_ident(input: &str) -> IResult<&str, ASTNodes> {
    let (input, style_type) = alt((
        trim(tag(STYLE_CLASS)),
        trim(tag(STYLE_ID)),
        trim(tag(STYLE_PESUDO)),
        trim(tag(IMPORT)),
        trim(tag(FUNCTION_SIGN)),
    ))(input)?;

    let (input, name) = parse_value(input)?;
    let style = Style::new_style_start(name, style_type.into());
    Ok((input, style.into()))
}

fn parse_property_key(input: &str) -> IResult<&str, &str> {
    parse_value(input)
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
    // let (remain, (sign, (name, params, is_style))) = alt((bind, function, normal))(value)?;

    match Value::parse_style(value) {
        Ok(value) => {
            let k = PropsKey::from_value_with(&value, key, true);
            Ok((input, (k, value)))
        }
        Err(e) => {
            panic!("value: {} can not parse, more info: {}", value, e);
        }
    }
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
                // remove end `}`
                // let (input, _) = many0(trim(tag(HOLDER_END)))(input)?;
                let (input, _) = trim(tag(HOLDER_END))(input)?;
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

/// # parse styleⓂ️
/// main style parser
/// ## Test
/// See [test_style](tests/src/parser/target/style.rs) 
#[allow(dead_code)]
pub fn parse_style(input: &str) -> Result<Vec<ASTNodes>, Error> {
    match many1(parse_single)(input) {
        Ok((remain, asts)) => {
            if remain.is_empty() {
                return Ok(asts);
            }

            Err(ParseError::template(remain).into())
        }
        Result::Err(e) => Err(ParseError::template(&e.to_string()).into()),
    }
}