/// impl for FixedString trait ----------------------------------------------------------------
#[macro_export]
macro_rules! split_fixed_impl {
    ($Str: ty) => {
        impl FixedString for $Str {
            fn split_fixed(&self, pat: &str) -> Vec<String> {
                split_fixed(self, pat)
            }

            fn split_fixed_option(&self, pat: &str) -> Option<Vec<String>> {
                let res = self.split_fixed(pat);
                if res.is_empty() {
                    None
                } else {
                    Some(res)
                }
            }
            fn is_inner_string(&self) -> bool {
                self.trim().starts_with('"') && self.ends_with('"')
            }
            fn snake_to_camel(&self) -> String {
                snake_to_camel(&self)
            }

            fn camel_to_snake(&self) -> String {
                camel_to_snake(&self)
            }

            fn camel_to_snake_ulid(&self, prefix: Option<&str>) -> String {
                if let Some(prefix) = prefix {
                    let (prefix_str, ulid) = self.split_once(prefix).unwrap();
                    if !prefix_str.is_empty() {
                        panic!("prefix split error, after split should be empty")
                    }
                    format!("{}{}", prefix.camel_to_snake(), Ulid::from(ulid).to_snake())
                } else {
                    self.camel_to_snake()
                }
            }

            fn has_ulid(&self, prefix: &str) -> bool {
                if let Some((prefix_str, _)) = self.split_once(prefix) {
                    prefix_str.is_empty()
                } else {
                    false
                }
            }

            fn parse_str_stream(&self) -> TokenStream {
                parse_str::<TokenStream>(&self).unwrap()
            }

            fn strip_prefix_suffix(&self, prefix: &str, suffix: &str) -> Result<String, Error> {
                strip_prefix_suffix(self, prefix, suffix)
            }
        }
    };
}
// -----------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! from_u_number {
    ($from: ident) => {
        impl From<$from> for Value {
            fn from(value: $from) -> Self {
                Value::USize(value as usize)
            }
        }
    };
}
#[macro_export]
macro_rules! from_i_number {
    ($from: ident) => {
        impl From<$from> for Value {
            fn from(value: $from) -> Self {
                Value::ISize(value as isize)
            }
        }
    };
}

#[macro_export]
macro_rules! from_value_to_primitive {
    ($(
        $T: ty, $TStr: expr , $P: path => $V: expr
    ),*) => {
        $(
            impl TryFrom<Value> for $T{
                type Error = gen_utils::error::Error;

                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    if let $P(n) = value {
                        Ok(primitive_callback(n, $V))
                    } else {
                        Err(ConvertError::FromTo {
                            from: "Value".to_string(),
                            to: $TStr.to_string(),
                        }
                        .into())
                    }
                }
            }
        )*
    };
}
