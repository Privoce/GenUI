#[macro_export]
macro_rules! from_builtin_widget {
    ($($F: ty => $T: path),*) => {
        $(
            impl From<$F> for BuiltinWidget {
                fn from(w: $F) -> Self {
                    $T(w)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! try_from_props {
    ($T:ty {
        $code: expr
    }) => {
        impl TryFrom<gen_parser::Props> for $T {
            type Error = gen_utils::error::Error;

            fn try_from(props: gen_parser::Props) -> Result<Self, Self::Error> {
                crate::builtin::prop::props_callback(props, $code)
            }
        }
    };
}
