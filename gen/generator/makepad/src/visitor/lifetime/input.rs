use crate::{compiler::WidgetPoll, visitor::InstanceOutput};

pub struct Input {
    /// widget poll，用于存储上下文中含有ID的Widget
    /// 这些Widget可能会在方法中被访问到，根据被访问的id可以生成对应的方法
    /// 例如方法中访问了:
    /// `let mut my_label = c_ref!(my_label);`, 那么就可以生成一个方法: `let mut my_label = self.glabel(id!(my_label));`
    pub widget_poll: WidgetPoll,
    /// 实例, 实例能够知道在方法中那些部分需要被替换(提升)
    /// 例如: 实例名 = prop, fields = ["a", "b"]
    /// 那么方法中只要是使用了`prop.a`或者`prop.b`的地方都需要被替换为`self.a`或者`self.b`
    pub instance: Option<InstanceOutput>,
    /// 其他可能会被调用的方法名
    pub fn_names: Vec<String>,
}

impl Input {
    pub fn new(
        widget_poll: WidgetPoll,
        instance: Option<InstanceOutput>,
        fn_names: Vec<String>,
    ) -> Self {
        Self {
            widget_poll,
            instance,
            fn_names,
        }
    }
    pub fn get_widget_name(&self, id: &str) -> Option<&crate::model::AbsWidget> {
        self.widget_poll.get(id)
    }
}

