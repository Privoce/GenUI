use gen_analyzer::{value::Value, PropKey};
use gen_utils::{err_from_to, error::Error};

use crate::{
    builtin::prop::{
        Align, DVec2, Flow, Layout, LiveDependency, Margin, MouseCursor, Padding, Prop, Size, TextWalk, Themes, Vec2, Walk, F32, F64
    },
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    BackgroundColor(MakepadColor),
    BackgroundVisible(bool),
    HoverColor(MakepadColor),
    StrokeHoverColor(MakepadColor),
    TextHoverColor(MakepadColor),
    FocusColor(MakepadColor),
    StrokeFocusColor(MakepadColor),
    TextFocusColor(MakepadColor),
    BorderColor(MakepadColor),
    BorderWidth(F32),
    BorderRadius(F32),
    ShadowColor(MakepadColor),
    SpreadRadius(F32),
    BlurRadius(F32),
    ShadowOffset(Vec2),
    Text(String),
    FontSize(F64),
    Color(MakepadColor),
    FontFamily(LiveDependency),
    HeightFactor(F64),
    LineScale(F64),
    Cursor(MouseCursor),
    Closeable(bool),
    Src(LiveDependency),
    IconBrightness(F32),
    IconCurve(F32),
    IconLinearize(F32),
    IconScale(F64),
    IconColor(MakepadColor),
    IconDrawDepth(F32),
    Visible(bool),
    // DrawText(DrawGText),
    TextWalk(TextWalk),
    GrabKeyFocus(bool),
    // DrawIcon(DrawGSvg),
    // DrawClose(DrawGIconPixel),
    IconWalk(Walk),
    IconLayout(Layout),
    // DrawTag(DrawGView),
    Walk(Walk),
    Layout(Layout),
    AnimationKey(bool),
    EventKey(bool),
}

from_gen_props!(Props);

impl TryFrom<(PropKey, Value)> for Props {
    type Error = Error;

