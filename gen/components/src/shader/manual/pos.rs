use makepad_widgets::*;

#[derive(Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Direction {
    #[pick] Horizontal = shader_enum(1),
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
    Bottom
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Position {
    Left = shader_enum(1),
    LeftTop,
    LeftBottom,
    Right,
    RightTop,
    RightBottom,
    Top,
    TopLeft,
    TopRight,
    #[pick]
    Bottom,
    BottomLeft,
    BottomRight
}
