mod prop;
mod event;

pub use event::*;
pub use prop::Props as LinkProps;

use crate::{
    builtin::{prop::{FromGenProps, Prop}, widget::WidgetImpl},
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct Link {
    pub prop: Option<Prop<LinkProps>>,
}

try_from_props! {
    Link {
       |props|  Ok(Self { prop: Prop::<LinkProps>::from_prop(props)? })
    }
}

to_live_design!(Link: "GLink");

impl WidgetImpl for Link {
    type EventType = LinkEvent;
}

impl TwoWayBindImpl for Link {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
