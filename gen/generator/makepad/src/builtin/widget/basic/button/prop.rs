use gen_parser::{PropsKey, Value};

use crate::{
    builtin::prop::{err_from_to, FromGenProps, Layout, MouseCursor, Prop, Themes, Vec2, Walk, F32},
    from_gen::MakepadColor, props_to_tokens,
};

// ------------ wait to implement ------------------------------------
// pub slot: WidgetRef,
// pub draw_button: DrawGView,
// pub animator: Animator,
// -------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    BackgroundColor(MakepadColor),
    BackgroundVisible(bool),
    HoverColor(MakepadColor),
    FocusColor(MakepadColor),
    ShadowColor(MakepadColor),
    SpreadRadius(F32),
    BlurRadius(F32),
    ShadowOffset(Vec2),
    BorderColor(MakepadColor),
    BorderWidth(F32),
    BorderRadius(F32),
    Cursor(MouseCursor),
    Visible(bool),
    GrabKeyFocus(bool),
    AnimationKey(bool),
    Walk(Walk),
    Layout(Layout),
    EventKey(bool),
}

impl FromGenProps for Prop<Props> {
    type Output = Prop<Props>;

    fn from_prop(prop: gen_parser::Props) -> Result<Option<Self::Output>, gen_utils::error::Error> {
        if let Some(props) = prop {
            let mut res = Prop::default();
            for (prop, value) in props {
                if prop.is_normal() {
                    res.push((prop, value).try_into()?);
                }
            }
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}
impl TryFrom<(PropsKey, Value)> for Props {
    type Error = gen_utils::error::Error;

    fn try_from(value: (PropsKey, Value)) -> Result<Self, Self::Error> {
        match value.0.name() {
            "theme" => Ok(Props::Theme(Themes::try_from(&value.1)?)),
            "background_color" => Ok(Props::BackgroundColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "focus_color" => Ok(Props::FocusColor(MakepadColor::try_from((&value.1, None))?)),
            "shadow_color" => Ok(Props::ShadowColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "spread_radius" => Ok(Props::SpreadRadius(value.1.as_f32()?.into())),
            "blur_radius" => Ok(Props::BlurRadius(value.1.as_f32()?.into())),
            "shadow_offset" => Ok(Props::ShadowOffset(Vec2::try_from(&value.1)?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_width" => Ok(Props::BorderWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            _ => Walk::try_from(&value)
                .map(Props::Walk)
                .or_else(|_| Layout::try_from(&value).map(Props::Layout))
                .map_err(|_| {
                    err_from_to(
                        "GenUI Props",
                        &format!("Makepad GButton Prop, Invalid Prop: {}", value.0.name()),
                    )
                    .into()
                }),
        }
    }
}

props_to_tokens!{
    Props,
    Props::Theme => theme, false,
    Props::BackgroundColor => background_color, false,
    Props::BackgroundVisible => background_visible, false,
    Props::HoverColor => hover_color, false,
    Props::FocusColor => focus_color, false,
    Props::ShadowColor => shadow_color, false,
    Props::SpreadRadius => spread_radius, false,
    Props::BlurRadius => blur_radius, false,
    Props::ShadowOffset => shadow_offset, false,
    Props::BorderColor => border_color, false,
    Props::BorderWidth => border_width, false,
    Props::BorderRadius => border_radius, false,
    Props::Cursor => cursor, false,
    Props::Visible => visible, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    Props::AnimationKey => animation_key, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::EventKey => event_key, false
}