//! 🆗 : 测试完成
//! ⚡️ : faster
use std::collections::HashMap;

// use crate::{
//     ast::{ASTNodes, PropertyKeyType, PropsKey, Tag},
//     common::parse_comment as parse_common_comment,
//     CloseType, Ident, Value,
// };

use crate::model::TemplateModel;

use gen_utils::error::{Error, ParseError};
use gen_utils::{
    common::tokenizer::{END_SIGN, END_START_SIGN, EQUAL_SIGN, SELF_END_SIGN},
    parser::{parse_bind_key, parse_function_key, parse_normal, parse_string, trim},
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{alphanumeric1, char},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

/// ## ⚡️ parse normal label 🆗
/// use in tag_start | tag_end to parse the tag_name
/// ### example
/// - parse xxx
/// - ~parse xxx-zzz~
/// - parse xxx_zzz
#[allow(dead_code)]
fn parse_tag_name(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

/// ## parse tag start (<tag_name key="value">) 🆗
/// format : `<tag_name key="value">`
/// ### return
/// `IResult<&str, ASTNodes>`
/// ### Example
/// ```rust
/// let input = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
/// let res = parse_tag_start(input).unwrap();
/// ```
pub fn parse_tag_start(input: &str) -> IResult<&str, ASTNodes> {
    let (remain, (name, props)) = trim(preceded(
        char('<'),
        tuple((parse_tag_name, parse_properties)),
    ))(input)?;
    let props = if props.is_empty() {
        None
    } else {
        Some(
            props
                .into_iter()
                .map(|(key_type, key, value)| (PropsKey::new(key, false, key_type), value))
                .collect::<HashMap<_, _>>(),
        )
    };
    let mut tag = Tag::new_tag_props(name, props);
    let mut remain = remain.trim();
    // check if remain start with `/>`, if true, is end tag
    if remain.starts_with(SELF_END_SIGN) {
        remain = remain.trim_start_matches(SELF_END_SIGN);
        tag.set_ty(CloseType::SelfClosed);
    } else {
        remain = remain.trim_start_matches(END_SIGN);
    }

    Ok((remain, tag.into()))
}

/// ## parse property key 🆗
/// - normal: k
/// - bind: :k
/// - function: @k
#[allow(dead_code)]
fn parse_property_key(input: &str) -> IResult<&str, (&str, &str)> {
    fn parse_normal_key(input: &str) -> IResult<&str, (&str, &str)> {
        let (input, value) = recognize(pair(
            alphanumeric1,
            take_while_m_n(0, usize::MAX, |c: char| c == '_' || c.is_alphanumeric()),
        ))(input)?;
        Ok((input, ("", value)))
    }
    alt((parse_bind_key, parse_function_key, parse_normal_key))(input)
}

/// ## parse tag property 🆗
/// - normal: `k=\"v\"` value always Value::String
/// - bind: `:k=\"v\"` value flexable (Value::Bind)
/// - function: `@k=\"v\"` value depend on function return (Value:Function)
/// ### return
/// (property_type, property_key, property_value)
#[allow(dead_code)]
fn parse_property(input: &str) -> IResult<&str, (PropertyKeyType, &str, Value)> {
    let (input, (key_type, key)) = parse_property_key(input)?;
    let input = input.trim();
    let key_type: PropertyKeyType = key_type.into();
    // if following is not `=`, means no value, use default true
    if !input.starts_with('=') {
        // now only `else` need to use bind
        let key_type = if key == "else" {
            PropertyKeyType::Bind
        } else {
            key_type
        };
        return Ok((
            input,
            (
                key_type,
                key,
                Value::Bind(crate::Bind::Normal(vec![Ident::new("else")])),
            ),
        ));
    }

    let (input, value) = preceded(tag(EQUAL_SIGN), parse_string)(input)?;
    // parse value
    let value = key_type
        .to_value(value)
        .map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))?;
    Ok((input, (key_type, key, value)))
}

fn parse_properties(input: &str) -> IResult<&str, Vec<(PropertyKeyType, &str, Value)>> {
    many0(trim(parse_property))(input)
}

/// ## parse end tag (`</xxx>`)
#[allow(dead_code)]
fn parse_end_tag_common(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, value) = trim(delimited(
        trim(tag(END_START_SIGN)),
        parse_tag_name,
        trim(tag(END_SIGN)),
    ))(input)?;
    Ok((input, (END_START_SIGN, value)))
}

/// ## parse tag end 🆗
/// - self end : `/>`
/// - more end : `>` after this , may include children nodes , end is tag end `</xxx>`
#[allow(dead_code)]
fn parse_tag_end(input: &str) -> IResult<&str, &str> {
    alt((tag(SELF_END_SIGN), tag(END_SIGN)))(input)
}

#[allow(dead_code)]
fn parse_comment(input: &str) -> IResult<&str, ASTNodes> {
    match parse_common_comment(input) {
        Ok((input, comment)) => Ok((input, comment.into())),
        Err(e) => Err(e),
    }
}

