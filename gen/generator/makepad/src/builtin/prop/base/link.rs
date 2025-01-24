use crate::try_from_enum_one_leaf;

#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub enum LinkType {
    #[default]
    NewTab,
    SameTab,
}

try_from_enum_one_leaf!{
    LinkType, "LinkType",
    LinkType::NewTab = "NewTab",
    LinkType::SameTab = "SameTab"
}