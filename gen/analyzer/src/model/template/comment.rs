use std::fmt::Display;
use gen_utils::parser::trim;
use nom::{
    branch::alt, bytes::complete::{tag, take_while}, error::ErrorKind, IResult
};
use crate::nom_err;

#[derive(Debug, Clone)]
pub enum Comment {
    /// `//`
    Normal(String),
    /// `///`
    Document(String),
    /// `//!`
    File(String),
}

impl Comment {
    /// ## parse comment
    /// - //
    /// - ///
    /// - //!
    /// this method just can parse a comment and convert to CommentTypes
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, comment_type) = trim(alt((tag("//!"), tag("///"), tag("//"))))(input)?;
        let (input, comment) = take_while(|c: char| c != '\n')(input)?;
        match comment_type {
            "//" => Ok((input, Comment::Normal(comment.to_string()))),
            "///" => Ok((input, Comment::Document(comment.to_string()))),
            "//!" => Ok((input, Comment::File(comment.to_string()))),
            _ => Err(nom_err!(input, ErrorKind::Tag)),
        }
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comment::Normal(c) => write!(f, "// {}", c),
            Comment::Document(c) => write!(f, "/// {}", c),
            Comment::File(c) => write!(f, "//! {}", c),
        }
    }
}
// /// # CommentPosition
// /// Although in code we can not limit the position of the comment, we should follow the rules:
// /// - `Comment::File` and `Comment::Document` should be above the code
// /// - `Comment::Normal` can be above or inline with the code
// #[derive(Debug, Clone, PartialEq, Default)]
// pub enum CommentPosition {
//     /// Above the code:
//     /// ```
//     /// // above comment
//     /// <label text="'hello'"></label>
//     /// ```
//     #[default]
//     Above,
//     /// Inline with the code:
//     /// ```
//     /// <label text="'hello'"></label> // inline comment
//     /// ```
//     Inline,
// }

#[cfg(test)]
mod test_comments {
    use super::Comment;

    #[test]
    fn display() {
        let c1 = Comment::Document("hello".to_string());
        let c2 = Comment::File("hello".to_string());
        let c3 = Comment::Normal("hello".to_string());

        assert_eq!(c1.to_string().as_str(), "/// hello");
        assert_eq!(c2.to_string().as_str(), "//! hello");
        assert_eq!(c3.to_string().as_str(), "// hello");
    }
}