    fn try_from(value: (PropKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name.as_str() {
            "theme" => Ok(Props::Theme(Themes::try_from(&value.1)?)),
            "background_color" => Ok(Props::BackgroundColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "stroke_hover_color" => Ok(Props::StrokeHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "text_hover_color" => Ok(Props::TextHoverColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "focus_color" => Ok(Props::FocusColor(MakepadColor::try_from((&value.1, None))?)),
            "stroke_focus_color" => Ok(Props::StrokeFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "text_focus_color" => Ok(Props::TextFocusColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_width" => Ok(Props::BorderWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "shadow_color" => Ok(Props::ShadowColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "spread_radius" => Ok(Props::SpreadRadius(value.1.as_f32()?.into())),
            "blur_radius" => Ok(Props::BlurRadius(value.1.as_f32()?.into())),
            "shadow_offset" => Ok(Props::ShadowOffset(Vec2::try_from(&value.1)?)),
            "text" => Ok(Props::Text(value.1.as_string()?)),
            "font_size" => Ok(Props::FontSize(value.1.as_f64()?.into())),
            "color" => Ok(Props::Color(MakepadColor::try_from((&value.1, None))?)),
            "font_family" => Ok(Props::FontFamily(LiveDependency::try_from(&value.1)?)),
            "height_factor" => Ok(Props::HeightFactor(value.1.as_f64()?.into())),
            "line_scale" => Ok(Props::LineScale(value.1.as_f64()?.into())),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "closeable" => Ok(Props::Closeable(value.1.as_bool()?)),
            "src" => Ok(Props::Src(LiveDependency::try_from(&value.1)?)),
            "icon_brightness" => Ok(Props::IconBrightness(value.1.as_f32()?.into())),
            "icon_curve" => Ok(Props::IconCurve(value.1.as_f32()?.into())),
            "icon_linearize" => Ok(Props::IconLinearize(value.1.as_f32()?.into())),
            "icon_scale" => Ok(Props::IconScale(value.1.as_f64()?.into())),
            "icon_color" => Ok(Props::IconColor(MakepadColor::try_from((&value.1, None))?)),
            "icon_draw_depth" => Ok(Props::IconDrawDepth(value.1.as_f32()?.into())),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            // "draw_text" => Ok(Props::DrawText(DrawGText::try_from(&value.1)?)),
            "text_height" => Ok(Props::TextWalk(
                Walk::Height(Size::try_from(&value.1)?).into(),
            )),
            "text_width" => Ok(Props::TextWalk(
                Walk::Width(Size::try_from(&value.1)?).into(),
            )),
            "text_margin" => Ok(Props::TextWalk(
                Walk::Margin(Margin::try_from(&value.1)?).into(),
            )),
            "text_abs_pos" => Ok(Props::TextWalk(
                Walk::AbsPos(DVec2::try_from(&value.1)?).into(),
            )),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "icon_height" => Ok(Props::IconWalk(Walk::Height(Size::try_from(&value.1)?))),
            "icon_width" => Ok(Props::IconWalk(Walk::Width(Size::try_from(&value.1)?))),
            "icon_margin" => Ok(Props::IconWalk(Walk::Margin(Margin::try_from(&value.1)?))),
            "icon_abs_pos" => Ok(Props::IconWalk(Walk::AbsPos(DVec2::try_from(&value.1)?))),
            "icon_scroll" => Ok(Props::IconLayout(Layout::Scroll(DVec2::try_from(&value.1)?))),
            "icon_clip_x" => Ok(Props::IconLayout(Layout::ClipX(value.1.as_bool()?))),
            "icon_clip_y" => Ok(Props::IconLayout(Layout::ClipY(value.1.as_bool()?))),
            "icon_padding" => Ok(Props::IconLayout(Layout::Padding(Padding::try_from(&value.1)?))),
            "icon_align" => Ok(Props::IconLayout(Layout::Align(Align::try_from(&value.1)?))),
            "icon_flow" => Ok(Props::IconLayout(Layout::Flow(Flow::try_from(&value.1)?))),
            "icon_spacing" => Ok(Props::IconLayout(Layout::Spacing(value.1.as_f64()?.into()))),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            _ => {
                if let Ok(walk) = Walk::try_from(&value) {
                    return Ok(Props::Walk(walk));
                } else if let Ok(layout) = Layout::try_from(&value) {
                    return Ok(Props::Layout(layout));
                } else {
                    return Err(
                        err_from_to!("GenUI Props" => &format!("Makepad GTag Prop, Invalid Prop: {}", &value.0.name)),
                    );
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
    Props::StrokeHoverColor => stroke_hover_color, false,
    Props::TextHoverColor => text_hover_color, false,
    Props::FocusColor => focus_color, false,
    Props::StrokeFocusColor => stroke_focus_color, false,
    Props::TextFocusColor => text_focus_color, false,
    Props::BorderColor => border_color, false,
    Props::BorderWidth => border_width, false,
    Props::BorderRadius => border_radius, false,
    Props::ShadowColor => shadow_color, false,
    Props::SpreadRadius => spread_radius, false,
    Props::BlurRadius => blur_radius, false,
    Props::ShadowOffset => shadow_offset, false,
    Props::Text => text, false,
    Props::FontSize => font_size, false,
    Props::Color => color, false,
    Props::FontFamily => font_family, false,
    Props::HeightFactor => height_factor, false,
    Props::LineScale => line_scale, false,
    Props::Cursor => cursor, false,
    Props::Closeable => closeable, false,
    Props::Src => src, false,
    Props::IconBrightness => icon_brightness, false,
    Props::IconCurve => icon_curve, false,
    Props::IconLinearize => icon_linearize, false,
    Props::IconScale => icon_scale, false,
    Props::IconColor => icon_color, false,
    Props::IconDrawDepth => icon_draw_depth, false,
    Props::Visible => visible, false,
    // Props::DrawText => draw_text, false,
    Props::TextWalk => text_walk, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    // Props::DrawIcon => draw_icon, false,
    Props::IconWalk => icon_walk, false,
    Props::IconLayout => icon_layout, false,
    // Props::DrawTag => draw_tag, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::AnimationKey => animation_key, false,
    Props::EventKey => event_key, false
}
