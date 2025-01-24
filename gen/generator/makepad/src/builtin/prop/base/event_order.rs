use gen_utils::error::Error;
use toml_edit::Formatted;

use crate::try_from_enum_one_leaf;

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum EventOrder {
    Down,
    #[default]
    Up,
    // List(Vec<LiveId>),
}

try_from_enum_one_leaf! {
    EventOrder, "EventOrder",
    EventOrder::Down = "Down",
    EventOrder::Up = "Up"
}

impl TryFrom<&toml_edit::Value> for EventOrder {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_str().map_or_else(
            || Err(Error::from("toml_edit::Item to EventOrder")),
            |s| s.parse(),
        )
    }
}

impl From<&EventOrder> for toml_edit::Value {
    fn from(value: &EventOrder) -> Self {
        let v = match value {
            EventOrder::Down => "Down",
            EventOrder::Up => "Up",
        };

        toml_edit::Value::String(Formatted::new(v.to_string()))
    }
}
