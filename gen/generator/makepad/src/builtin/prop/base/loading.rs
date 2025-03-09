use crate::try_from_enum_one_leaf;

#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub enum GLoadingType {
    #[default]
    Circle,
    DotLine,
    CircleDot,
}

try_from_enum_one_leaf! {
    GLoadingType, "GLoadingType",
    GLoadingType::Circle = "Circle",
    GLoadingType::DotLine = "DotLine",
    GLoadingType::CircleDot = "CircleDot"
}
