mod event;
mod prop;
mod error;
mod utils;

use event::impl_attr_event;
use proc_macro::TokenStream;
use prop::{impl_attr_prop, impl_proc_default_prop};

// Attr Macro: Prop -------------------------------------------------------------------------------------------

/// # Attr macro for the `Prop`
/// Attr macro aims to auto implement the `Live` and `Widget` traits for the struct.
/// It will finally convert to [After Compilation](#after-compilation) code.
/// > NOTE:
/// >
/// > This macro is a low-level macro for GenUI. All deref will be `GView` (inherits)
/// >
/// > If you need to create a powerful UI, see [Component Macro(comming soon)]
/// ## Example
/// ```rust
/// #[prop]
/// pub struct AProp {
///    pub a: i32,
/// }
/// ```
/// ## After Compilation
/// ```rust
/// #[derive(Live, Widget)]
/// pub struct AProp {
///    #[live]
///    pub a: i32,
///    #[deref]
///    pub deref_widget: GView,
/// }
/// ```
#[proc_macro_attribute]
pub fn prop(_attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_attr_prop(item)
}
// -----------------------------------------------------------------------------------------------------------------

// default_prop! ---------------------------------------------------------------------------------------------------
/// # Proc Default Prop Macro `default_prop!`
/// ## Example
/// ```rust
/// let mut prop = default_prop!{
///     MyStruct{
///         a: 10,
///     }
/// }
/// ```
#[proc_macro]
pub fn default_prop(input: TokenStream) -> TokenStream {
    impl_proc_default_prop(input)
}
// -----------------------------------------------------------------------------------------------------------------

// Attr Macro: Event -----------------------------------------------------------------------------------------------

/// # Attr macro for the `Event`
/// Attr macro aims to impl `DefaultNone` and `Clone`
/// ## Example
/// ```rust
/// #[event]
/// #[derive(Debug, Clone)]
/// pub enum AEvent {
///   ABtnClick,
/// }
/// ```
/// ## After Compilation
/// ```rust
/// #[derive(DefaultNone, Clone, Debug)]
/// pub enum AEvent {
///  ABtnClick,
/// }
/// ```
#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_attr_event(item)
}
// -----------------------------------------------------------------------------------------------------------------