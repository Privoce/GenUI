mod event;
mod prop;

pub use event::*;
pub use prop::Props as LabelProps;

use crate::{
    builtin::{
        prop::{FromGenProps, Prop},
        widget::WidgetImpl,
    },
    to_live_design, try_from_props,
    two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct Label {
    pub prop: Option<Prop<LabelProps>>,
}

try_from_props! {
    Label {
       |props|  Ok(Self { prop: Prop::<LabelProps>::from_prop(props)? })
    }
}

to_live_design!(Label: "GLabel");

impl WidgetImpl for Label {
    type EventType = LabelEvent;
}

impl TwoWayBindImpl for Label {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
