pub mod inline;
pub mod offline;
pub mod position;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Comments {
    /// `//`
    Normal(String),
    /// `///`
    Document(String),
    /// `//!`
    File(String),
}

#[allow(dead_code)]
impl Comments {
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal(_))
    }
    pub fn is_document(&self) -> bool {
        matches!(self, Self::Document(_))
    }
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }
}

impl Default for Comments {
    fn default() -> Self {
        Comments::Normal(String::new())
    }
}

impl From<&str> for Comments {
    fn from(value: &str) -> Self {
        Comments::Normal(value.to_string())
    }
}
impl From<(&str, &str)> for Comments {
    fn from(value: (&str, &str)) -> Self {
        match value.0 {
            "//" => Comments::Normal(value.1.to_owned()),
            "///" => Comments::Document(value.1.to_owned()),
            "//!" => Comments::File(value.1.to_owned()),
            _ => panic!("Invalid comment"),
        }
    }
}

impl Display for Comments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Comments::Normal(n) => format!("// {}", n),
            Comments::Document(d) => format!("/// {}", d),
            Comments::File(f) => format!("//! {}", f),
        };
        f.write_str(res.as_str())
    }
}

#[cfg(test)]
mod test_comments {
    use super::Comments;

    #[test]
    fn display() {
        let c1 = Comments::Document("hello".to_string());
        let c2 = Comments::File("hello".to_string());
        let c3 = Comments::Normal("hello".to_string());

        assert_eq!(c1.to_string().as_str(), "/// hello");
        assert_eq!(c2.to_string().as_str(), "//! hello");
        assert_eq!(c3.to_string().as_str(), "// hello");
    }
}
