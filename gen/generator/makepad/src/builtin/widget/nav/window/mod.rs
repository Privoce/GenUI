mod prop;

pub use prop::{default_window_props, Props as WindowProps};

use crate::{
    builtin::prop::{FromGenProps, Prop},
    to_live_design, try_from_props,
};

#[derive(Debug, Clone)]
pub struct Window {
    pub prop: Option<Prop<WindowProps>>,
}

try_from_props! {
    Window {
       |props| Ok(Self { prop: Prop::<WindowProps>::from_prop(props)? })
    }
}

to_live_design!(Window: "GWindow");
