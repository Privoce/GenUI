use gen_parser::Props;
use gen_utils::error::Error;

pub trait FromGenProps {
    type Output;
    fn from_prop(prop: Props) -> Result<Option<Self::Output>, Error>;
}
