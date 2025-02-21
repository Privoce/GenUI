use gen_analyzer::{value::Value, PropKey};
use gen_utils::error::Error;

use crate::{
    builtin::prop::{err_from_to, value_bool, F64},
    props_to_tokens,
};

use super::{Align, DVec2, Flow, Padding};

#[derive(Debug, Clone)]
pub enum Layout {
    Scroll(DVec2),
    ClipX(bool),
    ClipY(bool),
    Padding(Padding),
    Align(Align),
    Flow(Flow),
    Spacing(F64),
}

impl Layout {
    pub fn to_toml_item_kv(&self) -> (String, toml_edit::Value) {
        match self {
            Layout::Scroll(v) => ("scroll".to_string(), v.into()),
            Layout::ClipX(v) => ("clip_x".to_string(), (*v).into()),
            Layout::ClipY(v) => ("clip_y".to_string(), (*v).into()),
            Layout::Padding(v) => ("padding".to_string(), v.into()),
            Layout::Align(v) => ("align".to_string(), v.into()),
            Layout::Flow(v) => ("flow".to_string(), v.into()),
            Layout::Spacing(v) => ("spacing".to_string(), v.into()),
        }
    }
}

impl TryFrom<(&str, &toml_edit::Value)> for Layout {
    type Error = Error;

    fn try_from(value: (&str, &toml_edit::Value)) -> Result<Self, Self::Error> {
        let (key, val) = value;
        match key {
            "scroll" => Ok(Layout::Scroll(DVec2::try_from(val)?)),
            "clip_x" => Ok(Layout::ClipX(value_bool(val)?)),
            "clip_y" => Ok(Layout::ClipY(value_bool(val)?)),
            "padding" => Ok(Layout::Padding(Padding::try_from(val)?)),
            "align" => Ok(Layout::Align(Align::try_from(val)?)),
            "flow" => Ok(Layout::Flow(Flow::try_from(val)?)),
            "spacing" => Ok(Layout::Spacing(val.try_into()?)),
            _ => {
                return Err(err_from_to(
                    "GenUI Props",
                    &format!("Makepad Layout, Invalid Prop: {}", key),
                )
                .into());
            }
        }
    }
}

impl TryFrom<&(PropKey, Value)> for Layout {
    type Error = Error;

    fn try_from(value: &(PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "scroll" => Ok(Layout::Scroll(DVec2::try_from(&value.1)?)),
            "clip_x" => Ok(Layout::ClipX(value.1.as_bool()?)),
            "clip_y" => Ok(Layout::ClipY(value.1.as_bool()?)),
            "padding" => Ok(Layout::Padding(Padding::try_from(&value.1)?)),
            "align" => Ok(Layout::Align(Align::try_from(&value.1)?)),
            "flow" => Ok(Layout::Flow(Flow::try_from(&value.1)?)),
            "spacing" => Ok(Layout::Spacing(value.1.as_f64()?.into())),
            _ => {
                return Err(err_from_to(
                    "GenUI Props",
                    &format!("Makepad Layout, Invalid Prop: {}", &value.0.name),
                )
                .into());
            }
        }
    }
}

props_to_tokens! {
    Layout,
    Layout::Scroll => scroll, false,
    Layout::ClipX => clip_x, false,
    Layout::ClipY => clip_y, false,
    Layout::Padding => padding, false,
    Layout::Align => align, false,
    Layout::Flow => flow, false,
    Layout::Spacing => spacing, false
}
