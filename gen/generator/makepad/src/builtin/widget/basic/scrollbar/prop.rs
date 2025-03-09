use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

use crate::{builtin::prop::Prop, from_gen_props, props_to_tokens};

#[derive(Debug, Clone)]
pub enum Props {
    ShowScrollX(bool),
    ShowScrollY(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "scroll_x" => Ok(Props::ShowScrollX(value.1.as_bool()?)),
            "scroll_y" => Ok(Props::ShowScrollY(value.1.as_bool()?)),
            _ => Err(err_from_to!(
                "GenUI Props" => &format!("Makepad GScrollBars Prop, Invalid Prop: {}", value.0.name)
            )),
        }
    }
}

props_to_tokens! {
    Props,
    Props::ShowScrollX => show_scroll_x, false,
    Props::ShowScrollY => show_scroll_y, false
}
