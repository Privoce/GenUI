use crate::model::Template;
use crate::value::{Bind, Ident, Value};
use crate::{nom_err, Comment, Polls, PropKey, PropKeyType, SugarIter};
use gen_utils::error::{Error, ParseError};
use gen_utils::parser::parse_value;
use gen_utils::{
    common::tokenizer::{END_SIGN, END_START_SIGN, EQUAL_SIGN, SELF_END_SIGN},
    parser::{parse_string, trim},
};
use nom::combinator::opt;
use nom::error::ErrorKind;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::char,
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// ## ⚡️ parse normal label 🆗
/// use in tag_start | tag_end to parse the tag_name
/// ### example
/// - parse xxx
/// - ~parse xxx-zzz~
/// - parse xxx_zzz
#[allow(dead_code)]
fn parse_tag_name(input: &str) -> IResult<&str, &str> {
    // parse_normal(input, '_')
    parse_value(input)
}

/// ## parse tag start (<tag_name key="value">) 🆗
/// format : `<tag_name key="value">`
/// ### return
/// `IResult<&str, Template>`
/// ### Example
/// ```rust
/// let input = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
/// let res = parse_tag_start(input).unwrap();
/// ```
pub fn parse_tag_start(input: &str) -> IResult<&str, (Template, CloseType)> {
    let (remain, (name, props)) = trim(preceded(
        char('<'),
        tuple((parse_tag_name, parse_properties)),
    ))(input)?;

    let props = if let Some(props) = props {
        Some(HashMap::from_iter(props.into_iter()))
    } else {
        None
    };

    // let mut tag = Tag::new_tag_props(name, props);
    let mut template = Template::new(name);
    let mut remain = remain.trim();
    // check if remain start with `/>`, if true, is end tag
    let close_type = if remain.starts_with(SELF_END_SIGN) {
        // unimplemented!("self closed tag not support yet, please use `>` to close tag, expect support version 0.2.1+");
        // remain = remain.trim_start_matches(SELF_END_SIGN);
        // tag.set_ty(CloseType::SelfClosed);
        remain = remain.strip_prefix(SELF_END_SIGN).unwrap_or(remain);
        CloseType::SelfClosed
    } else {
        remain = remain.trim_start_matches(END_SIGN);
        CloseType::Usual
    };

    template.props = props;

    Ok((remain, (template, close_type)))
}

/// ## parse property key 🆗
/// - normal: k
/// - bind: :k
/// - function: @k
#[allow(dead_code)]
fn parse_property_key(input: &str) -> IResult<&str, (PropKeyType, &str)> {
    /// ## parse sign then get parse_value
    /// format: `_xxx_zzz` | `@sss_vvv`
    fn parse_sign_key<'a>(
        input: &'a str,
        sign: &'a str,
    ) -> IResult<&'a str, (PropKeyType, &'a str)> {
        let (input, sign) = tag(sign)(input)?;
        let (input, value) = parse_value(input)?;
        let sign = sign
            .parse::<PropKeyType>()
            .map_err(|_| nom_err!(sign, ErrorKind::Tag))?;
        Ok((input, (sign, value)))
    }

    fn parse_normal_key(input: &str) -> IResult<&str, (PropKeyType, &str)> {
        let (input, value) = parse_value(input)?;
        Ok((input, (PropKeyType::Normal, value)))
    }
    /// ## parse property bind key 🆗
    /// - `:xxx`
    /// - `:xxx_zzz`
    fn parse_bind_key(input: &str) -> IResult<&str, (PropKeyType, &str)> {
        parse_sign_key(input, ":")
    }

    /// ## parse property function key 🆗
    /// - `@xxx`
    /// - `@xxx_zzz`
    fn parse_function_key(input: &str) -> IResult<&str, (PropKeyType, &str)> {
        parse_sign_key(input, "@")
    }

    trim(alt((parse_bind_key, parse_function_key, parse_normal_key)))(input)
}

