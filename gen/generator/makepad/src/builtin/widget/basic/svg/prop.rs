use gen_parser::{PropsKey, Value};

use crate::{
    builtin::prop::{
        err_from_to, Layout, LiveDependency, MouseCursor, Prop, Themes, Walk, F32, F64,
    },
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    Brightness(F32),
    Curve(F32),
    Linearize(F32),
    Src(LiveDependency),
    Scale(F64),
    Color(MakepadColor),
    DrawDepth(F32),
    StrokeHoverColor(MakepadColor),
    StrokeFocusColor(MakepadColor),
    Cursor(MouseCursor),
    GrabKeyFocus(bool),
    Visible(bool),
    AnimationKey(bool),
    Walk(Walk),
    Layout(Layout),
    EventKey(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropsKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropsKey, Value)) -> Result<Self, Self::Error> {
        let name = value.0.name().to_string();
        match name.as_str() {
            "theme" => Ok(Props::Theme(Themes::try_from(&value.1)?)),
            "brightness" => Ok(Props::Brightness(value.1.as_f32()?.into())),
            "curve" => Ok(Props::Curve(value.1.as_f32()?.into())),
            "linearize" => Ok(Props::Linearize(value.1.as_f32()?.into())),
            "src" => Ok(Props::Src(LiveDependency::try_from(&value.1)?)),
            "scale" => Ok(Props::Scale(value.1.as_f64()?.into())),
            "color" => Ok(Props::Color(MakepadColor::try_from((&value.1, None))?)),
            "draw_depth" => Ok(Props::DrawDepth(value.1.as_f32()?.into())),
            "stroke_hover_color" => Ok(Props::StrokeHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "stroke_focus_color" => Ok(Props::StrokeFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to(
                        "GenUI Props",
                        &format!("Makepad Image Prop, Invalid Prop: {}", name),
                    )
                    .into());
                }
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::Theme => theme, false,
    Props::Brightness => brightness, false,
    Props::Curve => curve, false,
    Props::Linearize => linearize, false,
    Props::Src => src, false,
    Props::Scale => scale, false,
    Props::Color => color, false,
    Props::DrawDepth => draw_depth, false,
    Props::StrokeHoverColor => stroke_hover_color, false,
    Props::StrokeFocusColor => stroke_focus_color, false,
    Props::Cursor => cursor, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    Props::Visible => visible, false,
    Props::AnimationKey => animation_key, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::EventKey => event_key, false
}
