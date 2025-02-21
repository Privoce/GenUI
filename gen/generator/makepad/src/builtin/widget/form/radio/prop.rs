use gen_analyzer::{PropKey, value::Value};
use crate::{
    builtin::prop::{
        err_from_to, GChooseType, Layout, LiveDependency, MouseCursor, Prop, TextWrap, Themes,
        Vec2, Walk, F32, F64,
    },
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    Color(MakepadColor),
    TextHoverColor(MakepadColor),
    TextFocusColor(MakepadColor),
    FontSize(F64),
    HeightFactor(F64),
    Wrap(TextWrap),
    FontFamily(LiveDependency),
    TextVisible(bool),
    Size(F32),
    RadioBackgroundColor(MakepadColor),
    RadioBackgroundVisible(bool),
    RadioHoverColor(MakepadColor),
    RadioSelectedColor(MakepadColor),
    StrokeColor(MakepadColor),
    StrokeHoverColor(MakepadColor),
    StrokeSelectedColor(MakepadColor),
    RadioBorderColor(MakepadColor),
    RadioBorderWidth(F32),
    Scale(F32),
    BackgroundColor(MakepadColor),
    HoverColor(MakepadColor),
    FocusColor(MakepadColor),
    ShadowColor(MakepadColor),
    BorderColor(MakepadColor),
    BackgroundVisible(bool),
    BorderWidth(F32),
    BorderRadius(F32),
    SpreadRadius(F32),
    BlurRadius(F32),
    ShadowOffset(Vec2),
    Cursor(MouseCursor),
    Value(String),
    Selected(bool),
    Text(String),
    RadioType(GChooseType),
    Walk(Walk),
    Layout(Layout),
    Visible(bool),
    AnimationKey(bool),
    GrabKeyFocus(bool),
    EventKey(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "theme" => Ok(Props::Theme(Themes::try_from(&value.1)?)),
            "color" => Ok(Props::Color(MakepadColor::try_from((&value.1, None))?)),
            "text_hover_color" => Ok(Props::TextHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "text_focus_color" => Ok(Props::TextFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "font_size" => Ok(Props::FontSize(value.1.as_f64()?.into())),
            "height_factor" => Ok(Props::HeightFactor(value.1.as_f64()?.into())),
            "wrap" => Ok(Props::Wrap(TextWrap::try_from(&value.1)?)),
            "font_family" => Ok(Props::FontFamily(LiveDependency::try_from(&value.1)?)),
            "text_visible" => Ok(Props::TextVisible(value.1.as_bool()?)),
            "size" => Ok(Props::Size(value.1.as_f32()?.into())),
            "radio_background_color" => Ok(Props::RadioBackgroundColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "radio_background_visible" => Ok(Props::RadioBackgroundVisible(value.1.as_bool()?)),
            "radio_hover_color" => Ok(Props::RadioHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "radio_selected_color" => Ok(Props::RadioSelectedColor(MakepadColor::try_from((
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
            "radio_border_color" => Ok(Props::RadioBorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "radio_border_width" => Ok(Props::RadioBorderWidth(value.1.as_f32()?.into())),
            "scale" => Ok(Props::Scale(value.1.as_f32()?.into())),
            "background_color" => Ok(Props::BackgroundColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "focus_color" => Ok(Props::FocusColor(MakepadColor::try_from((&value.1, None))?)),
            "shadow_color" => Ok(Props::ShadowColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "border_width" => Ok(Props::BorderWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "spread_radius" => Ok(Props::SpreadRadius(value.1.as_f32()?.into())),
            "blur_radius" => Ok(Props::BlurRadius(value.1.as_f32()?.into())),
            "shadow_offset" => Ok(Props::ShadowOffset(Vec2::try_from(&value.1)?)),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "value" => Ok(Props::Value(value.1.as_string()?)),
            "selected" => Ok(Props::Selected(value.1.as_bool()?)),
            "text" => Ok(Props::Text(value.1.as_string()?)),
            "radio_type" => Ok(Props::RadioType(GChooseType::try_from(&value.1)?)),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to(
                        "GenUI Props",
                        &format!("Makepad GRadio Prop, Invalid Prop: {}", value.0.name),
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
    Props::Color => color, false,
    Props::TextHoverColor => text_hover_color, false,
    Props::TextFocusColor => text_focus_color, false,
    Props::FontSize => font_size, false,
    Props::HeightFactor => height_factor, false,
    Props::Wrap => wrap, false,
    Props::FontFamily => font_family, false,
    Props::TextVisible => text_visible, false,
    Props::Size => size, false,
    Props::RadioBackgroundColor => radio_background_color, false,
    Props::RadioBackgroundVisible => radio_background_visible, false,
    Props::RadioHoverColor => radio_hover_color, false,
    Props::RadioSelectedColor => radio_selected_color, false,
    Props::StrokeColor => stroke_color, false,
    Props::StrokeHoverColor => stroke_hover_color, false,
    Props::StrokeSelectedColor => stroke_selected_color, false,
    Props::RadioBorderColor => radio_border_color, false,
    Props::RadioBorderWidth => radio_border_width, false,
    Props::Scale => scale, false,
    Props::BackgroundColor => background_color, false,
    Props::HoverColor => hover_color, false,
    Props::FocusColor => focus_color, false,
    Props::ShadowColor => shadow_color, false,
    Props::BorderColor => border_color, false,
    Props::BackgroundVisible => background_visible, false,
    Props::BorderWidth => border_width, false,
    Props::BorderRadius => border_radius, false,
    Props::SpreadRadius => spread_radius, false,
    Props::BlurRadius => blur_radius, false,
    Props::ShadowOffset => shadow_offset, false,
    Props::Cursor => cursor, false,
    Props::Value => value, false,
    Props::Selected => selected, false,
    Props::Text => text, false,
    Props::RadioType => radio_type, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::Visible => visible, false,
    Props::AnimationKey => animation_key, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    Props::EventKey => event_key, false
}
