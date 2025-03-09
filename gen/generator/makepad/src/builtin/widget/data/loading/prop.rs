// #[live]
// pub theme: Themes,
// #[live]
// pub stroke_color: Option<Vec4>,
// // deref -------------------
// #[live]
// #[redraw]
// pub draw_loading: DrawGLoading,
// #[live]
// pub loading_type: GLoadingType,
// #[walk]
// pub walk: Walk,
// #[layout]
// pub layout: Layout,
// // frame -------------------
// #[live(true)]
// pub visible: bool,
// #[live(true)]
// pub animation_key: bool,
// #[live]
// pub time: f32,
// #[rust]
// next_frame: NextFrame,
// // store previous state(animation_key)
// #[rust]
// pub pre_state: bool,
// #[live(true)]
// pub event_key: bool,

use gen_analyzer::{value::Value, PropKey};
use gen_utils::err_from_to;

use crate::{
    builtin::prop::{GLoadingType, Layout, Prop, Themes, Walk},
    from_gen::MakepadColor,
    from_gen_props, props_to_tokens,
};

#[derive(Debug, Clone)]
pub enum Props {
    Theme(Themes),
    StrokeColor(MakepadColor),
    // DrawLoading(DrawGLoading),
    LoadingType(GLoadingType),
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
            "stroke_color" => Ok(Props::StrokeColor(MakepadColor::try_from((
                &value.1, None,
            ))?)),
            // "draw_loading" => Ok(Props::DrawLoading(DrawGLoading::try_from(&value.1)?)),
            "loading_type" => Ok(Props::LoadingType(GLoadingType::try_from(&value.1)?)),
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
                        "GenUI Props" => &format!("Makepad GLoading Prop, Invalid Prop: {}", value.0.name)
                    ).to_runtime("Makepad Compiler"));
                }
            }
        }
    }
}

props_to_tokens!{
    Props,
    Props::Theme => theme, false,
    Props::StrokeColor => stroke_color, false,
    // Props::DrawLoading => draw_loading, false,
    Props::LoadingType => loading_type, false,
    Props::Walk => walk, true,
    Props::Layout => layout, true,
    Props::Visible => visible, false,
    Props::AnimationKey => animation_key, false,
    Props::EventKey => event_key, false
}