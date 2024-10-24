#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub text: String,
    pub value: String,
}

impl SelectOption {
    pub fn new(text: &str, value: &str) -> Self {
        Self {
            text: text.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<(&str, &str)> for SelectOption {
    fn from(value: (&str, &str)) -> Self {
        Self::new(value.0, value.1)
    }
}
