use crate::try_from_enum_one_leaf;

#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub enum GChooseType {
    #[default]
    Round,
    Tick,
    Cross,
}

try_from_enum_one_leaf! {
    GChooseType, "GChooseType",
    GChooseType::Round = "Round",
    GChooseType::Tick = "Tick",
    GChooseType::Cross = "Cross"
}
