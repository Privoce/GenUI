use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

use gen_utils::error::{Error, ParseError, ParseType};
use gen_utils::parser::trim;
use nom::branch::alt;
use nom::bytes::complete::{take_while, take_while_m_n};
use nom::character::complete::{alphanumeric1, char, multispace0};
use nom::combinator::{map, recognize};
use nom::multi::{many1, separated_list0};
use nom::sequence::{pair, preceded, separated_pair};
use nom::{bytes::complete::tag, sequence::delimited, IResult};

/// # Bind Value
/// - in template: `:bind="A"` A is a bind ident
/// - in style: `$A`
/// ## For Bind
/// in template: `:for="(index, item) in iter_ident"` See [For]
/// ## Test
/// See [test_bind](tests/src/parser/value/bind.rs)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Bind {
    // normal is a bind ident, maybe it bind a ident or a closure ident
    Normal(Vec<Ident>),
    For(For),
}

impl Bind {
    pub fn ident(&self) -> String {
        match self {
            Bind::Normal(n) => n[0].to_string(),
            Bind::For(f) => f.iter_ident[0].to_string(),
        }
    }
    pub fn is(&self, s: &str) -> bool {
        match self {
            Bind::Normal(n) => n[0].name == s,
            Bind::For(f) => f.iter_ident[0].name == s,
        }
    }
    pub fn get_for(&self) -> Option<&For> {
        match self {
            Bind::For(f) => Some(f),
            _ => None,
        }
    }
    pub fn get_normal(&self) -> Option<&Vec<Ident>> {
        match self {
            Bind::Normal(n) => Some(&n),
            _ => None,
        }
    }
    pub fn parse_style(s: &str) -> Result<Self, Error> {
        let s = s.trim();
        if s.starts_with('$') {
            let s = s.trim_matches(|c| c == '$');
            return Ok(Bind::Normal(Ident::parse_idents(s)?));
        }
        return Err(ParseError::template(&format!("parse style bind: {} failed", s)).into());
    }
    pub fn parse_template(s: &str) -> Result<Self, Error> {
        s.trim().parse::<Bind>()
    }
}

impl Display for Bind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bind::Normal(n) => f.write_str(&n.iter().map(|i| i.to_string()).collect::<String>()),
            Bind::For(for_bind) => for_bind.fmt(f),
        }
    }
}

impl FromStr for Bind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<For>() {
            Ok(f) => {
                return Ok(Bind::For(f));
            }
            Err(_) => {
                if let Ok(normal) = Ident::parse_idents(s) {
                    return Ok(Bind::Normal(normal));
                }

                return Err(ParseError::template(&format!("parse bind: {} failed", s)).into());
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
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct For {
    pub iter_ident: Vec<Ident>,
    pub index: Option<String>,
    // pub item: Option<String>,
    pub item: ForItem,
}

impl For {
    pub fn ident(&self) -> String {
        self.iter_ident
            .first()
            .expect("iter_ident is empty")
            .name
            .to_string()
    }
    /// 是否使用了索引
    pub fn is_use_index(&self, s: &str) -> bool {
        s.contains(&self.fmt_index())
    }
    /// 是否使用了item
    /// 这里就不能简单使用contains进行判断了，例如
    /// 1. (index, item), item.0, item.1 | item在使用的时候可能会被拆分
    /// 2. (index, (item1, item2)), item1, item2 | 这种就比较好判断了
    /// 3. (index, (item1, item2)), do_fn(item1) | 表达式
    pub fn is_use_item(&self, s: &str) -> bool {
        match &self.item {
            ForItem::Ident(i) => s.contains(i),
            ForItem::Tuple(vec) => vec.iter().any(|item| item.is_use(s)),
            _ => false,
        }
    }
    pub fn iter_ident_as_fn(&self) -> String {
        self.iter_ident
            .iter()
            .map(|i| i.name.to_string())
            .collect::<Vec<String>>()
            .join("_")
    }
    pub fn fmt_iter_ident(&self) -> String {
        self.iter_ident
            .iter()
            .map(|i| i.to_string())
            .collect::<String>()
    }
    pub fn fmt_item_clone_tk(&self) -> String {
        self.item.item_clone()
    }
    pub fn fmt_enumerate(&self) -> String {
        format!("({}, {})", self.fmt_index(), self.fmt_item())
    }
    pub fn fmt_index(&self) -> String {
        self.index
            .as_ref()
            .unwrap_or(&"index".to_string())
            .to_string()
    }
    pub fn fmt_item(&self) -> String {
        self.item.to_string()
    }
    pub fn parser(input: &str) -> IResult<&str, Self> {
        /// 解析index 和 item部分
        fn index_item(input: &str) -> IResult<&str, (Option<String>, ForItem)> {
            // 这里由于和ForItem的解析逻辑是一样的，所以直接调用ForItem的解析方法，然后根据input中是否start_with`(`得到index
            let has_index = input.starts_with("(");
            let (input, item) = ForItem::parser(input)?;
            let (index, item) = if has_index {
                match &item {
                    ForItem::Tuple(vec) => {
                        // vec如果0或1个元素，那么这个元素就是item,否则第一个元素是index，后面的是item
                        match vec.len() {
                            0 | 1 => (None, item),
                            _ => {
                                let index = Some(vec[0].to_string());
                                let item = ForItem::Tuple(vec[1..].to_vec());
                                (index, item)
                            }
                        }
                    }
                    _ => (None, item),
                }
            } else {
                (None, item)
            };

            Ok((input, (index, item)))
        }

        let (input, ((index, item), idents)) =
            separated_pair(trim(index_item), trim(tag("in")), trim(many1(Ident::parse)))(input)?;

        if !input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }

        return Ok((
            input,
            Self {
                iter_ident: idents
                    .iter()
                    .map(|(ident, split)| Ident {
                        name: ident.to_string(),
                        split: *split,
                    })
                    .collect(),
                index,
                item,
            },
        ));
    }
}

impl FromStr for For {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        For::parser(s)
            .map_err(|e| {
                let mut err = ParseError::new(s, ParseType::DSLBind);
                let _ = err.set_other(e.to_string().as_str());
                err.into()
            })
            .map(|(_, f)| f)
    }
}

