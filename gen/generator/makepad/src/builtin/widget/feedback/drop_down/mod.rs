

mod event;
mod prop;


pub use event::*;
pub use prop::Props as DropDownProps;

use crate::{
    builtin::{
        prop::{FromGenProps, Prop},
        widget::WidgetImpl,
    },
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct DropDown {
    pub prop: Option<Prop<DropDownProps>>,
}


try_from_props! {
    DropDown {
       |props|  Ok(Self { prop: Prop::<DropDownProps>::from_prop(props)? })
    }
}

to_live_design!(DropDown: "GDropDown");

impl WidgetImpl for DropDown {
    type EventType = DropDownEvent;
}

impl TwoWayBindImpl for DropDown {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "opened"{
            Some(DropDownEvent::Changed.to_string())
        }else{
            None
        }
    }
}
