#[macro_export]
macro_rules! as_value {
    ($($F: ident, $T: ty, $To: expr => $P: path),*) => {
        $(
            pub fn $F(&self) -> Result<$T, Error> {
                match self {
                    $P(b) => Ok(b.clone()),
                    _ => Err(ConvertError::FromTo {
                        from: self.to_string(),
                        to: $To.to_string(),
                    }.into())
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! parse_base_value {
    ($($FName: ident, $T: ty => $P: path ,($FStr: expr => $TStr: expr)),*) => {
        $(
            fn $FName(s: &str) -> Result<Value, Error> {
                match s.parse::<$T>() {
                    Ok(v) => Ok($P(v)),
                    Err(_) => Err(ConvertError::FromTo {
                        from: $FStr.to_string(),
                        to: $TStr.to_string(),
                    }
                    .into()),
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! nom_err {
    ($input: expr, $e_kind: path) => {
        nom::Err::Error(nom::error::Error::new($input, $e_kind))
    };
}