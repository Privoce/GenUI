use crate::ast::comment::position::OfflinePosition;
use crate::ast::comment::Comments;
use crate::ast::Targets;
use gen_utils::common::tokenizer::{COMMENT_DOCUMENT, COMMENT_FILE, COMMENT_NROMAL};

use gen_utils::parser::trim;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::peek;
use nom::multi::many1;

use nom::{bytes::complete::tag, IResult};

use super::tag::parse_tag_check;


// pub fn parse_comment(input: &str) -> IResult<&str, TemplateASTNode> {
//     // let (input,value) = recognize(preceded(
//     //     alt((
//     //         tag(COMMENT_FILE),
//     //         tag(COMMENT_DOCUMENT),
//     //         tag(COMMENT_NROMAL),
//     //     )),
//     //     take_while(|c: char| c != '\n'),
//     // ))(input)?;
//     let (input, comment_type) = alt((
//         tag(COMMENT_FILE),
//         tag(COMMENT_DOCUMENT),
//         tag(COMMENT_NROMAL),
//     ))(input)?;
//     let (input, value) = take_while(|c: char| c != '\n')(input)?;

//     Ok((input, TemplateASTNode::comment(value, comment_type)))
// }

/// ## parse comment 🆗
/// - //
/// - ///
/// - //!
/// this method just can parse a comment and convert to Comments
pub fn parse_comment(input: &str) -> IResult<&str, Comments> {
    let (input, comment_type) = trim(alt((
        tag(COMMENT_FILE),
        tag(COMMENT_DOCUMENT),
        tag(COMMENT_NROMAL),
    )))(input)?;

    let (input, comment) = take_while(|c: char| c != '\n')(input)?;
    Ok((input, (comment_type, comment).into()))
}

/// # parse offline comment
/// this method should be used to parse offline comments to get ParseTarget
pub fn parse_offline_comment(input: &str) -> IResult<&str, Targets> {
    // tuple((parse_comment_value,peek(parse_template_tag)))(input)
    let (input, comment) = many1(parse_comment)(input)?;
    let mut is_end = false;
    // check template
    let targets = match peek(parse_tag_check)(input) {
        Ok((_, targets)) => Some(targets),
        Err(_) => {
            //向下无法找到任何标签，说明已经在底部了
            is_end = true;
            None
        }
    };
    let position = if is_end {
        OfflinePosition::End
    } else {
        match targets.unwrap() {
            Targets::Template(_) => OfflinePosition::AboveTemplate,
            Targets::Script { .. } => OfflinePosition::AboveScript,
            Targets::Style(_) => OfflinePosition::AboveStyle,
            Targets::Comment(_) => {
                panic!("not exist this condition: offline comment above offline comment")
            }
        }
    };

    Ok((input, Targets::Comment((comment, position).into())))
}
