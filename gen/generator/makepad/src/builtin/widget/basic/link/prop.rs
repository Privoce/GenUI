use gen_analyzer::{PropKey, value::Value};

use crate::{
    builtin::prop::{
        err_from_to, DVec2, Layout, LinkType, LiveDependency, Margin, MouseCursor, Prop, Size, TextWalk, Themes, Walk, F32, F64
    },
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    BackgroundColor(MakepadColor),
    HoverColor(MakepadColor),
    FocusColor(MakepadColor),
    BorderColor(MakepadColor),
    UnderlineVisible(bool),
    UnderlineColor(MakepadColor),
    UnderlineHoverColor(MakepadColor),
    UnderlineFocusColor(MakepadColor),
    UnderlineWidth(F32),
    BorderRadius(F32),
    Round(bool),
    BackgroundVisible(bool),
    Text(String),
    FontSize(F64),
    Color(MakepadColor),
    TextHoverColor(MakepadColor),
    TextFocusColor(MakepadColor),
    FontFamily(LiveDependency),
    Cursor(MouseCursor),
    Href(String),
    LinkType(LinkType),
    Visible(bool),
    TextWalk(TextWalk),
    Walk(Walk),
    Layout(Layout),
    AnimationKey(bool),
    EventKey(bool),
    GrabKeyFocus(bool),
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
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "focus_color" => Ok(Props::FocusColor(MakepadColor::try_from((&value.1, None))?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "underline_visible" => Ok(Props::UnderlineVisible(value.1.as_bool()?)),
            "underline_color" => Ok(Props::UnderlineColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "underline_hover_color" => Ok(Props::UnderlineHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "underline_focus_color" => Ok(Props::UnderlineFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "underline_width" => Ok(Props::UnderlineWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "round" => Ok(Props::Round(value.1.as_bool()?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "text" => Ok(Props::Text(value.1.as_string()?)),
            "font_size" => Ok(Props::FontSize(value.1.as_f64()?.into())),
            "color" => Ok(Props::Color(MakepadColor::try_from((&value.1, None))?)),
            "text_hover_color" => Ok(Props::TextHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "text_focus_color" => Ok(Props::TextFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "font_family" => Ok(Props::FontFamily(LiveDependency::try_from(&value.1)?)),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "href" => Ok(Props::Href(value.1.as_string()?)),
            "link_type" => Ok(Props::LinkType(LinkType::try_from(&value.1)?)),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "text_height" => Ok(Props::TextWalk(Walk::Height(Size::try_from(&value.1)?).into())),
            "text_width" => Ok(Props::TextWalk(Walk::Width(Size::try_from(&value.1)?).into())),
            "text_margin" => Ok(Props::TextWalk(Walk::Margin(Margin::try_from(&value.1)?).into())),
            "text_abs_pos" => Ok(Props::TextWalk(Walk::AbsPos(DVec2::try_from(&value.1)?).into())),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to(
                        "GenUI Props",
                        &format!("Makepad GView Prop, Invalid Prop: {}", value.0.name),
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
    Props::BackgroundColor => background_color, false,
    Props::HoverColor => hover_color, false,
    Props::FocusColor => focus_color, false,
    Props::BorderColor => border_color, false,
    Props::UnderlineVisible => underline_visible, false,
    Props::UnderlineColor => underline_color, false,
    Props::UnderlineHoverColor => underline_hover_color, false,
    Props::UnderlineFocusColor => underline_focus_color, false,
    Props::UnderlineWidth => underline_width, false,
    Props::BorderRadius => border_radius, false,
    Props::Round => round, false,
    Props::BackgroundVisible => background_visible, false,
    Props::Text => text, false,
    Props::FontSize => font_size, false,
    Props::Color => color, false,
    Props::TextHoverColor => text_hover_color, false,
    Props::TextFocusColor => text_focus_color, false,
    Props::FontFamily => font_family, false,
    Props::Cursor => cursor, false,
    Props::Href => href, false,
    Props::LinkType => link_type, false,
    Props::Visible => visible, false,
    Props::TextWalk => text_walk, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::AnimationKey => animation_key, false,
    Props::EventKey => event_key, false,
    Props::GrabKeyFocus => grab_key_focus, false
}
