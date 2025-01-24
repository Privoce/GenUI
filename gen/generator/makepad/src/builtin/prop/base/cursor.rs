use gen_utils::error::Error;
use toml_edit::Formatted;

use crate::{builtin::prop::err_from_to, try_from_enum_one_leaf};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MouseCursor {
    Hidden,
    /// default
    #[default]
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    NotAllowed,
    NResize,
    NeResize,
    EResize,
    SeResize,
    SResize,
    SwResize,
    WResize,
    NwResize,
    NsResize,
    NeswResize,
    EwResize,
    NwseResize,
    ColResize,
    RowResize,
}

try_from_enum_one_leaf! {
    MouseCursor, "MouseCursor",
    MouseCursor::Hidden = "Hidden",
    MouseCursor::Default = "Default",
    MouseCursor::Crosshair = "Crosshair",
    MouseCursor::Hand = "Hand",
    MouseCursor::Arrow = "Arrow",
    MouseCursor::Move = "Move",
    MouseCursor::Text = "Text",
    MouseCursor::Wait = "Wait",
    MouseCursor::Help = "Help",
    MouseCursor::NotAllowed = "NotAllowed",
    MouseCursor::NResize = "NResize",
    MouseCursor::NeResize = "NeResize",
    MouseCursor::EResize = "EResize",
    MouseCursor::SeResize = "SeResize",
    MouseCursor::SResize = "SResize",
    MouseCursor::SwResize = "SwResize",
    MouseCursor::WResize = "WResize",
    MouseCursor::NwResize = "NwResize",
    MouseCursor::NsResize = "NsResize",
    MouseCursor::NeswResize = "NeswResize",
    MouseCursor::EwResize = "EwResize",
    MouseCursor::NwseResize = "NwseResize",
    MouseCursor::ColResize = "ColResize",
    MouseCursor::RowResize = "RowResize"
}

impl TryFrom<&toml_edit::Value> for MouseCursor {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_str().map_or_else(
            || Err(Error::from(err_from_to("toml_edit::Item", "MouseCursor"))),
            |s| s.parse(),
        )
    }
}

impl From<&MouseCursor> for toml_edit::Value {
    fn from(value: &MouseCursor) -> Self {
        let v = match value {
            MouseCursor::Hidden => "Hidden".to_string(),
            MouseCursor::Default => "Default".to_string(),
            MouseCursor::Crosshair => "Crosshair".to_string(),
            MouseCursor::Hand => "Hand".to_string(),
            MouseCursor::Arrow => "Arrow".to_string(),
            MouseCursor::Move => "Move".to_string(),
            MouseCursor::Text => "Text".to_string(),
            MouseCursor::Wait => "Wait".to_string(),
            MouseCursor::Help => "Help".to_string(),
            MouseCursor::NotAllowed => "NotAllowed".to_string(),
            MouseCursor::NResize => "NResize".to_string(),
            MouseCursor::NeResize => "NeResize".to_string(),
            MouseCursor::EResize => "EResize".to_string(),
            MouseCursor::SeResize => "SeResize".to_string(),
            MouseCursor::SResize => "SResize".to_string(),
            MouseCursor::SwResize => "SwResize".to_string(),
            MouseCursor::WResize => "WResize".to_string(),
            MouseCursor::NwResize => "NwResize".to_string(),
            MouseCursor::NsResize => "NsResize".to_string(),
            MouseCursor::NeswResize => "NeswResize".to_string(),
            MouseCursor::EwResize => "EwResize".to_string(),
            MouseCursor::NwseResize => "NwseResize".to_string(),
            MouseCursor::ColResize => "ColResize".to_string(),
            MouseCursor::RowResize => "RowResize".to_string(),
        };

        toml_edit::Value::String(Formatted::new(v))
    }
}