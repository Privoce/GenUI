use gen_parser::{PropsKey, Value};
use gen_utils::error::Error;

use crate::{
    builtin::prop::{
        err_from_to, value_bool, EventOrder, FromGenProps, Layout, MouseCursor, Prop, Themes, Vec2,
        ViewOptimize, Walk, F32, F64,
    },
    from_gen::MakepadColor,
    props_to_tokens,
};

// ------------ wait to implement ------------------------------------
// pub scroll_bars: Option<LivePtr>,
// pub draw_view: DrawGView,
// pub animator: Animator,
// -------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    BackgroundColor(MakepadColor),
    HoverColor(MakepadColor),
    FocusColor(MakepadColor),
    BorderColor(MakepadColor),
    BorderWidth(F32),
    BorderRadius(F32),
    Visible(bool),
    BackgroundVisible(bool),
    ShadowColor(MakepadColor),
    SpreadRadius(F32),
    BlurRadius(F32),
    ShadowOffset(Vec2),
    Cursor(MouseCursor),
    AnimationKey(bool),
    GrabKeyFocus(bool),
    BlockSignalEvent(bool),
    // MinWidth(f32),
    // MinHeight(f32),
    // MaxWidth(f32),
    // MaxHeight(f32),
    Walk(Walk),
    Layout(Layout),
    EventOrder(EventOrder),
    DpiFactor(F64),
    Optimize(ViewOptimize),
    CaptureOverload(bool),
    EventKey(bool),
    BlockChildEvents(bool),
}

impl Props {
    pub fn to_toml_item_kv(&self) -> (String, toml_edit::Value) {
        match self {
            Props::Theme(themes) => ("theme".to_string(), themes.into()),
            Props::BackgroundColor(makepad_color) => {
                ("background_color".to_string(), makepad_color.into())
            }
            Props::HoverColor(makepad_color) => ("hover_color".to_string(), makepad_color.into()),
            Props::FocusColor(makepad_color) => ("focus_color".to_string(), makepad_color.into()),
            Props::BorderColor(makepad_color) => ("border_color".to_string(), makepad_color.into()),
            Props::BorderWidth(f) => ("border_width".to_string(), f.into()),
            Props::BorderRadius(f) => ("border_radius".to_string(), f.into()),
            Props::Visible(vis) => ("visible".to_string(), (*vis).into()),
            Props::BackgroundVisible(vis) => ("background_visible".to_string(), (*vis).into()),
            Props::ShadowColor(makepad_color) => ("shadow_color".to_string(), makepad_color.into()),
            Props::SpreadRadius(f) => ("spread_radius".to_string(), f.into()),
            Props::BlurRadius(f) => ("blur_radius".to_string(), f.into()),
            Props::ShadowOffset(vec2) => ("shadow_offset".to_string(), vec2.into()),
            Props::Cursor(mouse_cursor) => ("cursor".to_string(), mouse_cursor.into()),
            Props::AnimationKey(key) => ("animation_key".to_string(), (*key).into()),
            Props::GrabKeyFocus(focus) => ("grab_key_focus".to_string(), (*focus).into()),
            Props::BlockSignalEvent(e) => ("block_signal_event".to_string(), (*e).into()),
            Props::Walk(walk) => {
                let (key, value) = walk.to_toml_item_kv();
                (key, value)
            }
            Props::Layout(layout) => {
                let (key, value) = layout.to_toml_item_kv();
                (key, value)
            }
            Props::EventOrder(event_order) => ("event_order".to_string(), event_order.into()),
            Props::DpiFactor(f) => ("dpi_factor".to_string(), f.into()),
            Props::Optimize(view_optimize) => ("optimize".to_string(), view_optimize.into()),
            Props::CaptureOverload(o) => ("capture_overload".to_string(), (*o).into()),
            Props::EventKey(key) => ("event_key".to_string(), (*key).into()),
            Props::BlockChildEvents(e) => ("block_child_events".to_string(), (*e).into()),
        }
    }
}

impl TryFrom<(&str, &toml_edit::Value)> for Props {
    type Error = Error;

