use makepad_widgets::*;

#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum PopupMode {
    #[pick] Popup = shader_enum(1),
    ToolTip,
    Dialog
}

#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum TriggerMode {
    #[pick] Click = shader_enum(1),
    Hover = shader_enum(2)
}

#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum ComponentMode {
    #[pick] Real = shader_enum(1),
    Virtual = shader_enum(2)
}