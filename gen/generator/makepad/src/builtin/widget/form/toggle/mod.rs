mod event;
mod prop;

pub use event::*;
pub use prop::Props as ToggleProps;

use crate::{
    builtin::{
        prop::{FromGenProps, Prop},
        widget::WidgetImpl,
    },
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct Toggle {
    pub prop: Option<Prop<ToggleProps>>,
}


try_from_props! {
    Toggle {
       |props|  Ok(Self { prop: Prop::<ToggleProps>::from_prop(props)? })
    }
}

to_live_design!(Toggle: "GToggle");

impl WidgetImpl for Toggle {
    type EventType = ToggleEvent;
}

impl TwoWayBindImpl for Toggle {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "selected"{
            Some(ToggleEvent::Clicked.to_string())
        }else{
            None
        }
    }
}
