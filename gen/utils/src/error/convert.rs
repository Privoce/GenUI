use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ConvertError {
    FromTo { from: String, to: String },
    Serde(String),
    ValueType(String),
    UnSupport(String),
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::FromTo { from, to } => {
                f.write_fmt(format_args!("Convert Error: From {} to {}", from, to))
            }
            ConvertError::Serde(s) => f.write_fmt(format_args!("Serde Convert Error: {}", s)),
            ConvertError::ValueType(s) => f.write_fmt(format_args!(
                "Value Type Convert Error: unsupport convert to `{}`",
                s
            )),
            ConvertError::UnSupport(s) => f.write_fmt(format_args!(
                "UnSupport Error: value type unsupport to use `{}`",
                s
            )),
        }
    }
}