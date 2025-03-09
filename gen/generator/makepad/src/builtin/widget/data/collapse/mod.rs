mod prop;
mod event;

pub use event::*;
pub use prop::Props as CollapseProps;
use crate::{
    builtin::{prop::{FromGenProps, Prop}, widget::WidgetImpl},
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};

#[derive(Debug, Clone)]
pub struct Collapse {
    pub prop: Option<Prop<CollapseProps>>,
}

try_from_props! {
    Collapse {
       |props| Ok(Self { prop: Prop::<CollapseProps>::from_prop(props)? })
    }
}

to_live_design!(Collapse: "GCollapse");

impl WidgetImpl for Collapse {
    type EventType = CollapseEvent;
}

impl TwoWayBindImpl for Collapse {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "opened"{
            Some(CollapseEvent::Opened.to_string())
        }else{
            None
        }
    }
}
