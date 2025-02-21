use gen_analyzer::{PropKey, value::Value};

use crate::{
    builtin::{
        prop::{err_from_to, Direction, Prop, F32},
        widget::ViewProps,
    },
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    DerefWidget(ViewProps),
    StrokeWidth(F32),
    Direction(Direction),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        let name = value.0.name.to_string();
        match name.as_str() {
            "stroke_width" => Ok(Props::StrokeWidth(value.1.as_f32()?.into())),
            "direction" => Ok(Props::Direction(Direction::try_from(&value.1)?)),
            _ => {
                // handle deref widget
                if let Ok(p) = ViewProps::try_from(value) {
                    return Ok(Props::DerefWidget(p));
                } else {
                    return Err(err_from_to(
                        "GenUI Props",
                        &format!("Makepad Divider Prop, Invalid Prop: {}", name),
                    )
                    .into());
                }
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::DerefWidget => deref_widget, true,
    Props::StrokeWidth => stroke_width, false,
    Props::Direction => direction, false
}
