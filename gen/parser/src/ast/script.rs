use std::fmt::Display;

use gen_utils::error::{Error, ParseError};
use quote::quote;
use syn::Block;

use crate::target::parse_script;

/// # Script
/// which is from `.gen` file, in `.gen` file, people can write rust code or ets code
/// - `<script lang="rust">` or `<script>` is rust code (default is rust code for makepad framework)
/// - ~`<script lang="ets">` is ets code (ets is now for ark HarmonyOs)~
/// ---
/// if is rust code use Block to store, otherwise use String to store
#[derive(Debug, Clone, PartialEq)]
pub enum Script {
    /// rust code
    Rs(Block),
    Other {
        lang: String,
        code: String,
    },
}

#[allow(dead_code)]
impl Script {
    /// is current script is empty or not
    pub fn is_empty(&self) -> bool {
        match self {
            Script::Rs(block) => block.stmts.is_empty(),
            Script::Other { code, .. } => code.is_empty(),
        }
    }
}

impl From<Block> for Script {
    fn from(value: Block) -> Self {
        Script::Rs(value)
    }
}

impl TryFrom<(&str, Option<String>)> for Script {
    type Error = Error;

    fn try_from(value: (&str, Option<String>)) -> Result<Self, Self::Error> {
        match value.1.as_ref() {
            Some(lang) => match lang.as_str() {
                "rust" | "rs" => {
                    let code =
                        parse_script(value.0)?;
                    Ok(Script::Rs(code))
                }
                other => Ok(Script::Other {
                    lang: other.to_string(),
                    code: value.0.to_string(),
                }),
            },
            None => Err(ParseError::template("the tag must be script, current is not").into()),
        }
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Script::Rs(rs) => {
                // if is rust code use quote to format
                let res = quote! {
                    #rs
                };
                // remove `{}`
                let convert_str = res.to_string();
                let convert_str = &convert_str[1..convert_str.len() - 1];
                f.write_str(convert_str.trim())
            }
            Script::Other { code, .. } => f.write_str(code),
        }
    }
}
