mod conf;

pub use conf::RootConf;

use gen_utils::err_from_to;
use proc_macro2::TokenStream;
use syn::parse_str;

use crate::{
    token::ToLiveDesign, try_from_props,
    two_way_binding::TwoWayBindImpl,
};

#[derive(Debug, Clone)]
pub struct Root;

try_from_props! {
    Root {
        |props| {
            if props.is_some() {
                return Err(err_from_to!("GenUI Props" => "Makepad Root Prop, Root has no props"));
            }
            Ok(Self)
        }
    }
}

impl ToLiveDesign for Root {
    fn name(&self) -> proc_macro2::TokenStream {
        parse_str::<TokenStream>("Root").unwrap()
    }

    fn props(&self) -> Option<proc_macro2::TokenStream> {
        None
    }
}

impl TwoWayBindImpl for Root {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}
