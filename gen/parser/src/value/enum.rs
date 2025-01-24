use std::fmt::Display;

use gen_utils::{
    error::{Error, ParseError},
    parser::parse_value,
};

use super::Value;

/// # Enum Value
/// parse like:
/// 1. `TextAlign::Center::Place`-> `Enum{field_chain: [TextAlign, Center, Place]}`
/// 2. `TextAlign::Place(10)` -> `Enum{field_chain: [TextAlign, Place(10)]}`
/// 3. `TextAlign::Place{ x: 10 }` -> `Enum{field_chain: [TextAlign, Place]}`
/// ## Test
/// See [test_enum](tests/src/parser/value/enum.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub field_chain: Vec<EnumItem>,
}

impl Enum {
    pub fn leaf(&self) -> Option<&EnumItem> {
        self.field_chain.last()
    }
    pub fn is_anonymous(&self) -> bool {
        self.field_chain.len() == 1
    }
    pub fn parse_template(s: &str) -> Result<Self, Error>{
        Self::parse_style(s)
    }
    pub fn parse_style(s: &str) -> Result<Self, Error> {
        fn parse_leaf(s: &str) -> Result<EnumItem, Error> {
            let s = s.trim();
            match parse_value(s) {
                Ok((input, s)) => {
                    return if input.starts_with("(") && input.ends_with(")") {
                        let input = input.trim_matches(|c| c == '(' || c == ')');
                        Ok(EnumItem::Leaf(
                            s.to_string(),
                            Some(Value::parse_style(input)?),
                        ))
                    } else if input.starts_with("{") && input.ends_with("}") {
                        Ok(EnumItem::Leaf(
                            s.to_string(),
                            Some(Value::parse_style(input)?),
                        ))
                    } else {
                        if Value::check_unknown(s) {
                            return Ok(EnumItem::Leaf(s.to_string(), None));
                        }
                        return Err(ParseError::template("style Enum leaf parse failed").into());
                    };
                }
                Err(_) => Err(ParseError::template("style Enum leaf parse failed").into()),
            }
        }

        // split `::`
        if s.contains("::") {
            let mut field_chain = vec![];
            let enum_str = s.split("::").collect::<Vec<&str>>();
            for (index, item) in enum_str.iter().enumerate() {
                if index == 0 {
                    // root
                    field_chain.push(EnumItem::Root(item.to_string()));
                } else if index < enum_str.len() - 1 {
                    // branch
                    field_chain.push(EnumItem::Branch(item.to_string()));
                } else {
                    // leaf
                    field_chain.push(parse_leaf(item)?);
                }
            }
            return Ok(Enum { field_chain });
        } else {
            let leaf = parse_leaf(s)?;
            return Ok(Enum {
                field_chain: vec![leaf],
            });
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumItem {
    Root(String),
    Branch(String),
    Leaf(String, Option<Value>),
}

impl Display for EnumItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumItem::Root(s) => f.write_str(s),
            EnumItem::Branch(s) => f.write_str(s),
            EnumItem::Leaf(s, v) => {
                if let Some(v) = v {
                    if let Value::Struct(inner) = v {
                        f.write_fmt(format_args!("{}{}", s, inner))
                    } else {
                        f.write_fmt(format_args!("{}({})", s, v))
                    }
                } else {
                    f.write_str(s)
                }
            }
        }
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .field_chain
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("::"),
        )
    }
}