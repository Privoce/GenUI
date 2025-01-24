mod event;
mod prop;

pub use event::*;
pub use prop::Props as InputProps;

use crate::{
    builtin::{
        prop::{FromGenProps, Prop},
        widget::WidgetImpl,
    },
    to_live_design, try_from_props,
    two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct Input {
    pub prop: Option<Prop<InputProps>>,
}

try_from_props! {
    Input {
       |props|  Ok(Self { prop: Prop::<InputProps>::from_prop(props)? })
    }
}

to_live_design!(Input: "GInput");

impl WidgetImpl for Input {
    type EventType = InputEvent;
}

impl TwoWayBindImpl for Input {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "text" {
            Some(InputEvent::Changed.to_string())
        } else {
            None
        }
    }
}
