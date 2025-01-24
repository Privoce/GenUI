use crate::{compiler::WidgetPoll, model::CallbackWidget, visitor::InstanceOutput};

/// # Input for FnLzVisitor
/// 用于FnLzVisitor的输入，这些都是FnLzVisitor需要的数据
#[derive(Debug, Clone)]
pub struct Input {
    /// widget poll，用于存储上下文中含有ID的Widget
    /// 这些Widget可能会在方法中被访问到，根据被访问的id可以生成对应的方法
    /// 例如方法中访问了:
    /// `let mut my_label = c_ref!(my_label);`, 那么就可以生成一个方法: `let mut my_label = self.glabel(id!(my_label));`
    pub widget_poll: WidgetPoll,
    /// 用于存储当前的方法是属于那个Widget的，例如:`<button id="btn" @clicked="btn_clicked()">`
    /// 说明当前的方法是属于button的，那么就可以通过button的id找到button的信息以及方法和回调方法相关的信息
    pub callback_widget: Option<CallbackWidget>,
    /// 实例, 实例能够知道在方法中那些部分需要被替换(提升)
    /// 例如: 实例名 = prop, fields = ["a", "b"]
    /// 那么方法中只要是使用了`prop.a`或者`prop.b`的地方都需要被替换为`self.a`或者`self.b`
    pub instance: Option<InstanceOutput>,
    pub fn_names: Vec<String>,
}

impl Input {
    pub fn new(
        widget_poll: WidgetPoll,
        callback_widget: Option<CallbackWidget>,
        instance: Option<InstanceOutput>,
        fn_names: Vec<String>,
    ) -> Self {
        Self {
            widget_poll,
            callback_widget,
            instance,
            fn_names,
        }
    }
    pub fn get_widget_name(&self, id: &str) -> Option<&crate::model::AbsWidget> {
        self.widget_poll.get(id)
    }
}
