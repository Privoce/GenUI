mod prop;

use crate::{builtin::prop::{FromGenProps, Prop}, to_live_design, try_from_props, two_way_binding::TwoWayBindImpl};
pub use prop::Props as DividerProps;

#[derive(Debug, Clone)]
pub struct Divider {
    pub prop: Option<Prop<DividerProps>>,
}

try_from_props!{
    Divider {
        |props| Ok(Self { prop: Prop::<DividerProps>::from_prop(props)? })
    }
}

to_live_design!(Divider: "GDivider");

impl TwoWayBindImpl for Divider {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
