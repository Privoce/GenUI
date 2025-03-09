use crate::try_from_enum_one_leaf;
use gen_utils::{err_from_to, error::Error};
use toml_edit::Formatted;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Themes {
    Dark,
    #[default]
    Primary,
    Error,
    Warning,
    Success,
    Info,
}

try_from_enum_one_leaf! {
    Themes, "Themes",
    Themes::Dark = "Dark",
    Themes::Primary = "Primary",
    Themes::Error = "Error",
    Themes::Warning = "Warning",
    Themes::Success = "Success",
    Themes::Info = "Info"
}

impl From<&Themes> for toml_edit::Value {
    fn from(value: &Themes) -> Self {
        let v = match value {
            Themes::Dark => "Dark".to_string(),
            Themes::Primary => "Primary".to_string(),
            Themes::Error => "Error".to_string(),
            Themes::Warning => "Warning".to_string(),
            Themes::Success => "Success".to_string(),
            Themes::Info => "Info".to_string(),
        };

        toml_edit::Value::String(Formatted::new(v))
    }
}

impl TryFrom<&toml_edit::Value> for Themes {
    type Error = Error;

    fn try_from(
        value: &toml_edit::Value,
    ) -> Result<Self, <Self as TryFrom<&toml_edit::Value>>::Error> {
        value.as_str().map_or_else(
            || Err(err_from_to!("toml_edit::Item" => "Themes")),
            |s| s.parse(),
        )
    }
}

#[cfg(test)]
mod test_theme {

    use gen_analyzer::value::Value;
    use quote::ToTokens;

    use crate::builtin::prop::Themes;

    #[test]
    fn test1() {
        let v1 = Value::String("Dark".to_string());
        let v2 = Value::String("Primary".to_string());
        let t1 = Themes::try_from(&v1).unwrap();
        let t2 = Themes::try_from(&v2).unwrap();
        dbg!(t1.to_token_stream().to_string());
        dbg!(t2.to_token_stream().to_string());
    }
}
