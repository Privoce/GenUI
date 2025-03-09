#[macro_export]
macro_rules! err_from_to {
    ($From: expr => $To: expr) => {
        gen_utils::error::Error::Convert(
            gen_utils::error::ConvertError::FromTo {
                from: $From.to_string(),
                to: $To.to_string(),
            }
            .into(),
        )
    };
}

#[macro_export]
macro_rules! err_from {
    ($E: expr) => {
        gen_utils::error::Error::from($E)
    };
}
