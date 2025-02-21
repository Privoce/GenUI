use gen_analyzer::{PropKey, value::Value};

use crate::{
    builtin::prop::{
        err_from_to, Align, LiveDependency, Margin, MouseCursor, Padding, Prop, Size, TextWrap,
        Themes, F64,
    },
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

// ------------ wait to implement ------------------------------------
// pub draw_text: Option<DrawGText>, label中暂时不开启
// pub abs_pos: Option<DVec2>, abs_pos now have bug
// pub animator: Animator,
// -------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    Color(MakepadColor),
    StrokeHoverColor(MakepadColor),
    StrokeFocusColor(MakepadColor),
    FontSize(F64),
    Cursor(MouseCursor),
    LineSpacing(F64),
    HeightFactor(F64),
    Wrap(TextWrap),
    FontFamily(LiveDependency),
    Visible(bool),
    Height(Size),
    Width(Size),
    Margin(Margin),
    Padding(Padding),
    Align(Align),
    Text(String),
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
            "color" => Ok(Props::Color(MakepadColor::try_from((&value.1, None))?)),
            "stroke_hover_color" => Ok(Props::StrokeHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "stroke_focus_color" => Ok(Props::StrokeFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "font_size" => Ok(Props::FontSize(value.1.as_f64()?.into())),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "line_spacing" => Ok(Props::LineSpacing(value.1.as_f64()?.into())),
            "height_factor" => Ok(Props::HeightFactor(value.1.as_f64()?.into())),
            "wrap" => Ok(Props::Wrap(TextWrap::try_from(&value.1)?)),
            "font_family" => Ok(Props::FontFamily(LiveDependency::try_from(&value.1)?)),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "height" => Ok(Props::Height(Size::try_from(&value.1)?)),
            "width" => Ok(Props::Width(Size::try_from(&value.1)?)),
            "margin" => Ok(Props::Margin(Margin::try_from(&value.1)?)),
            "padding" => Ok(Props::Padding(Padding::try_from(&value.1)?)),
            "align" => Ok(Props::Align(Align::try_from(&value.1)?)),
            "text" => Ok(Props::Text(value.1.as_string()?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            _ => Err(err_from_to(
                "GenUI Props",
                &format!("Makepad GLabel Prop, Invalid Prop: {}", &value.0.name),
            )
            .into()),
        }
    }
}

props_to_tokens! {
    Props,
    Props::Theme => theme, false,
    Props::Color => color, false,
    Props::StrokeHoverColor => stroke_hover_color, false,
    Props::StrokeFocusColor => stroke_focus_color, false,
    Props::FontSize => font_size, false,
    Props::Cursor => cursor, false,
    Props::LineSpacing => line_spacing, false,
    Props::HeightFactor => height_factor, false,
    Props::Wrap => wrap, false,
    Props::FontFamily => font_family, false,
    Props::Visible => visible, false,
    Props::Height => height, false,
    Props::Width => width, false,
    Props::Margin => margin, false,
    Props::Padding => padding, false,
    Props::Align => align, false,
    Props::Text => text, false,
    Props::AnimationKey => animation_key, false,
    Props::EventKey => event_key, false,
    Props::GrabKeyFocus => grab_key_focus, false
}
