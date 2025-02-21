use gen_analyzer::{PropKey, value::Value};

use crate::{
    builtin::prop::{
        err_from_to, Layout, LiveDependency, Prop, TextWrap, Themes, Vec2, Walk, F32, F64,
    },
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    ShadowColor(MakepadColor),
    SpreadRadius(F32),
    BlurRadius(F32),
    ShadowOffset(Vec2),
    PlaceholderColor(MakepadColor),
    Color(MakepadColor),
    CursorColor(MakepadColor),
    SelectColor(MakepadColor),
    BackgroundColor(MakepadColor),
    BackgroundVisible(bool),
    Visible(bool),
    HoverColor(MakepadColor),
    TextHoverColor(MakepadColor),
    TextFocusColor(MakepadColor),
    CursorHoverColor(MakepadColor),
    CursorFocusColor(MakepadColor),
    SelectHoverColor(MakepadColor),
    SelectFocusColor(MakepadColor),
    FocusColor(MakepadColor),
    BorderColor(MakepadColor),
    BorderWidth(F32),
    BorderRadius(F32),
    FontSize(F64),
    HeightFactor(F64),
    Wrap(TextWrap),
    FontFamily(LiveDependency),
    CursorBorderRadius(F64),
    Walk(Walk),
    Layout(Layout),
    CursorWidth(F64),
    ReadOnly(bool),
    NumericOnly(bool),
    Placeholder(String),
    Text(String),
    EventKey(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "theme" => Ok(Props::Theme(Themes::try_from(&value.1)?)),
            "shadow_color" => Ok(Props::ShadowColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "spread_radius" => Ok(Props::SpreadRadius(value.1.as_f32()?.into())),
            "blur_radius" => Ok(Props::BlurRadius(value.1.as_f32()?.into())),
            "shadow_offset" => Ok(Props::ShadowOffset(Vec2::try_from(&value.1)?)),
            "placeholder_color" => Ok(Props::PlaceholderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "color" => Ok(Props::Color(MakepadColor::try_from((&value.1, None))?)),
            "cursor_color" => Ok(Props::CursorColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "select_color" => Ok(Props::SelectColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "background_color" => Ok(Props::BackgroundColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "text_hover_color" => Ok(Props::TextHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "text_focus_color" => Ok(Props::TextFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "cursor_hover_color" => Ok(Props::CursorHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "cursor_focus_color" => Ok(Props::CursorFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "select_hover_color" => Ok(Props::SelectHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "select_focus_color" => Ok(Props::SelectFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "focus_color" => Ok(Props::FocusColor(MakepadColor::try_from((&value.1, None))?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_width" => Ok(Props::BorderWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "font_size" => Ok(Props::FontSize(value.1.as_f64()?.into())),
            "height_factor" => Ok(Props::HeightFactor(value.1.as_f64()?.into())),
            "wrap" => Ok(Props::Wrap(TextWrap::try_from(&value.1)?)),
            "font_family" => Ok(Props::FontFamily(LiveDependency::try_from(&value.1)?)),
            "cursor_border_radius" => Ok(Props::CursorBorderRadius(value.1.as_f64()?.into())),
            "cursor_width" => Ok(Props::CursorWidth(value.1.as_f64()?.into())),
            "read_only" => Ok(Props::ReadOnly(value.1.as_bool()?)),
            "numeric_only" => Ok(Props::NumericOnly(value.1.as_bool()?)),
            "placeholder" => Ok(Props::Placeholder(value.1.as_string()?)),
            "text" => Ok(Props::Text(value.1.as_string()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to(
                        "GenUI Props",
                        &format!("Makepad GCheckbox Prop, Invalid Prop: {}", value.0.name),
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
    Props::ShadowColor => shadow_color, false,
    Props::SpreadRadius => spread_radius, false,
    Props::BlurRadius => blur_radius, false,
    Props::ShadowOffset => shadow_offset, false,
    Props::PlaceholderColor => placeholder_color, false,
    Props::Color => color, false,
    Props::CursorColor => cursor_color, false,
    Props::SelectColor => select_color, false,
    Props::BackgroundColor => background_color, false,
    Props::BackgroundVisible => background_visible, false,
    Props::Visible => visible, false,
    Props::HoverColor => hover_color, false,
    Props::TextHoverColor => text_hover_color, false,
    Props::TextFocusColor => text_focus_color, false,
    Props::CursorHoverColor => cursor_hover_color, false,
    Props::CursorFocusColor => cursor_focus_color, false,
    Props::SelectHoverColor => select_hover_color, false,
    Props::SelectFocusColor => select_focus_color, false,
    Props::FocusColor => focus_color, false,
    Props::BorderColor => border_color, false,
    Props::BorderWidth => border_width, false,
    Props::BorderRadius => border_radius, false,
    Props::FontSize => font_size, false,
    Props::HeightFactor => height_factor, false,
    Props::Wrap => wrap, false,
    Props::FontFamily => font_family, false,
    Props::CursorBorderRadius => cursor_border_radius, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::CursorWidth => cursor_width, false,
    Props::ReadOnly => read_only, false,
    Props::NumericOnly => numeric_only, false,
    Props::Placeholder => placeholder, false,
    Props::Text => text, false,
    Props::EventKey => event_key, false
}
