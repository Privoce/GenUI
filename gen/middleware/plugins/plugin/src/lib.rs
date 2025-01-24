//! # Plugin transform module
//! this module is responsible for transforming the plugin into a token in GenUI framework.
//! If you write a GenUI plugin, you need to use this module to connect your plugin to the GenUI framework.
//! ## Example
//! After you write a network plugin, you need to define a `token.toml` file in the root of your plugin.
//! ```toml
//! [plugin]
//! name = "network-http"
//! category = "network"
//!
//! [macros]
//! http_get = { category = "prop_macro"}
//! ```
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
mod extern_struct;
mod token;

pub use extern_struct::*;
pub use token::*;
