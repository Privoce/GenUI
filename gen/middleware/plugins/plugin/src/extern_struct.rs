use quote::ToTokens;
use syn::Macro;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MacroContext {
    /// macro identifier
    pub ident: String,
    /// macro tokens
    pub tokens: String,
}

impl MacroContext {
    pub fn new(ident: &str, tokens: &str) -> Self {
        Self {
            ident: ident.to_string(),
            tokens: tokens.to_string(),
        }
    }
    pub fn to_string_struct() -> String {
        r#"
#[repr(C)]
pub struct MacroContext {
    pub ident: String,
    pub tokens: String,
}
       "#
        .to_string()
    }
    pub fn to_string_param() -> String {
        "mac: &mut MacroContext".to_string()
    }
}

impl From<Macro> for MacroContext {
    fn from(value: Macro) -> Self {
        Self {
            ident: value.path.to_token_stream().to_string(),
            tokens: value.tokens.to_string(),
        }
    }
}