impl Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({index}, {item}) in {iter_ident}",
            index = self.index.as_ref().unwrap_or(&"index".to_string()),
            item = self.item,
            iter_ident = self
                .iter_ident
                .iter()
                .map(|i| i.to_string())
                .collect::<String>()
        )
    }
}

/// # For Loop Item
/// - `:for="(index, item) in iter_ident"` => `ForItem::Ident("item".to_string())`
/// - `:for="(index, (item1, item2, ...)) in iter_ident"` => `ForItem::Tuple(vec![
///     ForItem::Ident("item1".to_string()),
///     ForItem::Ident("item2".to_string()),
///     ...
/// ])`
/// - `:for="(index, ()) in iter_ident"` => `ForItem::Tuple(vec![])`
/// - `:for="(index, _) in iter_ident"` => `ForItem::None`
/// - `:for="item in iter_ident"` => `ForItem::Ident("item".to_string())`
/// - `:for="(index, (item1, (item2, item3), item4)) in iter_ident"` => `ForItem::Tuple(vec![
///     ForItem::Ident("item1".to_string()),
///     ForItem::Tuple(vec![
///         ForItem::Ident("item2".to_string()),
///         ForItem::Ident("item3".to_string()),
///     ]),
///     ForItem::Ident("item4".to_string()),
/// ])`
/// - `:for="(index, (item1, ..)) in iter_ident"` => `ForItem::Tuple(vec![
///     ForItem::Ident("item1".to_string()),
///     ForItem::More("".to_string()),
/// ])`
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ForItem {
    Tuple(Vec<ForItem>),
    Ident(String),
    /// `..`
    More,
    None,
}

impl Default for ForItem {
    fn default() -> Self {
        ForItem::Ident("item".to_string())
    }
}

impl Display for ForItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForItem::Tuple(items) => match items.len() {
                0 => f.write_str("()"),
                1 => items[0].fmt(f),
                _ => f.write_fmt(format_args!(
                    "({})",
                    items
                        .iter()
                        .map(|item| { item.to_string() })
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
            },
            ForItem::Ident(ident) => write!(f, "{}", ident),
            ForItem::None => write!(f, "_"),
            ForItem::More => write!(f, ".."),
        }
    }
}

impl ForItem {
    pub fn is_use(&self, s: &str) -> bool {
        match self {
            ForItem::Ident(i) => s.contains(i),
            ForItem::Tuple(vec) => vec.iter().any(|item| item.is_use(s)),
            _ => false,
        }
    }
    pub fn item_clone(&self) -> String {
        match self {
            ForItem::Tuple(vec) => vec.iter().fold(String::new(), |mut tk, item| {
                tk.push_str(&item.item_clone());
                tk
            }),
            ForItem::Ident(i) => {
                format!("let {} = {}.clone();", i, i)
            }
            ForItem::More => "..".to_string(),
            ForItem::None => "_".to_string(),
        }
    }

