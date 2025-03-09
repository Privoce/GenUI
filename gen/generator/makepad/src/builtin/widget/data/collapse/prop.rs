use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

use crate::{
    builtin::prop::{Layout, MouseCursor, Position4, Prop, Walk, F64},
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Walk(Walk),
    Layout(Layout),
    RectSize(F64),
    Opened(bool),
    Fold(F64),
    Cursor(MouseCursor),
    GrabKeyFocus(bool),
    Visible(bool),
    AnimationKey(bool),
    Position(Position4),
    EventKey(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "rect_size" => Ok(Props::RectSize(value.1.as_f64()?.into())),
            "opened" => Ok(Props::Opened(value.1.as_bool()?)),
            "fold" => Ok(Props::Fold(value.1.as_f64()?.into())),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            "position" => Ok(Props::Position(Position4::try_from(&value.1)?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to!(
                        "GenUI Props" => &format!("Makepad GCollapse Prop, Invalid Prop: {}", value.0.name)
                    ).to_runtime("Makepad Compiler"));
                }
            }
        }
    }
}

props_to_tokens!{
    Props,
    Props::RectSize => rect_size, false,
    Props::Opened => opened, false,
    Props::Fold => fold, false,
    Props::Cursor => cursor, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    Props::Visible => visible, false,
    Props::AnimationKey => animation_key, false,
    Props::EventKey => event_key, false,
    Props::Position => position, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true
}