    fn try_from(value: (&str, &toml_edit::Value)) -> Result<Self, Self::Error> {
        match value.0 {
            "theme" => Ok(Props::Theme(value.1.try_into()?)),
            "background_color" => Ok(Props::BackgroundColor(value.1.try_into()?)),
            "hover_color" => Ok(Props::HoverColor(value.1.try_into()?)),
            "focus_color" => Ok(Props::FocusColor(value.1.try_into()?)),
            "border_color" => Ok(Props::BorderColor(value.1.try_into()?)),
            "border_width" => Ok(Props::BorderWidth(value.1.try_into()?)),
            "border_radius" => Ok(Props::BorderRadius(value.1.try_into()?)),
            "visible" => Ok(Props::Visible(value_bool(value.1)?)),
            "background_visible" => Ok(Props::BackgroundVisible(value_bool(value.1)?)),
            "shadow_color" => Ok(Props::ShadowColor(value.1.try_into()?)),
            "spread_radius" => Ok(Props::SpreadRadius(value.1.try_into()?)),
            "blur_radius" => Ok(Props::BlurRadius(value.1.try_into()?)),
            "shadow_offset" => Ok(Props::ShadowOffset(value.1.try_into()?)),
            "cursor" => Ok(Props::Cursor(value.1.try_into()?)),
            "animation_key" => Ok(Props::AnimationKey(value_bool(value.1)?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value_bool(value.1)?)),
            "block_signal_event" => Ok(Props::BlockSignalEvent(value_bool(value.1)?)),
            // "min_width" => Ok(Props::MinWidth(&value.1.try_into()?)),
            // "min_height" => Ok(Props::MinHeight(&value.1.try_into()?)),
            // "max_width" => Ok(Props::MaxWidth(&value.1.try_into()?)),
            // "max_height" => Ok(Props::MaxHeight(&value.1.try_into()?)),
            "event_order" => Ok(Props::EventOrder(value.1.try_into()?)),
            "dpi_factor" => Ok(Props::DpiFactor(value.1.try_into()?)),
            "optimize" => Ok(Props::Optimize(value.1.try_into()?)),
            "capture_overload" => Ok(Props::CaptureOverload(value_bool(value.1)?)),
            "event_key" => Ok(Props::EventKey(value_bool(value.1)?)),
            "block_child_events" => Ok(Props::BlockChildEvents(value_bool(value.1)?)),
            _ => {
                
                if let Ok(prop) = Walk::try_from(value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to(
                        "GenUI Props",
                        &format!("Makepad GView Prop, Invalid Prop: {}", value.0),
                    )
                    .into());
                }
            }
        }
    }
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
            "hover_color" => Ok(Props::HoverColor(MakepadColor::try_from((&value.1, None))?)),
            "focus_color" => Ok(Props::FocusColor(MakepadColor::try_from((&value.1, None))?)),
            "border_color" => Ok(Props::BorderColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "border_width" => Ok(Props::BorderWidth(value.1.as_f32()?.into())),
            "border_radius" => Ok(Props::BorderRadius(value.1.as_f32()?.into())),
            "visible" => Ok(Props::Visible(value.1.as_bool()?)),
            "background_visible" => Ok(Props::BackgroundVisible(value.1.as_bool()?)),
            "shadow_color" => Ok(Props::ShadowColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            "spread_radius" => Ok(Props::SpreadRadius(value.1.as_f32()?.into())),
            "blur_radius" => Ok(Props::BlurRadius(value.1.as_f32()?.into())),
            "shadow_offset" => Ok(Props::ShadowOffset(Vec2::try_from(&value.1)?)),
            "cursor" => Ok(Props::Cursor(MouseCursor::try_from(&value.1)?)),
            "animation_key" => Ok(Props::AnimationKey(value.1.as_bool()?)),
            "grab_key_focus" => Ok(Props::GrabKeyFocus(value.1.as_bool()?)),
            "block_signal_event" => Ok(Props::BlockSignalEvent(value.1.as_bool()?)),
            // "min_width" => Ok(Props::MinWidth(value.1.as_f32()?)),
            // "min_height" => Ok(Props::MinHeight(value.1.as_f32()?)),
            // "max_width" => Ok(Props::MaxWidth(value.1.as_f32()?)),
            // "max_height" => Ok(Props::MaxHeight(value.1.as_f32()?)),
            // "walk" => Ok(Props::Walk(Walk::try_from(&value.1)?)),
            // "layout" => Ok(Props::Layout(Layout::try_from(&value.1)?)),
            "event_order" => Ok(Props::EventOrder(EventOrder::try_from(&value.1)?)),
            "dpi_factor" => Ok(Props::DpiFactor(value.1.as_f64()?.into())),
            "optimize" => Ok(Props::Optimize(ViewOptimize::try_from(&value.1)?)),
            "capture_overload" => Ok(Props::CaptureOverload(value.1.as_bool()?)),
            "event_key" => Ok(Props::EventKey(value.1.as_bool()?)),
            "block_child_events" => Ok(Props::BlockChildEvents(value.1.as_bool()?)),
            _ => {
                if let Ok(prop) = Walk::try_from(&value) {
                    return Ok(Props::Walk(prop));
                } else if let Ok(prop) = Layout::try_from(&value) {
                    return Ok(Props::Layout(prop));
                } else {
                    return Err(err_from_to(
                        "GenUI Props",
                        &format!("Makepad GView Prop, Invalid Prop: {}", value.0.name()),
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
    Props::BorderWidth => border_width, false,
    Props::BorderRadius => border_radius, false,
    Props::Visible => visible, false,
    Props::BackgroundVisible => background_visible, false,
    Props::ShadowColor => shadow_color, false,
    Props::SpreadRadius => spread_radius, false,
    Props::BlurRadius => blur_radius, false,
    Props::ShadowOffset => shadow_offset, false,
    Props::Cursor => cursor, false,
    Props::AnimationKey => animation_key, false,
    Props::GrabKeyFocus => grab_key_focus, false,
    Props::BlockSignalEvent => block_signal_event, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::EventOrder => event_order, false,
    Props::DpiFactor => dpi_factor, false,
    Props::Optimize => optimize, false,
    Props::CaptureOverload => capture_overload, false,
    Props::EventKey => event_key, false,
    Props::BlockChildEvents => block_child_events, false
}
