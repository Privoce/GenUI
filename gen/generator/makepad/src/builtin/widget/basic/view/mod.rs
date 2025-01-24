mod prop;

pub use prop::Props as ViewProps;

use crate::{
    builtin::prop::{FromGenProps, Prop},
    to_live_design, try_from_props, two_way_binding::TwoWayBindImpl,
};

#[derive(Debug, Clone)]
pub struct View {
    pub prop: Option<Prop<ViewProps>>,
}

try_from_props! {
    View {
       |props|  Ok(Self { prop: Prop::<ViewProps>::from_prop(props)? })
    }
}

to_live_design!(View: "GView");

impl TwoWayBindImpl for View {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
