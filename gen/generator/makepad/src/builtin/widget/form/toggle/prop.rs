use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

use crate::{
    builtin::prop::{GToggleType, Layout, MouseCursor, Prop, Themes, Walk, F32},
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    BackgroundColor(MakepadColor),
    BackgroundVisible(bool),
    HoverColor(MakepadColor),
    SelectedColor(MakepadColor),
    StrokeColor(MakepadColor),
    StrokeHoverColor(MakepadColor),
    StrokeSelectedColor(MakepadColor),
    BorderColor(MakepadColor),
    BorderWidth(F32),
    BorderRadius(F32),
    Scale(F32),
    Cursor(MouseCursor),
    Selected(bool),
    GrabKeyFocus(bool),
    ToggleType(GToggleType),
    // DrawToggle(DrawGToggle),
    Walk(Walk),
    Layout(Layout),
    Visible(bool),
    AnimationKey(bool),
    EventKey(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "theme" => Ok(Props::Theme(Themes::try_from(&value.1)?)),
            "background_color" => Ok(Props::BackgroundColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "selected_color" => Ok(Props::SelectedColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "stroke_color" => Ok(Props::StrokeColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "stroke_hover_color" => Ok(Props::StrokeHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "stroke_selected_color" => Ok(Props::StrokeSelectedColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_width" => Ok(Props::BorderWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "scale" => Ok(Props::Scale(value.1.as_f32()?.into())),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "selected" => Ok(Props::Selected(value.1.as_bool()?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "toggle_type" => Ok(Props::ToggleType(GToggleType::try_from(&value.1)?)),
            // "draw_toggle" => Ok(Props::DrawToggle(DrawGToggle::try_from(&value.1)?)),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to!(
                        "GenUI Props" => &format!("Makepad GToggle Prop, Invalid Prop: {}", value.0.name)
                    ));
                }
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::Theme => theme, false,
    Props::BackgroundColor => background_color, false,
    Props::BackgroundVisible => background_visible, false,
    Props::HoverColor => hover_color, false,
    Props::SelectedColor => selected_color, false,
    Props::StrokeColor => stroke_color, false,
    Props::StrokeHoverColor => stroke_hover_color, false,
    Props::StrokeSelectedColor => stroke_selected_color, false,
    Props::BorderColor => border_color, false,
    Props::BorderWidth => border_width, false,
    Props::BorderRadius => border_radius, false,
    Props::Scale => scale, false,
    Props::Cursor => cursor, false,
    Props::Selected => selected, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    Props::ToggleType => toggle_type, false,
    // Props::DrawToggle => draw_toggle, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::Visible => visible, false,
    Props::AnimationKey => animation_key, false,
    Props::EventKey => event_key, false
}
