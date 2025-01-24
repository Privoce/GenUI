//! Parse tag
//! - template
//! - script
//! - style

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    multi::many0,
    sequence::delimited,
    IResult,
};

use super::comment::parse_offline_comment;
use crate::{ast::Targets, target::template::html::parse_tag_start};
use gen_utils::{
    common::tokenizer::{END_SIGN, END_START_SIGN, SCRIPT, STYLE, TAG_START, TEMPLATE},
    parser::trim,
};

fn delimited_tag<'a>(
    input: &'a str,
    start: &'a str,
    tag_name: &'a str,
) -> IResult<&'a str, &'a str> {
    delimited(trim(tag(start)), tag(tag_name), trim(tag(END_SIGN)))(input)
}

fn start<'a>(input: &'a str, tag_name: &'a str) -> IResult<&'a str, &'a str> {
    delimited_tag(input, TAG_START, tag_name)
}
pub fn end<'a>(input: &'a str, tag_name: &'a str) -> IResult<&'a str, &'a str> {
    delimited_tag(input, END_START_SIGN, tag_name)
}

pub fn until_end<'a>(input: &'a str, tag_name: &'a str) -> IResult<&'a str, &'a str> {
    let mut rest = input;
    let mut remain = "";

    loop {
        match take_until(END_START_SIGN)(rest) {
            Ok((new_rest, taken)) => {
                // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
                match end(new_rest, tag_name) {
                    Ok((final_rest, _)) => {
                        //将taken继续放入remain中
                        remain = &input[..(remain.len() + taken.len())];
                        // 成功找到结束标签，返回累积的内容和剩余的输入
                        return Ok((final_rest, remain));
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

/// # parse normal tag
/// example: `<tag_name>xxxx</tag_name>`
/// ## use
/// - parse template tag
/// - parse script tag
/// - parse style tag
pub fn parse_tag<'o>(input: &'o str, tag_name: &'o str) -> IResult<&'o str, &'o str> {
    let (input, _) = start(input, tag_name)?;
    until_end(input, tag_name)
}

fn parse_template_check(input: &str) -> IResult<&str, Targets> {
    match parse_tag(input, TEMPLATE) {
        Ok(_) => Ok(("", Targets::Template(""))),
        Err(e) => Err(e),
    }
}
fn parse_script_check(input: &str) -> IResult<&str, Targets> {
    // match parse_tag(input, SCRIPT) {
    //     Ok(_) => Ok(("", Targets::Script(""))),
    //     Err(e) => Err(e),
    // }
    let (input, node) = parse_tag_start(input)?;
    match until_end(input, SCRIPT) {
        Ok((_input, content)) => {
            let ast_node = node.is_tag_and_get().unwrap().clone();
            Ok(("", Targets::Script { content, ast_node }))
        }
        Err(e) => Err(e),
    }
}
fn parse_style_check(input: &str) -> IResult<&str, Targets> {
    match parse_tag(input, STYLE) {
        Ok(_) => Ok(("", Targets::Style(""))),
        Err(e) => Err(e),
    }
}

pub fn parse_tag_check(input: &str) -> IResult<&str, Targets> {
    alt((parse_template_check, parse_script_check, parse_style_check))(input)
}

/// # parse `<template>` tag
/// it can handle many conditions
/// - normal: `<template>xxxx</template>`
/// - strange: `<  template   >xxxx</ template>`
/// ## return
/// `IResult<&str, &str>` parse as => `(_,remain)`
/// > remain: remain &str which has consumed template tag
pub fn parse_template_tag(input: &str) -> IResult<&str, Targets> {
    let (input, remain) = parse_tag(input, TEMPLATE)?;
    Ok((input, Targets::Template(remain)))
}

/// # parse `<script>` tag
pub fn parse_script_tag(input: &str) -> IResult<&str, Targets> {
    // let (input, remain) = parse_tag(input, SCRIPT)?;
    // Ok((input, Targets::Script(remain)))
    let (input, node) = parse_tag_start(input)?;

    let (input, content) = until_end(input, SCRIPT)?;
    let ast_node = node.is_tag_and_get().unwrap().clone();
    Ok((input, Targets::Script { content, ast_node }))
}

/// # parse `<style>` tag
pub fn parse_style_tag(input: &str) -> IResult<&str, Targets> {
    let (input, remain) = parse_tag(input, STYLE)?;
    Ok((input, Targets::Style(remain)))
}

/// # parse the whole gen template file
/// after parse, get `Vec<Targets>`
/// then need to convert `Vec<Targets>` -> `ParseTarget`
pub fn parse_all(input: &str) -> IResult<&str, Vec<Targets>> {
    many0(alt((
        parse_offline_comment,
        parse_template_tag,
        parse_script_tag,
        parse_style_tag,
    )))(input)
}