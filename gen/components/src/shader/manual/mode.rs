use makepad_widgets::*;

/// The `PopupMode` enum represents the different modes for a popup
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum PopupMode {
    #[pick]
    Popup = shader_enum(1),
    ToolTip,
    Dialog,
    Drawer,
}

/// The `TriggerMode` enum represents the different modes for a trigger
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum TriggerMode {
    #[pick]
    Click = shader_enum(1),
    Hover = shader_enum(2),
}

/// The `ComponentMode` enum represents the different modes for a component
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum ComponentMode {
    #[pick]
    Real = shader_enum(1),
    Virtual = shader_enum(2),
}

/// The `UploadMode` enum represents the different modes for uploading
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum UploadMode {
    Folder = shader_enum(1),
    Folders = shader_enum(2),
    #[pick]
    File = shader_enum(3),
    Files = shader_enum(4),
}

impl UploadMode {
    pub fn is_multi(&self) -> bool {
        match self {
            UploadMode::Folder | UploadMode::File => false,
            UploadMode::Folders | UploadMode::Files => true,
        }
    }
}

#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum WindowButtonMode{
    Desktop,
    #[pick]
    Tool
}