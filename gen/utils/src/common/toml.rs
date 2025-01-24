use std::{fmt::Display, path::Path};

use toml_edit::DocumentMut;

use crate::error::{Error, ParseError, ParseType};

use super::fs;

pub trait ToToml: Display {
    /// ## read and parse to DocumentMut from toml file
    fn read<P>(path: P) -> Result<DocumentMut, Error>
    where
        P: AsRef<Path>,
    {
        fs::read(path).and_then(|content| {
            content
                .parse::<DocumentMut>()
                .map_err(|e| Error::Parse(ParseError::new(e.to_string().as_str(), ParseType::Toml)))
        })
    }
    /// ## write to toml file
    fn write<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        fs::write(path, &self.to_string())
    }
    /// ## convert to DocumentMut
    fn to_toml(&self) -> DocumentMut;
}

pub fn read_to_doc<P>(path: P) -> Result<DocumentMut, Error>
where
    P: AsRef<Path>,
{
    fs::read(path.as_ref())?
        .parse::<DocumentMut>()
        .map_err(|e| Error::from(e.to_string()))
}
