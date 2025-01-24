use proc_macro2::TokenStream;
use syn::parse_str;
use crate::{split_fixed_impl, common::{snake_to_camel, camel_to_snake, Ulid}};

/// ## Split a string by a fixed pattern
/// In Rust, when you use the split method on a string (or a slice of characters) with a pattern
/// that isn't found in the string, it will indeed return a vector containing a single empty string slice,
/// `vec![""]`. This behavior is a part of how Rust's split method is designed,
/// and it might be unexpected if you're not familiar with the details of how split works.
pub fn split_fixed(input: &str, pat: &str) -> Vec<String> {
    input
        .split(pat)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
}

/// # FixedString
/// Fixed String trait is used add some useful methods to the string type.
pub trait FixedString {
    /// ## Split a string by a fixed pattern
    /// - if the pattern is not found in the string, it will return an empty vector instead of `vec![""]`
    fn split_fixed(&self, pat: &str) -> Vec<String>;
    /// ## Split a string by a fixed pattern
    /// - if the length is 0 return None
    fn split_fixed_option(&self, pat: &str) -> Option<Vec<String>>;
    /// ## Judge the string is inner string
    /// - if the string is wrapped by `"`return true else false
    /// this fn will trim the string first
    fn is_inner_string(&self) -> bool;
    /// ## convert snake to camel
    fn snake_to_camel(&self) -> String;
    /// ## camel to snake
    fn camel_to_snake(&self) -> String;
    /// ## convert camel to snake which camel is contain ulid
    /// prefix is use to split, cause this string often format: `$widget_name$ulid`
    fn camel_to_snake_ulid(&self, prefix: Option<&str>) -> String;
    /// ## is str has ulid or not, split by prefix
    fn has_ulid(&self, prefix: &str) -> bool;
    /// ## to_token_stream
    fn parse_str_stream(&self) -> TokenStream;
}

split_fixed_impl!(String);
split_fixed_impl!(&str);