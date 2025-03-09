use crate::try_from_enum_one_leaf;

#[derive(Clone, PartialEq, Debug, Copy, Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

try_from_enum_one_leaf! {
    Direction, "Direction",
    Direction::Horizontal = "Horizontal",
    Direction::Vertical = "Vertical"
}

#[derive(Clone, PartialEq, Debug, Copy, Default)]
pub enum Position4 {
    Left,
    Right,
    Top,
    #[default]
    Bottom,
}

try_from_enum_one_leaf! {
    Position4, "Position4",
    Position4::Left = "Left",
    Position4::Right = "Right",
    Position4::Top = "Top",
    Position4::Bottom = "Bottom"
}

#[derive(Clone, PartialEq, Debug, Copy, Default)]
pub enum Position {
    Left ,
    LeftTop,
    LeftBottom ,
    Right ,
    RightTop ,
    RightBottom ,
    Top,
    TopLeft ,
    TopRight,
    #[default]
    Bottom ,
    BottomLeft ,
    BottomRight ,
}

try_from_enum_one_leaf! {
    Position, "Position",
    Position::Left = "Left",
    Position::LeftTop = "LeftTop",
    Position::LeftBottom = "LeftBottom",
    Position::Right = "Right",
    Position::RightTop = "RightTop",
    Position::RightBottom = "RightBottom",
    Position::Top = "Top",
    Position::TopLeft = "TopLeft",
    Position::TopRight = "TopRight",
    Position::Bottom = "Bottom",
    Position::BottomLeft = "BottomLeft",
    Position::BottomRight = "BottomRight"
}