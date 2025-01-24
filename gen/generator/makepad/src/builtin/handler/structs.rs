use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use gen_parser::Value;
use gen_utils::error::Error;

use crate::builtin::prop::{err_field, err_from_to};

pub struct StructHandler<'a> {
    value: &'a Value,
}

impl<'a> StructHandler<'a> {
    pub fn new<'b>(fields: &'b BTreeMap<String, Value>, field: &str) -> Result<Self, Error>
    where
        'b: 'a,
    {
        if let Some(v) = fields.get(field) {
            return Ok(StructHandler { value: v });
        } else {
            return Err(err_field(field));
        }
    }
    ///  handle struct field for Value::Struct, only for string field
    pub fn str(&self) -> Result<&str, Error> {
        if let Value::String(s) = &self.value {
            return Ok(s);
        }

        Err(err_from_to("Value", "String").into())
    }

    pub fn usize(&self) -> Result<usize, Error> {
        if let Value::USize(n) = self.value {
            return Ok(*n);
        }
        Err(err_from_to("Value", "Number::USize").into())
    }

    pub fn isize(&self) -> Result<isize, Error> {
        if let Value::ISize(n) = self.value {
            return Ok(*n);
        }
        Err(err_from_to("Value", "Number::ISize").into())
    }

    pub fn f64(&self) -> Result<f64, Error> {
        if let Value::Double(n) = self.value {
            return Ok(*n);
        }
        Err(err_from_to("Value", "Number::F64").into())
    }

    pub fn bool(&self) -> Result<bool, Error> {
        if let Value::Bool(b) = self.value {
            return Ok(*b);
        }
        Err(err_from_to("Value", "Bool").into())
    }
}

pub fn handle_struct_field<F, T, E>(
    fields: &BTreeMap<String, Value>,
    field: &str,
    f: F,
) -> Result<T, Error>
where
    E: Display,
    T: FromStr<Err = E>,
    F: FnOnce(&Value) -> Result<T, Error>,
{
    fields
        .get(field)
        .map_or_else(|| Err(err_field(field)), |v| f(v))
}
