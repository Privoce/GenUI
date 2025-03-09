mod convert;
mod env;
mod macros;
mod parse;
mod strategy;
mod style;
mod tag;

pub use convert::*;
pub use env::EnvError;
pub use parse::*;
mod compiler;
mod fs;
pub use compiler::CompilerError;
pub use fs::FsError;
use std::{error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Error {
    Parse(ParseError),
    Convert(ConvertError),
    FromDynError(String),
    Env(EnvError),
    // -------- compiler -----------
    Compiler(CompilerError),
    // -------- fs -----------
    Fs(FsError),
}

impl Error {
    pub fn is_runtime(&self) -> bool {
        match self {
            Error::Compiler(CompilerError::Runtime { .. }) => true,
            _ => false,
        }
    }
    pub fn to_runtime(self, target: &str) -> Self {
        let target = target.to_string();
        match self {
            Error::Parse(parse_error) => {
                let msg = parse_error.to_string();
                Self::Compiler(CompilerError::Runtime { target, msg })
            }
            Error::Convert(convert_error) => {
                let msg = convert_error.to_string();
                Self::Compiler(CompilerError::Runtime { target, msg })
            }
            Error::FromDynError(msg) => {
                let msg = msg.to_string();
                Self::Compiler(CompilerError::Runtime { target, msg })
            }
            Error::Env(env_error) => {
                let msg = env_error.to_string();
                Self::Compiler(CompilerError::Runtime { target, msg })
            }
            Error::Compiler(compiler_error) => match compiler_error {
                CompilerError::EnvCheck { .. } => Self::Compiler(CompilerError::Runtime {
                    target,
                    msg: compiler_error.to_string(),
                }),
                CompilerError::Runtime { .. } => Self::Compiler(compiler_error),
                CompilerError::Conf(_) => {
                    let msg = compiler_error.to_string();
                    Self::Compiler(CompilerError::Runtime { target, msg })
                }
            },
            Error::Fs(fs_error) => {
                let msg = fs_error.to_string();
                Self::Compiler(CompilerError::Runtime { target, msg })
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(parse_error) => parse_error.fmt(f),
            Error::Compiler(compiler_error) => compiler_error.fmt(f),
            Error::Fs(fs_error) => fs_error.fmt(f),
            Error::Convert(convert_error) => convert_error.fmt(f),
            Error::FromDynError(e) => f.write_str(e),
            Error::Env(env_error) => env_error.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<ConvertError> for Error {
    fn from(value: ConvertError) -> Self {
        Self::Convert(value)
    }
}

impl From<CompilerError> for Error {
    fn from(value: CompilerError) -> Self {
        Self::Compiler(value)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::FromDynError(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::FromDynError(e.to_string())
    }
}

impl From<FsError> for Error {
    fn from(value: FsError) -> Self {
        Self::Fs(value)
    }
}

impl From<EnvError> for Error {
    fn from(value: EnvError) -> Self {
        Self::Env(value)
    }
}
