mod prop;
mod event;

pub use event::*;
pub use prop::Props as ButtonProps;
use crate::{
    builtin::{prop::{FromGenProps, Prop}, widget::WidgetImpl},
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};

#[derive(Debug, Clone)]
pub struct Button {
    pub prop: Option<Prop<ButtonProps>>,
}

try_from_props! {
    Button {
       |props| Ok(Self { prop: Prop::<ButtonProps>::from_prop(props)? })
    }
}

to_live_design!(Button: "GButton");

impl WidgetImpl for Button {
    type EventType = ButtonEvent;
}

impl TwoWayBindImpl for Button {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
