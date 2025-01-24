use gen_utils::error::Error;
use toml_edit::Formatted;

use crate::try_from_enum_one_leaf;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ViewOptimize {
    #[default]
    None,
    DrawList,
    Texture,
}

try_from_enum_one_leaf! {
    ViewOptimize, "ViewOptimize",
    ViewOptimize::None = "None",
    ViewOptimize::DrawList = "DrawList",
    ViewOptimize::Texture = "Texture"
}

impl From<&ViewOptimize> for toml_edit::Value {
    fn from(value: &ViewOptimize) -> Self {
        let v = match value {
            ViewOptimize::None => "None",
            ViewOptimize::DrawList => "DrawList",
            ViewOptimize::Texture => "Texture",
        };

        toml_edit::Value::String(Formatted::new(v.to_string()))
    }
}

impl TryFrom<&toml_edit::Value> for ViewOptimize {
    type Error = Error;

    fn try_from(value: &toml_edit::Value) -> Result<Self, Self::Error> {
        value.as_str().map_or_else(
            || Err(Error::from("toml_edit::Item to ViewOptimize")),
            |s| s.parse(),
        )
    }
}