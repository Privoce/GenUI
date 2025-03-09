use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

use crate::{
    builtin::prop::{CloseMode, Position, Prop, TriggerMode, F32},
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Position(Position),
    TriggerMode(TriggerMode),
    Opened(bool),
    Offset(F32),
    OffsetX(F32),
    OffsetY(F32),
    Visible(bool),
    Proportion(F32),
    EventKey(bool),
    CloseMode(CloseMode),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "position" => Ok(Props::Position(Position::try_from(&value.1)?)),
            "trigger_mode" => Ok(Props::TriggerMode(TriggerMode::try_from(&value.1)?)),
            "opened" => Ok(Props::Opened(value.1.as_bool()?)),
            "offset" => Ok(Props::Offset(value.1.as_f32()?.into())),
            "offset_x" => Ok(Props::OffsetX(value.1.as_f32()?.into())),
            "offset_y" => Ok(Props::OffsetY(value.1.as_f32()?.into())),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "proportion" => Ok(Props::Proportion(value.1.as_f32()?.into())),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            "close_mode" => Ok(Props::CloseMode(CloseMode::try_from(&value.1)?)),
            _ => {
                return Err(err_from_to!(
                    "GenUI Props" => &format!("Makepad GDropDown Prop, Invalid Prop: {}", value.0.name)
                ).to_runtime("Makepad Compiler"));
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::Position => position, false,
    Props::TriggerMode => trigger_mode, false,
    Props::Opened => opened, false,
    Props::Offset => offset, false,
    Props::OffsetX => offset_x, false,
    Props::OffsetY => offset_y, false,
    Props::Visible => visible, false,
    Props::Proportion => proportion, false,
    Props::EventKey => event_key, false,
    Props::CloseMode => close_mode, false
}