    pub fn parser(input: &str) -> IResult<&str, ForItem> {
        /// 匹配标识符
        fn ident(input: &str) -> IResult<&str, String> {
            let is_ident_char = |c: char| c.is_alphanumeric() || c == '_';
            map(take_while(is_ident_char), |s: &str| s.to_string())(input)
        }

        /// 匹配 ".."
        fn more(input: &str) -> IResult<&str, ForItem> {
            map(tag(".."), |_| ForItem::More)(input)
        }

        /// 匹配 "_"
        fn none(input: &str) -> IResult<&str, ForItem> {
            map(tag("_"), |_| ForItem::None)(input)
        }

        /// 匹配 `ForItem::Ident`
        fn for_ident(input: &str) -> IResult<&str, ForItem> {
            map(ident, ForItem::Ident)(input)
        }

        /// 匹配 `ForItem::Tuple`
        fn for_tuple(input: &str) -> IResult<&str, ForItem> {
            let inner = separated_list0(
                preceded(multispace0, char(',')),
                preceded(multispace0, for_item),
            );
            let parser = delimited(char('('), inner, char(')'));
            map(parser, ForItem::Tuple)(input)
        }

        /// 匹配完整的 ForItem
        fn for_item(input: &str) -> IResult<&str, ForItem> {
            alt((for_tuple, none, more, for_ident))(input)
        }

        for_item(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum IdentSplit {
    None,
    Dot,
    Holder,
}

impl IdentSplit {
    pub fn is_none(&self) -> bool {
        matches!(self, IdentSplit::None)
    }
    pub fn is_dot(&self) -> bool {
        matches!(self, IdentSplit::Dot)
    }
    pub fn is_holder(&self) -> bool {
        matches!(self, IdentSplit::Holder)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: String,
    /// 分割符: `[]`, `.` 两种
    pub split: IdentSplit,
}

impl Ident {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            split: IdentSplit::None,
        }
    }
    pub fn dot(name: &str) -> Self {
        Self {
            name: name.to_string(),
            split: IdentSplit::Dot,
        }
    }
    pub fn holder(name: &str) -> Self {
        Self {
            name: name.to_string(),
            split: IdentSplit::Holder,
        }
    }
    /// 解析带有层级调用(具有间隔`.`的标识符)的字段，用于解析类似`a.b.c`的标识符 => vec![Ident("a"), Ident("b"), Ident("c")]
    /// 还有例如`a[0][1]`的标识符 =>
    pub fn parse_idents(input: &str) -> Result<Vec<Self>, Error> {
        let (remain, idents) = many1(Self::parse)(input).map_err(|e| Error::from(e.to_string()))?;

        if remain.is_empty() {
            return Ok(idents
                .iter()
                .map(|(ident, split)| Ident {
                    name: ident.to_string(),
                    split: *split,
                })
                .collect());
        } else {
            return Err(ParseError::template(&format!(
                "parse idents: {} failed, still remain: {}",
                input, remain
            ))
            .into());
        }
    }
    pub fn parser(input: &str) -> Result<Self, Error> {
        let (remain, (ident, split)) =
            Self::parse(input).map_err(|e| Error::from(e.to_string()))?;
        if remain.is_empty() {
            return Ok(Ident {
                name: ident.to_string(),
                split,
            });
        } else {
            return Err(ParseError::template(&format!(
                "parse ident: {} failed, still remain: {}",
                input, remain
            ))
            .into());
        }
    }
    pub fn ident(&self) -> String {
        self.name.to_string()
    }

    /// 解析全部
    fn parse(s: &str) -> IResult<&str, (&str, IdentSplit)> {
        /// 以`.`分割的标识符
        fn dot(s: &str) -> IResult<&str, (&str, IdentSplit)> {
            let (remain, ident) = preceded(tag("."), normal)(s)?;

            Ok((remain, (ident, IdentSplit::Dot)))
        }
        /// 以`[]`包裹的标识符
        fn holder(s: &str) -> IResult<&str, (&str, IdentSplit)> {
            let (remain, ident) = delimited(tag("["), normal, tag("]"))(s)?;

            Ok((remain, (ident, IdentSplit::Holder)))
        }
        /// 没有分割符的标识符
        fn normal(s: &str) -> IResult<&str, &str> {
            recognize(pair(
                alphanumeric1,
                take_while_m_n(0, usize::MAX, |c: char| c == '_' || c.is_alphanumeric()),
            ))(s)
        }

        fn ident(s: &str) -> IResult<&str, (&str, IdentSplit)> {
            let (remain, ident) = recognize(pair(
                alphanumeric1,
                take_while_m_n(0, usize::MAX, |c: char| c == '_' || c.is_alphanumeric()),
            ))(s)?;

            Ok((remain, (ident, IdentSplit::None)))
        }

        alt((dot, holder, ident))(s)
    }
}

impl FromStr for Ident {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser(s)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.split {
            IdentSplit::None => write!(f, "{}", self.name),
            IdentSplit::Dot => write!(f, ".{}", self.name),
            IdentSplit::Holder => write!(f, "[{}]", self.name),
        }
    }
}
