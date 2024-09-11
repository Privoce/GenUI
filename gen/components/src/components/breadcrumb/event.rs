use makepad_widgets::{DefaultNone, KeyModifiers};

#[derive(Clone, Debug, DefaultNone)]
pub enum GBreadCrumbEvent {
    Hover(GBreadCrumbEventParam),
    Clicked(GBreadCrumbEventParam),
    ToHome,
    None,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbEventParam{
    pub index: usize,
    pub item: String,
    pub key_modifiers: KeyModifiers,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GBreadCrumbItemEvent {
    Hover(GBreadCrumbEventItemParam),
    Clicked(GBreadCrumbEventItemParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GBreadCrumbEventItemParam{
    pub item: String,
    pub key_modifiers: KeyModifiers,
}