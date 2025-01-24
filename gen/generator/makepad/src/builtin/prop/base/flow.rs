use gen_utils::error::Error;
use toml_edit::Formatted;

use crate::try_from_enum_one_leaf;

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum Flow {
    #[default]
    Right,
    Down,
    Overlay,
    RightWrap,
}

try_from_enum_one_leaf! {
    Flow, "Flow",
    Flow::Right = "Right",
    Flow::Down = "Down",
    Flow::Overlay = "Overlay",
    Flow::RightWrap = "RightWrap"
}

impl From<&Flow> for toml_edit::Value {
    fn from(value: &Flow) -> Self {
        let v = match value {
            Flow::Right => "Right",
            Flow::Down => "Down",
            Flow::Overlay => "Overlay",
            Flow::RightWrap => "RightWrap",
        };

        toml_edit::Value::String(Formatted::new(v.to_string()))
    }
}

impl TryFrom<&toml_edit::Value> for Flow {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_str().map_or_else(
            || Err(Error::from("toml_edit::Item to Flow")),
            |s| s.parse(),
        )
    }
}
