use std::fmt::Display;

use makepad_widgets::*;

#[derive(Copy, Clone, Live, LiveHook, Debug)]
#[live_ignore]
pub enum LinkType {
    #[pick]
    NewTab,
    SameTab,
}

impl Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkType::NewTab => f.write_str("New Tab"),
            LinkType::SameTab => f.write_str("Same Tab"),
        }
    }
}
