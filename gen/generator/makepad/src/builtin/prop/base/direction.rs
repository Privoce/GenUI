use crate::try_from_enum_one_leaf;

#[derive(Clone, PartialEq, Debug, Copy, Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

try_from_enum_one_leaf!{
    Direction, "Direction",
    Direction::Horizontal = "Horizontal",
    Direction::Vertical = "Vertical"
}

