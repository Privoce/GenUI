use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

use crate::{
    builtin::prop::{ImageFit, Layout, LiveDependency, MouseCursor, Prop, Walk, F32, F64},
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Visible(bool),
    GrabKeyFocus(bool),
    Opacity(F32),
    Cursor(MouseCursor),
    Scale(F64),
    Fit(ImageFit),
    Src(Src),
    MinWidth(i64),
    MinHeight(i64),
    Rotation(F32),
    Walk(Walk),
    Layout(Layout),
    EventKey(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        let name = value.0.name.to_string();
        match name.as_str() {
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "opacity" => Ok(Props::Opacity(value.1.as_f32()?.into())),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "scale" => Ok(Props::Scale(value.1.as_f64()?.into())),
            "fit" => Ok(Props::Fit(ImageFit::try_from(&value.1)?)),
            "min_width" => Ok(Props::MinWidth(value.1.as_isize()? as i64)),
            "min_height" => Ok(Props::MinHeight(value.1.as_isize()? as i64)),
            "rotation" => Ok(Props::Rotation(value.1.as_f32()?.into())),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            "src" => Ok(Props::Src(LiveDependency::try_from(&value.1)?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to!(
                        "GenUI Props" => &format!("Makepad Image Prop, Invalid Prop: {}", name)
                    ));
                }
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::Visible => visible, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    Props::Opacity => opacity, false,
    Props::Cursor => cursor, false,
    Props::Scale => scale, false,
    Props::Fit => fit, false,
    Props::MinWidth => min_width, false,
    Props::MinHeight => min_height, false,
    Props::Rotation => rotation, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::EventKey => event_key, false,
    Props::Src => src, false
}
