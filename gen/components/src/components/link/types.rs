use makepad_widgets::*;

#[derive(Copy, Clone, Live, LiveHook, Debug)]
#[live_ignore]
pub enum LinkType {
    #[pick]
    NewTab,
    SameTab,
}
