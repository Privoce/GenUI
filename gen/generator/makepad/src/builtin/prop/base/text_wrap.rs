use crate::try_from_enum_one_leaf;

#[derive(Debug, Clone, Copy, Default)]
pub enum TextWrap {
    #[default]
    Ellipsis,
    Word,
    Line,
}

try_from_enum_one_leaf! {
    TextWrap, "TextWrap",
    TextWrap::Ellipsis = "Ellipsis",
    TextWrap::Word = "Word",
    TextWrap::Line = "Line"
}
