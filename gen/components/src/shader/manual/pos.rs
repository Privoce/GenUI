use makepad_widgets::*;

#[derive(Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Direction {
    #[pick]
    Horizontal = shader_enum(1),
    Vertical = shader_enum(2),
}

#[derive(Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Position4 {
    Left = shader_enum(1),
    Right,
    Top,
    #[pick]
    Bottom,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Position {
    Left = shader_enum(1),
    LeftTop = shader_enum(2),
    LeftBottom = shader_enum(3),
    Right = shader_enum(4),
    RightTop = shader_enum(5),
    RightBottom = shader_enum(6),
    Top = shader_enum(7),
    TopLeft = shader_enum(8),
    TopRight = shader_enum(9),
    #[pick]
    Bottom = shader_enum(10),
    BottomLeft = shader_enum(11),
    BottomRight = shader_enum(12),
}

impl Position {
    pub fn to_drawer(&self) -> Self {
        match self {
            Position::Left | Position::LeftTop | Position::LeftBottom => Position::Left,
            Position::Right | Position::RightTop | Position::RightBottom => Position::Right,
            Position::Top | Position::TopLeft | Position::TopRight => Position::Top,
            Position::Bottom | Position::BottomLeft | Position::BottomRight => Position::Bottom,
        }
    }
}
