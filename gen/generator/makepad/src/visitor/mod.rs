//! Trans-Visitor模块，用于将RSSYin的抽象语法树转换为其他语言的抽象语法树
//! 在这个模块中，我们要处理很多的映射关系
//! 这些转换都是延迟的，也就是说，我们不会立即转换，而是在需要的时候才转换
//! 因为单纯在脚本处理的时候并不知道某些方法它是使用者写的辅助方法还是需要转为Makepad的方法

mod event;
mod r#fn;
mod instance;
mod lifetime;
mod prop;
mod style;
mod sugar;

pub use event::*;
pub use instance::InstanceLzVisitor;
pub use lifetime::*;
pub use prop::PropLzVisitor;
pub use r#fn::FnLzVisitor;
pub use style::{IdClass, StyleVisitor};
pub use sugar::*;
