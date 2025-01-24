use gen_utils::error::Error;
use crate::visitor::InstanceOutput;

pub trait FnVisitorImpl {
    /// 设置是否为可变方法，如果是true那就不需要改变了
    fn set_mut(&mut self, is_mut: bool);
    fn has_widget(&self, id: &str) -> bool;
    fn instance(&self) -> Option<&InstanceOutput>;
    fn get_widget_name(&self, id: &str) -> Result<String, Error>;
}
