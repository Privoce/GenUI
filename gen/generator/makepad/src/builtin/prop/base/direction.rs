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
