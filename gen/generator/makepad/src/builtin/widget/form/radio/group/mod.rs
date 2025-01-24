mod event;
mod prop;

use crate::{
    builtin::{
        prop::{FromGenProps, Prop},
        widget::WidgetImpl,
    },
    to_live_design, try_from_props,
    two_way_binding::TwoWayBindImpl,
};
pub use event::*;
pub use prop::Props as RadioGroupProps;
#[derive(Debug, Clone)]
pub struct RadioGroup {
    pub prop: Option<Prop<RadioGroupProps>>,
}

try_from_props! {
    RadioGroup {
       |props|  Ok(Self { prop: Prop::<RadioGroupProps>::from_prop(props)? })
    }
}

to_live_design!(RadioGroup: "GRadioGroup");

impl WidgetImpl for RadioGroup {
    type EventType = RadioGroupEvent;
}

impl TwoWayBindImpl for RadioGroup {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "selected" {
            Some(RadioGroupEvent::Changed.to_string())
        } else {
            None
        }
    }
}
