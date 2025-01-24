mod event;
mod prop;
mod group;

pub use group::*;
pub use event::*;
pub use prop::Props as CheckboxProps;

use crate::{
    builtin::{
        prop::{FromGenProps, Prop},
        widget::WidgetImpl,
    },
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct Checkbox {
    pub prop: Option<Prop<CheckboxProps>>,
}


try_from_props! {
    Checkbox {
       |props|  Ok(Self { prop: Prop::<CheckboxProps>::from_prop(props)? })
    }
}

to_live_design!(Checkbox: "GCheckbox");

impl WidgetImpl for Checkbox {
    type EventType = CheckboxEvent;
}

impl TwoWayBindImpl for Checkbox {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "selected"{
            Some(CheckboxEvent::Clicked.to_string())
        }else{
            None
        }
    }
}
