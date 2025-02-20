use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Imports(pub Vec<Import>);

impl FromStr for Imports {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(|c| c == '{' || c == '}').trim();

        Ok(Self(
            s.split(';')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<Import>())
                .collect::<Result<Vec<Import>, Error>>()?,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Import(pub Vec<String>);

impl FromStr for Import {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split("::")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
        ))
    }
}
