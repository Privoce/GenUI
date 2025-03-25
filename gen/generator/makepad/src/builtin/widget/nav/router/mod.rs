mod event;
mod prop;

use crate::{
    builtin::prop::{FromGenProps, Prop},
    try_from_props,
};
pub use prop::Props as RouterProps;

#[derive(Debug, Clone)]
pub struct Router {
    pub prop: Option<Prop<RouterProps>>,
}

try_from_props! {
    Router {
       |props| Ok(Self { prop: Prop::<RouterProps>::from_prop(props)? })
    }
}