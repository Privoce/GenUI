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
pub use prop::Props as CheckboxGroupProps;
#[derive(Debug, Clone)]
pub struct CheckboxGroup {
    pub prop: Option<Prop<CheckboxGroupProps>>,
}

try_from_props! {
    CheckboxGroup {
       |props|  Ok(Self { prop: Prop::<CheckboxGroupProps>::from_prop(props)? })
    }
}

to_live_design!(CheckboxGroup: "GCheckboxGroup");

impl WidgetImpl for CheckboxGroup {
    type EventType = CheckboxGroupEvent;
}

impl TwoWayBindImpl for CheckboxGroup {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "selected" {
            Some(CheckboxGroupEvent::Changed.to_string())
        } else {
            None
        }
    }
}
