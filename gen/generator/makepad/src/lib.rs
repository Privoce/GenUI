use gen_utils::common::token_stream_to_tree;

use proc_macro2::{TokenStream, TokenTree};

/// makepad compiler
pub mod compiler;
/// makepad model
pub mod model;
/// makepad widget prop
pub mod prop;
pub mod utils;
/// makepad widget
pub mod widget;

pub trait ToToken {
    fn to_token_stream(&self) -> TokenStream;
    fn to_token_trees(&self) -> Vec<TokenTree> {
        token_stream_to_tree(self.to_token_stream())
    }
}
