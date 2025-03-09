use std::{fmt::Display, str::FromStr};

use gen_utils::{err_from_to, error::Error};

/// convert string to vector
pub fn convert_str_to_vec<T, E>(s: &str) -> Result<Vec<T>, Error>
where
    E: Display,
    T: FromStr<Err = E>,
{
    let mut res = Vec::new();
    for item in s.split(' ') {
        let item = item
            .parse::<T>()
            .map_err(|e| Error::FromDynError(e.to_string()))?;
        res.push(item);
    }

    Ok(res)
}

pub fn err_field(field: &str) -> Error {
    Error::FromDynError(format!("can not find target field: {} in Struct", field))
}

pub fn err_field_type(field: &str, target: &str) -> Error {
    Error::FromDynError(format!(
        "can not find target type field: {} in Struct, expect type: {}",
        field, target
    ))
}

pub fn item_bool(item: &toml_edit::Item) -> Result<bool, Error> {
    item.as_bool()
        .map_or_else(|| Err(err_from_to!("toml_edit::Item" => "bool")), |b| Ok(b))
}
pub fn value_bool(value: &toml_edit::Value) -> Result<bool, Error> {
    value.as_bool().map_or_else(
        || Err(err_from_to!("toml_edit::Value" => "bool")),
        |b| Ok(b),
    )
}
