use crate::{
    builtin::prop::{NavMode, Prop},
    from_gen_props, props_to_tokens,
};
use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

#[derive(Debug, Clone)]
pub enum Props {
    NavMode(NavMode),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "nav_mode" => Ok(Props::NavMode(NavMode::try_from(&value.1)?)),

            _ => {
                return Err(err_from_to!(
                    "GenUI Props" => &format!("Makepad GCollapse Prop, Invalid Prop: {}", value.0.name)
                ).to_runtime("Makepad Compiler"));
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::NavMode => nav_mode, false
}
