use makepad_widgets::*;

#[derive(Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum PopupMode {
    #[pick] Popup = shader_enum(1),
    ToolTip,
    Dialog
}