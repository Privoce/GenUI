use makepad_widgets::*;

live_design! {
    DrawGToolButton = {{DrawGToolButton}}{}
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGToolButton {
    #[live]
    button_type: GToolButtonType,
    #[deref]
    draw_super: DrawQuad,
}

#[derive(Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum GToolButtonType {
    Min = shader_enum(1),
    Max = shader_enum(2),
    FullScreen = shader_enum(3),
    Left = shader_enum(4),
    Right = shader_enum(5),
    More = shader_enum(6),
    Close = shader_enum(7),
    Up = shader_enum(8),
    Down = shader_enum(9),
    Switch = shader_enum(10),
    Exit = shader_enum(11),
    Expand = shader_enum(12),
    ExpandTop = shader_enum(13),
    ExpandBottom = shader_enum(14),
    ExpandLeft = shader_enum(15),
    ExpandRight = shader_enum(16),
    Upload = shader_enum(17),
    Download = shader_enum(18),
    Add = shader_enum(19),
    Delete = shader_enum(20),
    Correct = shader_enum(21),
    Fresh = shader_enum(22),
    Play = shader_enum(23),
    Stop = shader_enum(24),
    Setting = shader_enum(25),
    Bind = shader_enum(26),
    Menu = shader_enum(27),
    Emoji = shader_enum(28),
    Phone = shader_enum(29),
    #[pick]
    None = shader_enum(30),
}
