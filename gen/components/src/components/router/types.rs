use makepad_widgets::{Live, LiveId};

#[derive(Default, Debug)]
pub enum ActiveRouter {
    #[default]
    None,
    Active(LiveId),
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PageType{
    #[default]
    Bar,
    Nav,
    /// no default display page
    None,
}