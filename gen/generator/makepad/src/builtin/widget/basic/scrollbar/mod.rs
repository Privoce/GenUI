mod prop;

pub use prop::Props as ScrollBarsProps;

use crate::{
    builtin::prop::{FromGenProps, Prop},
    to_live_design, try_from_props,
};
#[derive(Debug, Clone)]
pub struct ScrollBars {
    pub prop: Option<Prop<ScrollBarsProps>>,
}

try_from_props! {
    ScrollBars {
       |props|  Ok(Self { prop: Prop::<ScrollBarsProps>::from_prop(props)? })
    }
}

to_live_design!(ScrollBars: "GScrollBars");
