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
pub use prop::Props as TagProps;

#[derive(Debug, Clone)]
pub struct Tag {
    pub prop: Option<Prop<TagProps>>,
}

try_from_props! {
    Tag {
       |props| Ok(Self { prop: Prop::<TagProps>::from_prop(props)? })
    }
}

to_live_design!(Tag: "GTag");

impl WidgetImpl for Tag {
    type EventType = TagEvent;
}

impl TwoWayBindImpl for Tag {
    fn twb_event(prop: &str) -> Option<String> {
        if prop == "visible" {
            Some(TagEvent::Closed.to_string())
        } else {
            None
        }
    }
}
