//! 设计Makepad中的双向绑定
//! ```rust
//! <checkbox id="check" text="check" :selected="checked" />
//! <checkbox id="check2" text="check2" :selected="checked" />
//! 
//! #[derive(Live, Widget)]
//! struct Component1 {
//!     #[deref]
//!     deref_widget: GView,
//!     #[live]
//!     checked: bool, // checkbox的选中状态和selected绑定
//!     #[rust]
//!     on_checked_change: Option<Box<dyn Fn(&mut Cx, bool) -> ()>>,
//! }
//! ```
//! 当`checked`的值发生变化时，会调用`on_checked_change`方法

mod twb_poll;
mod get_set;

pub use twb_poll::*;
pub use get_set::*;

pub trait TwoWayBindImpl {
    fn twb_event(prop: &str) -> Option<String>;
}