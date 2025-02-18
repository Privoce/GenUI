use gen_utils::{
    common::tokenizer::{FUNCTION_SIGN, IMPORT},
    error::{Error, ParseError},
    parser::{parse_value, trim},
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_until1},
    error::ErrorKind,
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

use gen_utils::common::tokenizer::{
    HOLDER_END, HOLDER_START, STYLE_CLASS, STYLE_END, STYLE_ID, STYLE_PESUDO, STYLE_START,
};

use crate::{nom_err, Comment, PropKey, Style, StyleVal};

use super::value::Value;

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
fn parse_ident(input: &str) -> IResult<&str, String> {
    let (input, (style_type, name)) = pair(
        alt((
            trim(tag(STYLE_CLASS)),
            trim(tag(STYLE_ID)),
            trim(tag(STYLE_PESUDO)),
            trim(tag(IMPORT)),
            trim(tag(FUNCTION_SIGN)),
        )),
        parse_value,
    )(input)?;

    // let style = Style::new_style_start(name, style_type.into());
    Ok((input, format!("{}{}", style_type, name)))
}

fn parse_property_key(input: &str) -> IResult<&str, &str> {
    parse_value(input)
}

/// ## parse style property
/// - normal : `xxx:zzz;`
/// - bind : `xxx:$zzz;`
/// - function : `xxx:zzz();`
fn parse_property(input: &str) -> IResult<&str, (PropKey, Value)> {
    // maybe user write some comment before the property
    let (input, _) = parse_comment(input)?;
    let (input, (key, _, value)) =
        tuple((parse_property_key, trim(tag(":")), take_until1(";")))(input)?;

    //remove `;`
    let (input, _) = trim(tag(";"))(input)?;
    // let (remain, (sign, (name, params, is_style))) = alt((bind, function, normal))(value)?;
    match Value::parse_style(value) {
        Ok(value) => {
            let k = PropKey::from_value_with(&value, key, true);
            // after all maybe user write some comment, but we do not care
            let (input, _) = parse_comment(input)?;
            Ok((input, (k, value)))
        }
        Err(_) => Err(nom_err!(input, ErrorKind::Fail)),
    }
}

fn parse_properties(input: &str) -> IResult<&str, Vec<(PropKey, Value)>> {
    many0(trim(parse_property))(input)
}

#[allow(dead_code)]
fn parse_comment(input: &str) -> IResult<&str, Vec<Comment>> {
    many0(Comment::parse)(input)
}

/// ## parse single style
/// ```
/// .a{
///   color: red;
///   .b {
///     color: blue;
///   }
/// }
/// // ----- after ---------    
/// {
///     .a : {color: red}, // a class style
///     .a.b : {color: blue} // b class style, which is a child of a
/// }
/// ```
fn parse_single(input: &str) -> IResult<&str, Style> {
    // [parse comment if exist] --------------------------------------------------------------------------------------------------
    let (input, _) = parse_comment(input)?;
    // [parse style ident] -------------------------------------------------------------------------------------------------------
    let (input, key) = trim(parse_ident)(input)?;
    let mut style = Style::new();
    // [find open `{`] -----------------------------------------------------------------------------------------------------------
    let (input, _) = trim(tag(HOLDER_START))(input)?;
    // [if end or parse properties] ----------------------------------------------------------------------------------------------
    let (input, children, properties) = if input.trim().starts_with(HOLDER_END) {
        // - [end] ---------------------------------------------------------------------------------------------------------------
        (input, None, None)
    } else {
        // [parse properties] ----------------------------------------------------------------------------------------------------
        let (input, properties) = parse_properties(input)?;
        let properties = if properties.is_empty() {
            None
        } else {
            Some(StyleVal::from_iter(properties.into_iter()))
        };
        // [nesting parse] -------------------------------------------------------------------------------------------------------
        let (input, children) = many0(parse_single)(input)?;
        // [remove end `}`] ------------------------------------------------------------------------------------------------------
        let (input, _) = trim(tag(HOLDER_END))(input)?;
        if children.is_empty() {
            (input, None, properties)
        } else {
            (input, Some(children), properties)
        }
    };
    // [set properties] ---------------------------------------------------------------------------------------------------------
    style.insert(key.to_string(), properties.unwrap_or_default());
    // [set children] -----------------------------------------------------------------------------------------------------------
    match children {
        Some(children) => {
            for child in children {
                // let all children key add parent key
                style.extend(child.into_iter().map(|(k, v)| (format!("{}{}", key, k), v)));
            }
        }
        None => {}
    }
    Ok((input, style))
}

/// # parse styleⓂ️
/// main style parser
/// ## Test
/// See [test_style](tests/src/parser/target/style.rs)
#[allow(dead_code)]
pub fn parse(input: &str) -> Result<Style, Error> {
    match many0(parse_single)(input) {
        Ok((remain, styles)) => {
            if remain.is_empty() {
                let mut style = Style::new();
                for s in styles {
                    style.extend(s.into_iter());
                }
                return Ok(style);
            }

            Err(ParseError::template(remain).into())
        }
        Result::Err(e) => Err(ParseError::template(&e.to_string()).into()),
    }
}
