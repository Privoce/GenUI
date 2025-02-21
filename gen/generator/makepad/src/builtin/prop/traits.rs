pub trait FromGenProps {
    type Output;
    fn from_prop(
        prop: Option<gen_analyzer::Props>,
    ) -> Result<Option<Self::Output>, gen_utils::error::Error>;
}
