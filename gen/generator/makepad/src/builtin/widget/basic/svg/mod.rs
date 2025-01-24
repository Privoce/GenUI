mod event;
mod prop;

pub use event::*;
pub use prop::Props as SvgProps;

use crate::{builtin::{prop::{FromGenProps, Prop}, widget::WidgetImpl}, to_live_design, try_from_props, two_way_binding::TwoWayBindImpl};
#[derive(Debug, Clone)]
pub struct Svg {
    pub prop: Option<Prop<SvgProps>>,
}

try_from_props! {
    Svg {
       |props|  Ok(Self { prop: Prop::<SvgProps>::from_prop(props)? })
    }
}

to_live_design!(Svg: "GSvg");

impl WidgetImpl for Svg {
    type EventType = SvgEvent;
}

impl TwoWayBindImpl for Svg {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
