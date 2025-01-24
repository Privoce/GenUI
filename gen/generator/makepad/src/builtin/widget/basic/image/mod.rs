mod prop;
mod event;

pub use event::*;
pub use prop::Props as ImageProps;

use crate::{
    builtin::{prop::{FromGenProps, Prop}, widget::WidgetImpl},
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};

#[derive(Debug, Clone)]
pub struct Image {
    pub prop: Option<Prop<ImageProps>>,
}

try_from_props! {
    Image {
       |props|  Ok(Self { prop: Prop::<ImageProps>::from_prop(props)? })
    }
}

to_live_design!(Image: "GImage");

impl WidgetImpl for Image {
    type EventType = ImageEvent;
}

impl TwoWayBindImpl for Image {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
