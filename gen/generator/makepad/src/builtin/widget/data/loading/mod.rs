mod prop;
mod event;

pub use event::*;
pub use prop::Props as LoadingProps;
use crate::{
    builtin::{prop::{FromGenProps, Prop}, widget::WidgetImpl},
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};

#[derive(Debug, Clone)]
pub struct Loading {
    pub prop: Option<Prop<LoadingProps>>,
}

try_from_props! {
    Loading {
       |props| Ok(Self { prop: Prop::<LoadingProps>::from_prop(props)? })
    }
}

to_live_design!(Loading: "GLoading");

impl WidgetImpl for Loading {
    type EventType = LoadingEvent;
}

impl TwoWayBindImpl for Loading {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
