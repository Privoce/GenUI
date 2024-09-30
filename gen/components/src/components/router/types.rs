use makepad_widgets::LiveId;

#[derive(Default, Debug)]
pub enum ActiveRouter {
    #[default]
    None,
    Active(LiveId),
}
