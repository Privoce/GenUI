use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum EnvError {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    Empty
}

impl Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::Set { key, value } => {
                f.write_fmt(format_args!("Set Env Error: key: {}, value: {}", key, value))
            }
            EnvError::Get { key } => f.write_fmt(format_args!("Get Env Error: key: {}", key)),
            EnvError::Remove { key } => f.write_fmt(format_args!("Remove Env Error: key: {}", key)),
            EnvError::Empty => f.write_str("Env is empty"),
        }
    }
}