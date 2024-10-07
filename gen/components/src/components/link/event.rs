use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

use super::types::LinkType;

#[derive(Clone, Debug, DefaultNone)]
pub enum GLinkEvent {
    Hover(FingerHoverEvent),
    /// clicked(key_modifiers, href, link_type)
    Clicked(GLinkClickedParam),
    Released(FingerUpEvent),
    Pressed(FingerDownEvent),
    None,
}

#[derive(Debug, Clone)]
pub struct GLinkClickedParam {
    pub href: Option<String>,
    pub ty: LinkType,
    pub e: FingerUpEvent,
}