#[deprecated = "use parse_end_tag_common instead"]
#[allow(dead_code)]
fn to_end_tag(input: &str, tag_name: String) -> IResult<&str, &str> {
    let mut rest = input;
    let mut remain = "";
    let mut nested_count = 0; // 用于计数嵌套标签

    loop {
        match take_until(END_START_SIGN)(rest) {
            Ok((new_rest, taken)) => {
                // 尝试匹配开始标签，增加嵌套计数

                if taken.trim().starts_with(&(String::from("<") + &tag_name)) {
                    nested_count += 1;
                }
                // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
                match delimited(
                    trim(tag(END_START_SIGN)),
                    tag(tag_name.as_str()),
                    trim(tag(END_SIGN)),
                )(new_rest)
                {
                    Ok((final_rest, _)) => {
                        if nested_count == 0 {
                            // 将 taken 继续放入 remain 中
                            remain = &input[..(remain.len() + taken.len())];
                            // 成功找到结束标签，返回累积的内容和剩余的输入
                            return Ok((final_rest, remain));
                        } else {
                            nested_count -= 1; // 减少嵌套计数，继续处理
                            remain = &input[..(remain.len() + taken.len() + tag_name.len() + 3)]; // 加 3 是为了包括 "</"
                            rest = final_rest;
                        }
                        // //将taken继续放入remain中
                        // remain = &input[..(remain.len() + taken.len())];
                        // // 成功找到结束标签，返回累积的内容和剩余的输入
                        // return Ok((final_rest, remain));
                    }
                    Err(_) => {
                        // 没有找到有效的结束标签，将 "</" 之前的内容加入累积，并继续处理
                        remain = &input[..input.len() - new_rest.len() + 2]; // 加 2 是为了包括 "</"
                        rest = &new_rest[2..]; // 跳过 "</"，继续尝试
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }
}

#[allow(dead_code)]
fn parse_end_tag(input: &str, name: String) -> IResult<&str, (&str, &str)> {
    let (input, value) = trim(delimited(
        trim(tag(END_START_SIGN)),
        tag(&*name),
        trim(tag(END_SIGN)),
    ))(input)?;
    Ok((input, (END_START_SIGN, value)))
}

/// ## parse tag ✅ 🆗
#[allow(dead_code)]
pub fn parse_tag<'a>(
    input: &'a str,
) -> Result<(&'a str, ASTNodes), nom::Err<nom::error::Error<&'a str>>> {
    // parse tag start or comment return ASTNodes, we can use is_tag to check
    let (input, mut ast_node) = trim(alt((parse_comment, parse_tag_start)))(input)?;
    let (is_tag, is_self_closed) = ast_node.is_tag_close();
    if is_tag && !is_self_closed {
        // is tag, nest parse tag
        let tag_name = ast_node.get_tag_name().to_string();
        // trim input and check is start with `</tag_name>`
        match parse_end_tag(input, tag_name.clone()) {
            Ok((input, _)) => {
                return Ok((input, ast_node));
            }
            Err(_) => {
                // has children, parse children
                let (input, mut children) = many0(parse_tag)(input)?;

                let input = match parse_end_tag_common(input) {
                    Ok((remain, _)) => remain,
                    Err(_) => input,
                };

                if !children.is_empty() {
                    children
                        .iter_mut()
                        .for_each(|child| child.set_parent(ast_node.clone()));

                    ast_node.set_tag_children(children);
                }
                let input = input.trim();
                // dbg!(input);
                // 这里说明有和当前ast_node同级的标签，需要返回到上一级来解析
                if preceded(char('<'), parse_tag_name)(input).is_ok()
                    && parse_end_tag_common(input).is_err()
                {
                    // // means input still has tags
                    // let (input, mut children_remain) = many0(|i| parse_tag(i, nests))(input)?;
                    // // dbg!(input, &ast_node, &children_remain);
                    // let mut ast_node_no_children = ast_node.clone();
                    // ast_node_no_children.clear_tag_children();
                    // children_remain
                    //     .iter_mut()
                    //     .for_each(|child| child.set_parent(ast_node_no_children.clone()));

                    // ast_node.extend_tag_children(children_remain);
                    return Ok((input, ast_node));
                }
                return Ok((input, ast_node));
            }
        };
    }
    // if is not tag, is comment -> do recursive parse
    Ok((input, ast_node))
}

/// ## parse template Ⓜ️
/// main template parser
#[allow(dead_code)]
pub fn parse_template(input: &str) -> Result<Vec<ASTNodes>, Error> {
    match many1(parse_tag)(input) {
        Ok((remain, asts)) => {
            if remain.is_empty() {
                return Ok(asts);
            }
            Err(ParseError::template(remain).into())
        }
        Result::Err(e) => Err(ParseError::template(&e.to_string()).into()),
    }
}
