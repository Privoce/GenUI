use proc_macro2::TokenStream;

use self::live_design::LiveDesign;

pub mod app_main;
pub mod attr;
pub mod field;
pub mod handler;
pub mod live_design;
pub mod match_event;
pub mod role;
pub mod traits;
pub mod widget;
pub mod live_hook;
pub mod auto_builtin_widgets;
pub mod safe_widget;
pub mod safe_traits;

pub trait ToLiveDesign {
    fn widget_tree(&self) -> Option<TokenStream>;
    fn widget_logic(&self) -> Option<TokenStream>;
    fn widget_imports(&self) -> Option<TokenStream>;
    fn widget_uses(&self) -> Option<TokenStream>;
    fn to_live_design(&self) -> LiveDesign;
}
