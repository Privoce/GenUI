mod event;
mod prop;
mod group;

pub use group::*;
pub use event::*;
pub use prop::Props as RadioProps;

use crate::{
    builtin::{
        prop::{FromGenProps, Prop},
        widget::WidgetImpl,
    },
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct Radio {
    pub prop: Option<Prop<RadioProps>>,
}


try_from_props! {
    Radio {
       |props|  Ok(Self { prop: Prop::<RadioProps>::from_prop(props)? })
    }
}

to_live_design!(Radio: "GRadio");

impl WidgetImpl for Radio {
    type EventType = RadioEvent;
}

impl TwoWayBindImpl for Radio {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "selected"{
            Some(RadioEvent::Clicked.to_string())
        }else{
            None
        }
    }
}
