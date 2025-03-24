mod prop;
mod event;

#[derive(Debug, Clone)]
pub struct Router {
    pub prop: Option<Prop<RouterProps>>,
}

try_from_props! {
    Router {
       |props| Ok(Self { prop: Prop::<RouterProps>::from_prop(props)? })
    }
}