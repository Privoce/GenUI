use std::{fmt::Display, str::FromStr};

use nom::{
    bytes::complete::{take_till1, take_while_m_n},
    character::complete::{alpha1, char, multispace0},
    combinator::recognize,
    sequence::{delimited, pair},
    IResult,
};

use crate::error::Error;

/// Author of the project
/// format: name <email>
#[derive(Debug, Clone)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

impl FromStr for Author {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s) {
            Ok((remain, author)) => {
                if remain.trim().is_empty() {
                    return Ok(author);
                } else {
                    return Err(format!("still have remain: {}", remain).into());
                }
            }
            Err(e) => Err(e.to_string().into()),
        }
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.email {
            Some(email) => write!(f, "{} <{}>", self.name, email),
            None => write!(f, "{}", self.name),
        }
    }
}


fn parse(input: &str) -> IResult<&str, Author> {
    let (input, name) = trim(parse_value)(input)?;

    let (input, email) = if !input.trim().is_empty() {
        let (input, email) = delimited(char('<'), take_till1(|c| c == '>'), char('>'))(input)?;
        (input, Some(email))
    } else {
        (input, None)
    };

    Ok((
        input,
        Author {
            name: name.to_string(),
            email: email.map(|x| x.to_string()),
        },
    ))
}

// ----------------------------------------------------------------------------------------------------------
// ------------------ copy from utils/src/parser/normal.rs --------------------------------------------------
// ----------------------------------------------------------------------------------------------------------
fn parse_normal(input: &str, sign: char) -> IResult<&str, &str> {
    recognize(pair(
        alpha1,
        take_while_m_n(0, usize::MAX, |c: char| c == sign || c.is_alphanumeric()),
    ))(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

#[allow(unused_mut)]
fn trim<'a, P, O>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, parser, multispace0)
}
// ----------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test_author {
    use crate::compiler::Author;

    #[test]
    fn test_parse() {
        let input1 = "syf <syf@gmail.com>";
        let input2 = "syf";
        let input3 = "syf <";
        let input4 = "syf <syf@gmail.com> 22";
        let _res1 = input1.parse::<Author>();
        let _res2 = input2.parse::<Author>();
        let _res3 = input3.parse::<Author>();
        let _res4 = input4.parse::<Author>();

        // dbg!(res1);
        // dbg!(res2);
        // dbg!(res3);
        // dbg!(res4);
    }
}
