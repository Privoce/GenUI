use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

use crate::{
    builtin::prop::{CloseMode, Layout, MouseCursor, PopupMode, Prop, Themes, Vec2, Walk, F32},
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    Opacity(F32),
    BackgroundColor(MakepadColor),
    HoverColor(MakepadColor),
    FocusColor(MakepadColor),
    BorderColor(MakepadColor),
    BorderWidth(F32),
    BorderRadius(F32),
    Visible(bool),
    BackgroundVisible(bool),
    Cursor(MouseCursor),
    Mode(PopupMode),
    CloseMode(CloseMode),
    ShadowColor(MakepadColor),
    SpreadRadius(F32),
    BlurRadius(F32),
    ShadowOffset(Vec2),
    // DrawPopup(DrawGPopup),
    Walk(Walk),
    Layout(Layout),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "theme" => Ok(Props::Theme(Themes::try_from(&value.1)?)),
            "opacity" => Ok(Props::Opacity(value.1.as_f32()?.into())),
            "background_color" => Ok(Props::BackgroundColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "focus_color" => Ok(Props::FocusColor(MakepadColor::try_from((&value.1, None))?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_width" => Ok(Props::BorderWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "mode" => Ok(Props::Mode(PopupMode::try_from(&value.1)?)),
            "close_mode" => Ok(Props::CloseMode(CloseMode::try_from(&value.1)?)),
            "shadow_color" => Ok(Props::ShadowColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "spread_radius" => Ok(Props::SpreadRadius(value.1.as_f32()?.into())),
            "blur_radius" => Ok(Props::BlurRadius(value.1.as_f32()?.into())),
            "shadow_offset" => Ok(Props::ShadowOffset(Vec2::try_from(&value.1)?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to!(
                        "GenUI Props" => &format!("Makepad GPopup Prop, Invalid Prop: {}", value.0.name)
                    ).to_runtime("Makepad Compiler"));
                }
            }
        }
    }
}

props_to_tokens! {
    Props,
    Props::Theme => theme, false,
    Props::Opacity => opacity, false,
    Props::BackgroundColor => background_color, false,
    Props::HoverColor => hover_color, false,
    Props::FocusColor => focus_color, false,
    Props::BorderColor => border_color, false,
    Props::BorderWidth => border_width, false,
    Props::BorderRadius => border_radius, false,
    Props::Visible => visible, false,
    Props::BackgroundVisible => background_visible, false,
    Props::Cursor => cursor, false,
    Props::Mode => mode, false,
    Props::CloseMode => close_mode, false,
    Props::ShadowColor => shadow_color, false,
    Props::SpreadRadius => spread_radius, false,
    Props::BlurRadius => blur_radius, false,
    Props::ShadowOffset => shadow_offset, false,
    // Props::DrawPopup => draw_popup, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true
}
