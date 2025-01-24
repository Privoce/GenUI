mod color;
/// comment parsers
mod comment;
#[cfg(feature = "makepad")]
mod shader;
mod tag;
mod special;

pub use color::*;
pub use comment::parse_comment;
pub use tag::{end, parse_all, until_end};
pub use special::Special;
pub use shader::MakepadShader;