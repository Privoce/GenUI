use std::fmt::Display;

/// # Parse Error
/// common error for parse, if any you need to parse, but failed, you can use this error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub target: String,
    pub other: Option<String>,
    pub ty: ParseType,
}

#[derive(Debug, Clone)]
pub enum ParseType {
    RustDep,
    Template,
    DSLBind,
    Toml,
    Color(String),
    Conf,
    Other(String),
}

impl ParseError {
    pub fn new(target: &str, ty: ParseType) -> Self {
        Self {
            target: target.to_string(),
            other: None,
            ty,
        }
    }
    pub fn rust_dep(target: &str) -> Self {
        Self {
            target: target.to_string(),
            other: None,
            ty: ParseType::RustDep,
        }
    }
    pub fn set_other(&mut self, other: &str) -> &mut Self {
        self.other = Some(other.to_string());
        self
    }
    pub fn set_ty(&mut self, ty: ParseType) -> &mut Self {
        self.ty = ty;
        self
    }
    pub fn template(target: &str) -> Self {
        Self {
            target: target.to_string(),
            other: None,
            ty: ParseType::Template,
        }
    }
    pub fn other(target: &str, ty: &str) -> Self {
        Self {
            target: target.to_string(),
            other: None,
            ty: ParseType::Other(ty.to_string()),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match &self.ty {
            ParseType::RustDep => "Rust Dependence".to_string(),
            ParseType::Template => "DSL Template".to_string(),
            ParseType::Other(s) => s.to_string(),
            ParseType::DSLBind => "DSL Bind Value".to_string(),
            ParseType::Color(s) => format!("Color-{}", s),
            ParseType::Conf => "Toml Config".to_string(),
            ParseType::Toml => "Toml".to_string(),
        };
        let mut fmt_out = format!("GenUI Parse {} Error:\ntarget: {}", ty, self.target);

        if let Some(other) = self.other.as_ref() {
            fmt_out.push_str(&format!("\nother msg: {}", other));
        }

        f.write_str(&fmt_out)
    }
}
