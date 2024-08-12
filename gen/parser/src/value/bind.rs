use std::fmt::Display;
use std::str::FromStr;

use gen_utils::error::Errors;
use gen_utils::parser::{parse_normal, trim};
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::alphanumeric1;
use nom::combinator::recognize;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, sequence::delimited, IResult};

#[derive(Debug, PartialEq, Clone)]
pub enum Bind {
    // normal is a bind ident, maybe it bind a ident or a closure ident
    Normal(String),
    For(For),
}

impl Bind {
    pub fn get_for(&self) -> Option<&For> {
        match self {
            Bind::For(f) => Some(f),
            _ => None,
        }
    }
    pub fn get_normal(&self) -> Option<&String> {
        match self {
            Bind::Normal(n) => Some(n),
            _ => None,
        }
    }
}

impl Display for Bind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bind::Normal(n) => f.write_str(n),
            Bind::For(for_bind) => for_bind.fmt(f),
        }
    }
}

impl FromStr for Bind {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<For>() {
            Ok(f) => {
                return Ok(Bind::For(f));
            }
            Err(_) => {
                return Ok(Bind::Normal(s.to_string()));
            }
        }
    }
}

/// # For loop bind
/// ## format:
/// 1. `:for="(index, item) in iter_ident"`
/// 2. `:for="item in iter_ident"`
/// 3. `:for="(index, (item1, item2, ...)) in iter_ident"`
/// 4. `:for="(index, ()) in iter_ident"`
/// 5. `:for="(index, _) in iter_ident"`
#[derive(Debug, PartialEq, Clone)]
pub struct For {
    pub iter_ident: String,
    pub index: Option<String>,
    pub item: Option<String>,
}

impl For {
    // todo!("这个方法后面需要移除，暂时写在这里，这个实际item到底默认是什么其实需要依靠iter_ident的类型来判断，但现在还没写好这个判断的地方")
    // 目前我没找到相关可以识别的包，可能需要自己写一个
    pub fn fmt_enumerate(&self) -> String {
        format!(
            "({}, {})",
            self.index.as_ref().unwrap_or(&"index".to_string()),
            self.item.as_ref().unwrap_or(&"item".to_string())
        )
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        fn parse_ident(input: &str) -> IResult<&str, &str> {
            parse_normal(input, '_')
        }
        fn ident(input: &str) -> IResult<&str, &str> {
            alt((parse_ident, tag("_"), alphanumeric1))(input)
        }
        fn index_item(input: &str) -> IResult<&str, Vec<&str>> {
            // here has `()`, and may nested `()`, but we do not care nested `()`, it will be set as item
            // and actually, it only has two options:
            // 1. only has one item, no index
            // 2. has two items, one is index, one is item (around by `()` also be treated as item)
            let (input, _) = tag("(")(input)?;
            // now check if contains `(`, if not, it is a normal ident, it's options 1
            if input.contains("(") {
                let mut res = vec![];
                let (input, index) = take_until("(")(input)?;
                // now we get the index, but we should trim it and check if it has `,`, if has, remove it
                let index = index.trim().trim_end_matches(',').trim();
                res.push(index);

                //    let (input, item) = delimited(trim(tag("(")), trim(take_until(")")), trim(tag(")")))(input)?;
                let (input, item) = recognize(delimited(
                    trim(tag("(")),
                    trim(take_until(")")),
                    trim(tag(")")),
                ))(input)?;
                res.push(item);
                let (input, _) = tag(")")(input)?;
                return Ok((input, res));
            } else {
                // it is a normal ident
                let (input, item) = separated_list0(tag(","), trim(ident))(input)?;
                let (input, _) = tag(")")(input)?;
                return Ok((input, item));
            }
        }

        fn only_item(input: &str) -> IResult<&str, Vec<&str>> {
            let (input, item) = trim(ident)(input)?;
            Ok((input, vec![item]))
        }

        let (input, (index_item, ident)) = separated_pair(
            trim(alt((index_item, only_item))),
            trim(tag("in")),
            trim(ident),
        )(input)?;

        if !input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }

        let (index, item) = match index_item.len() {
            1 => (None, Some(index_item[0].to_string())),
            2 => (
                Some(index_item[0].to_string()),
                Some(index_item[1].to_string()),
            ),
            _ => (
                Some(index_item[0].to_string()),
                Some(index_item[1..].join(",")),
            ),
        };

        return Ok((
            input,
            Self {
                iter_ident: ident.to_string(),
                index,
                item,
            },
        ));
    }
}

impl FromStr for For {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        For::parser(s)
            .map_err(|e| Errors::ParseError(e.to_string()))
            .map(|(_, f)| f)
    }
}

impl From<(String, Option<String>, Option<String>)> for For {
    fn from(value: (String, Option<String>, Option<String>)) -> Self {
        Self {
            iter_ident: value.0,
            index: value.1,
            item: value.2,
        }
    }
}

impl Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.index, &self.item) {
            (Some(index), Some(item)) => write!(
                f,
                "({index}, {item}) in {iter_ident}",
                index = index,
                item = item,
                iter_ident = self.iter_ident
            ),
            (None, Some(item)) => write!(
                f,
                "{item} in {iter_ident}",
                item = item,
                iter_ident = self.iter_ident
            ),
            (Some(index), None) => write!(
                f,
                "{index} in {iter_ident}",
                index = index,
                iter_ident = self.iter_ident
            ),
            _ => write!(f, "in {iter_ident}", iter_ident = self.iter_ident),
        }
    }
}

#[cfg(test)]
mod bind_test {
    use super::*;

    #[test]
    fn test_from_str() {
        let bind = Bind::from_str("item in iter_ident").unwrap();
        assert_eq!(
            bind,
            Bind::For(For {
                iter_ident: "iter_ident".to_string(),
                index: None,
                item: Some("item".to_string())
            })
        );
    }

    #[test]
    fn test_bind() {
        let (input, bind) = For::parser("(index, item) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, "iter_ident");
        assert_eq!(bind.index, Some("index".to_string()));
        assert_eq!(bind.item, Some("item".to_string()));

        let (input, bind) = For::parser("item in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, "iter_ident");
        assert_eq!(bind.index, None);
        assert_eq!(bind.item, Some("item".to_string()));

        let (input, bind) = For::parser("(item1, (item2, item3)) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, "iter_ident");
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(bind.item, Some("(item2, item3)".to_string()));

        let (input, bind) = For::parser("(item1, _) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, "iter_ident");
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(bind.item, Some("_".to_string()));

        let (input, bind) = For::parser("(item1, ()) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, "iter_ident");
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(bind.item, Some("()".to_string()));

        let (input, bind) = For::parser("(item1, (_, _)) in iter_ident").unwrap();
        assert_eq!(input, "");
        assert_eq!(bind.iter_ident, "iter_ident");
        assert_eq!(bind.index, Some("item1".to_string()));
        assert_eq!(bind.item, Some("(_, _)".to_string()));
    }
}
