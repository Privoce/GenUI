use proc_macro2::TokenStream;

mod global;

pub trait ToLiveDesign {
    fn name(&self) -> TokenStream;
    fn props(&self) -> Option<TokenStream>;
}

#[macro_export]
macro_rules! to_live_design {
    ($T: ty : $N: expr) => {
        impl crate::token::ToLiveDesign for $T {
            fn name(&self) -> proc_macro2::TokenStream {
                syn::parse_str::<proc_macro2::TokenStream>($N).unwrap()
            }

            fn props(&self) -> Option<proc_macro2::TokenStream> {
                self.prop.as_ref().map(|x| quote::ToTokens::to_token_stream(x))
            }
        }
    };
}

#[macro_export]
macro_rules! to_live_design_inherits {
    ($T: ty : $N: expr) => {
        impl crate::token::ToLiveDesign for $T {
            fn name(&self) -> proc_macro2::TokenStream {
                syn::parse_str::<proc_macro2::TokenStream>($N).unwrap()
            }

            fn props(&self) -> Option<proc_macro2::TokenStream> {
                self.0.prop.as_ref().map(|x| quote::ToTokens::to_token_stream(x))
            }
        }
    };
}