mod app_main;
mod live_hook;
mod match_event;
mod widget;
mod widget_match_event;

pub use app_main::AppMainTrait;
pub use live_hook::{ImplLiveHook, LiveHookTrait, LiveHookType};
pub use match_event::MatchEventTrait;
use proc_macro2::TokenStream;
pub use widget::*;
pub use widget_match_event::{WidgetMatchEventTrait, WidgetMatchEventType};

fn push_handle(target: &mut Option<TokenStream>, tk: TokenStream) {
    if let Some(handle) = target.as_mut() {
        handle.extend(tk);
    } else {
        target.replace(tk);
    }
}
