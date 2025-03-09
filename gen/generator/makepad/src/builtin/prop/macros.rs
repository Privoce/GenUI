#[macro_export]
macro_rules! try_from_value_ref_enum {
    ($($T: ty , $TStr: expr),*) => {
        $(
            impl TryFrom<&Value> for $T {
                type Error = gen_utils::error::Error;

                fn try_from(value: &Value) -> Result<Self, <Self as TryFrom<&Value>>::Error> {
                    if let Value::Enum(e) = value {
                        return e.try_into();
                    } else if let Value::String(s) = value {
                        return s.parse();
                    } else {
                        return Err(gen_utils::err_from_to!("Value" => $TStr));
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! try_from_value_ref_struct {
    ($($T: ty, $TStr: expr, $C: ty),*) => {
        $(
            impl TryFrom<&Value> for $T {
                type Error = Error;

                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    if let Value::Struct(s) = value {
                        return s.try_into();
                    } else if let Value::UnKnown(s) = value {
                        return s.parse();
                    } else if let Value::Double(d) = value{
                        return (*d).try_into();
                    } else {
                        return Err(gen_utils::err_from_to!("Value" => $TStr));
                    }
                }
            }

            impl FromStr for $T {
                type Err = Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    convert_str_to_vec::<$C, ParseFloatError>(s).and_then(|vecs| vecs.try_into())
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! try_from_enum_one_leaf {
    ($T: ty, $S: expr ,$($P: path = $I: expr),*) => {
        impl TryFrom<&Vec<gen_analyzer::value::EnumItem>> for $T {
            type Error = gen_utils::error::Error;

            fn try_from(value: &Vec<gen_analyzer::value::EnumItem>) -> Result<Self, <Self as TryFrom<&gen_analyzer::value::EnumItem>>::Error> {
                if value.len() == 1 {
                    return value.get(0).unwrap().try_into();
                } else if value.len() == 2 {
                    let root = value.get(0).unwrap();
                    let leaf = value.get(1).unwrap();
                    if let gen_analyzer::value::EnumItem::Root(root) = root {
                        if root == $S {
                            return leaf.try_into();
                        }
                    }
                }
                Err(gen_utils::err_from_to!("EnumItem" => $S))
            }
        }

        impl TryFrom<&gen_analyzer::value::EnumItem> for $T {
            type Error = gen_utils::error::Error;

            fn try_from(value: &gen_analyzer::value::EnumItem) -> Result<Self, <Self as TryFrom<&gen_analyzer::value::EnumItem>>::Error> {
                match value {
                    gen_analyzer::value::EnumItem::Leaf(s, _) => Ok(s.parse()?),
                    _ => Err(gen_utils::err_from_to!("EnumItem" => $S)),
                }
            }
        }

        impl TryFrom<&gen_analyzer::value::Enum> for $T {
            type Error = gen_utils::error::Error;

            fn try_from(value: &gen_analyzer::value::Enum) -> Result<Self, <Self as TryFrom<&gen_analyzer::value::Enum>>::Error> {
                let gen_analyzer::value::Enum { field_chain } = value;
                field_chain.try_into()
            }
        }

        impl std::str::FromStr for $T {
            type Err = gen_utils::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($I => Ok($P),)*
                    _ => Err(gen_utils::err_from_to!("Value" => "EventOrder")),
                }
            }
        }

        impl quote::ToTokens for $T {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                let res = match self {
                    $($P => $I,)*
                };
                tokens.extend(syn::parse_str::<proc_macro2::TokenStream>(res));
            }
        }
    };
}

#[macro_export]
macro_rules! struct_float_to_tokens {
    ($(
        $T: ty {$($K: ident),*}
    ),*) => {
        $(
            impl quote::ToTokens for $T {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    $(
                        let val = self.$K as f64;
                        let $K = syn::parse_str::<proc_macro2::TokenStream>(&gen_utils::common::format_float(val)).unwrap();
                    )*
                    tokens.extend(quote::quote! {
                        {
                            $(
                                $K: #$K
                            ),*
                        }
                    });
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! struct_float_dvec_to_tokens {
    ($(
        $T: ty => $V:ident ($($K:ident),*)
    ),*) => {
        $(
            impl quote::ToTokens for $T {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    $(
                        let val = self.$K;
                        let $K = syn::parse_str::<proc_macro2::TokenStream>(&gen_utils::common::format_float(val)).unwrap();
                    )*
                    tokens.extend(quote::quote! {
                        $V($(
                            #$K
                        ),*)
                    });
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! props_to_tokens {
    ($T: ty, $($P: path => $V: expr, $Deref: expr),*) => {
        impl quote::ToTokens for $T {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match self {
                    $(
                        $P(v) => {
                            if $Deref{
                                tokens.extend(quote::ToTokens::to_token_stream(v));
                            }else{
                                let v = quote::ToTokens::to_token_stream(v);
                                tokens.extend(quote::quote! {
                                    $V: #v
                                });
                            }
                        },
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! try_from_f64_vec {
    ($($T:ty { $($field:ident),+ }),*) => {
        $(
            impl TryFrom<f64> for $T {
                type Error = Error;

                fn try_from(value: f64) -> Result<Self, Self::Error> {
                    Ok(Self {
                        $(
                            $field: value as _,
                        )+
                    })
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! from_gen_props {
    ($T: ty) => {
        impl crate::builtin::prop::FromGenProps for Prop<$T> {
            type Output = Prop<$T>;

            fn from_prop(
                prop: Option<gen_analyzer::Props>,
            ) -> Result<Option<Self::Output>, gen_utils::error::Error> {
                if let Some(props) = prop {
                    let mut res = Prop::default();
                    for (prop, value) in props {
                        if prop.is_normal() {
                            res.push((prop, value).try_into()?);
                        }
                    }
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            }
        }
    };
}
