use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until, take_until1, take_while_m_n},
    character::complete::{alpha1, multispace0},
    combinator::{opt, recognize},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

use crate::{common::shadow_cmd_with, error::Error};

/// List all the installed packages by cargo
/// ## format:
/// ract v0.1.2:
/// ract
/// robius-packaging-commands v0.1.0 (https://github.com/project-robius/robius-packaging-commands.git#8203f63b):
///     robius-packaging-commands
/// rust_pixel v0.5.9 (/Users/shengyifei/projects/gen_ui/rust_pixel):
///     cargo-pixel
pub fn cargo_install_list() -> Result<HashMap<String, InstalledItem>, Error> {
    shadow_cmd_with(
        "cargo",
        ["install", "--list"],
        Option::<&std::path::Path>::None,
        |status, output| {
            if status.success() {
                // get the output
                let output = String::from_utf8_lossy(&output.stdout);
                let (remain, items) = parse(&output).map_err(|e| Error::from(e.to_string()))?;
                if remain.is_empty() {
                    Ok(items)
                } else {
                    Err(Error::from(format!(
                        "Failed to parse the output, still have remain: {}",
                        remain
                    )))
                }
            } else {
                Err(Error::from("Failed to list installed packages"))
            }
        },
    )
}

#[derive(Debug)]
pub struct InstalledItem {
    pub version: String,
    /// The path of the package or remote url
    pub path: Option<String>,
    pub exes: Vec<String>,
}

fn line(input: &str) -> IResult<&str, (String, InstalledItem)> {
    // split bt `:` (需要排除`://`的情况)
    let (remain, input) = take_until_unbalanced('(', ')', ":")(input)?;
    // 1. name
    let (input, name) = trim(parse_name)(input)?;
    // 2. version
    let (input, version) = trim(parse_version)(input)?;
    // 3. path(optional)
    let (input, path) = trim(opt(delimited(tag("("), take_until1(")"), tag(")"))))(input)?;

    if !input.is_empty() {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    // 4. exes (split by `space`)
    let (remain, _) = tag(":")(remain)?;
    let mut exes = vec![];
    let input = parse_exe(remain, &mut exes)
        .map_err(|_| nom::Err::Error(nom::error::Error::new(remain, nom::error::ErrorKind::Tag)))?;
    Ok((
        input,
        (
            name.to_string(),
            InstalledItem {
                version: version.to_string(),
                path: path.map(|s| s.to_string()),
                exes,
            },
        ),
    ))
}

fn parse_exe<'a>(input: &'a str, exes: &mut Vec<String>) -> Result<&'a str, Error> {
    if input.is_empty() {
        return Ok(input);
    }
    match line(input) {
        Ok(_) => Ok(input),
        Err(_) => {
            let (remain, exe) = trim(parse_name)(input).map_err(|e| Error::from(e.to_string()))?;
            exes.push(exe.to_string());
            parse_exe(remain, exes)
        }
    }
}

#[allow(unused)]
fn find_separator(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until(":")(input)?;
    // 向前查看，确保这个 : 不在括号内
    let mut paren_count = 0;
    for c in input.chars() {
        match c {
            '(' => paren_count += 1,
            ')' => paren_count -= 1,
            ':' if paren_count == 0 => break,
            _ => continue,
        }
    }
    if paren_count == 0 {
        tag(":")(input)
    } else {
        // 如果在括号内，继续寻找下一个 :
        let (remain, _) = take_until(":")(input)?;
        find_separator(remain)
    }
}

// 辅助函数：在考虑括号平衡的情况下寻找分隔符
fn take_until_unbalanced<'a>(
    open: char,
    close: char,
    delimiter: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        if input.is_empty() {
            return Ok(("", input));
        }

        let mut paren_count = 0;
        let mut last_pos = 0;

        for (idx, c) in input.char_indices() {
            match c {
                c if c == open => paren_count += 1,
                c if c == close => paren_count -= 1,
                _ if paren_count == 0 && input[idx..].starts_with(delimiter) => {
                    return Ok((&input[idx..], &input[..idx]))
                }
                _ => {}
            }
            last_pos = idx;
        }

        if last_pos == input.len() - 1 {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TakeUntil,
            )))
        } else {
            Ok((&input[last_pos + 1..], &input[..last_pos + 1]))
        }
    }
}

fn parse(input: &str) -> IResult<&str, HashMap<String, InstalledItem>> {
    let (input, items) = many0(trim(line))(input)?;
    if !input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    } else {
        return Ok((input, items.into_iter().collect()));
    }
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alpha1,
        take_while_m_n(0, usize::MAX, |c: char| {
            c == '_' || c == '-' || c.is_alphanumeric()
        }),
    ))(input)
}

/// format: `v0.1.2`
fn parse_version(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        tag("v"),
        take_while_m_n(0, usize::MAX, |c: char| c == '.' || c.is_numeric()),
    ))(input)
}

#[allow(unused_mut)]
fn trim<'a, P, O>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, parser, multispace0)
}

