use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroError {
    PropImplTo,
    LiveRustConflict
}

impl Error for MacroError {}

impl Display for MacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroError::PropImplTo => f.write_str("Prop Macro can only be derived for struct!"),
            MacroError::LiveRustConflict => f.write_str("#[live] and #[rust] can not both exist in the same field!"),
        }
    }
}


// panic macro for the error
pub fn panic(e: MacroError) -> !{
    panic!("GenUI Macro: {}", e.to_string())
}