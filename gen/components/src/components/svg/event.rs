use std::path::PathBuf;

use makepad_widgets::{DefaultNone, KeyModifiers};

#[derive(Debug, Clone, DefaultNone)]
pub enum GSvgEvent {
    Clicked(GSvgEventParam),
    Hover(GSvgEventParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GSvgEventParam{
    pub src: PathBuf,
    pub key_modifiers: KeyModifiers,
}
