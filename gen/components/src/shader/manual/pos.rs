use makepad_widgets::*;

#[derive(Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Direction {
    #[pick]
    Horizontal = shader_enum(1),
    Vertical = shader_enum(2),
}

impl Direction {
    pub fn to_f32(&self) -> f32 {
        match self {
            Direction::Horizontal => 1.0,
            Direction::Vertical => 0.0,
        }
    }
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
    /// return angle offset
    pub fn angle_offset(&self, size: DVec2) -> f32 {
        match self {
            Position::Left | Position::Right | Position::Bottom | Position::Top => 0.0,
            Position::LeftTop
            | Position::LeftBottom
            | Position::RightTop
            | Position::RightBottom => (size.y / 2.0) as f32,
            Position::TopLeft
            | Position::TopRight
            | Position::BottomLeft
            | Position::BottomRight => (size.x / 2.0) as f32,
        }
    }
}
