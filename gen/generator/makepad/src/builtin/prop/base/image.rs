use crate::try_from_enum_one_leaf;

#[derive(Clone, PartialEq, Debug, Default, Copy)]
pub enum ImageFit {
    #[default]
    Stretch,
    Horizontal,
    Vertical,
    Smallest,
    Biggest,
    Size,
}

try_from_enum_one_leaf!{
    ImageFit, "ImageFit",
    ImageFit::Stretch = "Stretch",
    ImageFit::Horizontal = "Horizontal",
    ImageFit::Vertical = "Vertical",
    ImageFit::Smallest = "Smallest",
    ImageFit::Biggest = "Biggest",
    ImageFit::Size = "Size"
}