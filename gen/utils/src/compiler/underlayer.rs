use std::{any::Any, fmt::{Debug, Display}};

use toml_edit::Item;

pub trait UnderlayerConfImpl: Debug + Display + Any {
    /// ## convert to toml item
    fn to_item(&self) -> Item;
    fn as_any(&self) -> &dyn Any;
}