/// ## parse tag property 🆗
/// - normal: `k=\"v\"` value always Value::String
/// - bind: `:k=\"v\"` value flexable (Value::Bind)
/// - function: `@k=\"v\"` value depend on function return (Value:Function)
/// ### return
/// (property_type, property_key, property_value)
#[allow(dead_code)]
fn parse_property(input: &str) -> IResult<&str, (PropKey, Value)> {
    let (input, (key_type, key)) = parse_property_key(input)?;
    let input = input.trim();
    // if following is not `=`, means no value, use default true
    if !input.starts_with('=') {
        // now only `else` need to use bind
        let kv = if key == "else" {
            (
                PropKey::new_bind(key, false),
                Value::Bind(Bind::Normal(vec![Ident::new("else")])),
            )
        } else {
            (PropKey::new_tag_normal(key), Value::Bool(true))
        };
        return Ok((input, kv));
    }

    let (input, value) = preceded(tag(EQUAL_SIGN), parse_string)(input)?;
    // parse value
    let value = key_type
        .to_value(value)
        .map_err(|_| nom_err!(value, ErrorKind::Tag))?;
    Ok((input, (PropKey::new(key, false, key_type), value)))
}

fn parse_properties(input: &str) -> IResult<&str, Option<Vec<(PropKey, Value)>>> {
    opt(many0(trim(parse_property)))(input)
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
fn parse_comment(input: &str) -> IResult<&str, Vec<Comment>> {
    many0(Comment::parse)(input)
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

/// ## parse tag ✅ 🆗 Result<(&'a str, Template), nom::Err<nom::error::Error<&'a str>>>
#[allow(dead_code)]
fn parse_tag<'a>(
    poll: Arc<RwLock<Polls>>,
    mut root: bool,
    mut iter: Option<SugarIter>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Template> {
    move |input: &str| {
        // [parse comment if exist] ------------------------------------------------------------------------------------
        let (input, comments) = parse_comment(input)?;
        // [parse tag start] -------------------------------------------------------------------------------------------
        let (input, (mut template, close_type)) = parse_tag_start(input)?;
        template.root = root;
        root = false;
        let sugar_iter = template
            .after_prop_parse(Arc::clone(&poll), iter.as_ref())
            .map_err(|e| {
                eprintln!("parse_tag error: {:?}", e);
                nom_err!(input, ErrorKind::Fail)
            })?;
        iter = sugar_iter;
        if !comments.is_empty() {
            template.comments.replace(comments);
        }
        // let (is_tag, is_self_closed) = template.is_tag_close();
        // trim input and check is start with `</tag_name>`
        let input = match close_type {
            CloseType::SelfClosed => {
                // no children, return
                input
            }
            CloseType::Usual => {
                // is tag, nest parse tag
                let tag_name = template.name.to_string();
                match parse_end_tag(input, tag_name.to_string()) {
                    Ok((input, _)) => input,
                    Err(_) => {
                        // has children, parse children
                        let (input, mut children) =
                            many0(parse_tag(Arc::clone(&poll), root, iter.clone()))(input)?;

                        let input = match parse_end_tag_common(input) {
                            Ok((remain, _)) => remain,
                            Err(_) => input,
                        };

                        if !children.is_empty() {
                            let (special, name) = template.as_parent();
                            for child in children.iter_mut() {
                                child.set_parent(
                                    special.to_string(),
                                    name.to_string(),
                                    template.root,
                                );
                                child.after_all(Arc::clone(&poll)).map_err(|e| {
                                    eprintln!("parse_tag error: {:?}", e);
                                    nom_err!(input, ErrorKind::Fail)
                                })?;
                            }

                            template.children.replace(children);
                        }
                        let input = input.trim();
                        // 这里说明有和当前ast_node同级的标签，需要返回到上一级来解析
                        if preceded(char('<'), parse_tag_name)(input).is_ok()
                            && parse_end_tag_common(input).is_err()
                        {
                            input
                        } else {
                            input
                        }
                    }
                }
            }
        };

        return Ok((input, template));
    }
}

/// ## parse template Ⓜ️
/// main template parser
#[allow(dead_code)]
pub fn parse(input: &str, poll: Arc<RwLock<Polls>>, root: bool) -> Result<Template, Error> {
    match parse_tag(poll, root, None)(input) {
        Ok((remain, template)) => {
            if remain.is_empty() {
                return Ok(template);
            }
            Err(ParseError::template(remain).into())
        }
        Err(e) => Err(ParseError::template(&e.to_string()).into()),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CloseType {
    SelfClosed,
    Usual,
}
