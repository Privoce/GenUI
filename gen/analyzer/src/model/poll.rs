use std::collections::HashMap;

use gen_utils::error::Error;

use crate::value::{Function, Value};

use super::{Parent, PropKey};

/// # Polls
/// 对每个模型中组件的绑定属性和事件属性进行池化，用于进行静态分析
#[derive(Debug, Clone, Default)]
pub struct Polls {
    pub binds: Option<Binds>,
    pub events: Option<Events>,
}

impl Polls {
    pub fn bind_mut(&mut self) -> &mut Binds {
        self.binds.get_or_insert_with(Default::default)
    }
    pub fn event_mut(&mut self) -> &mut Events {
        self.events.get_or_insert_with(Default::default)
    }
    pub fn insert_prop(&mut self, key: &str, value: PropComponent) -> () {
        self.bind_mut()
            .entry(key.to_string())
            .or_insert_with(Default::default)
            .push(value);
    }
    pub fn insert_event(&mut self, value: EventComponent) -> () {
        self.event_mut().push(value);
    }
}

/// key: bind ident , value: bind component
pub type Binds = HashMap<String, Vec<PropComponent>>;
/// 组件的事件池，用于存储组件的事件，一个事件可以被多个组件绑定
/// 目前来看通过组件为单位而不是使用类似Binds的方式来存储事件是更好的选择
pub type Events = Vec<EventComponent>;

#[derive(Debug, Clone)]
pub struct PropComponent {
    /// id of the widget
    pub id: String,
    /// name of the widget
    pub name: String,
    /// 绑定的prop的key
    pub prop: String,
    /// 如果组件被设置成as_prop, 这里会有值
    pub as_prop: Option<String>,
    /// 标识父组件的引用
    pub father_ref: Option<Parent>,
}

// ------------------------------------ Event -----------------------------------------------------------------
/// 处理带有Callbacks的Widget
#[derive(Debug, Clone)]
pub struct EventComponent {
    pub id: String,
    pub name: String,
    /// key: callback name, value: callback function
    /// example: `@hover_in="lb_hover_in()"`
    /// => key: lb_hover_in, value: {name: "hover_in", func: Function(xxx)}
    pub callbacks: HashMap<String, CallbackFn>,
}

impl EventComponent {
    pub fn convert_callbacks(
        callbacks: &HashMap<PropKey, Value>,
    ) -> Result<HashMap<String, CallbackFn>, Error> {
        let mut res = HashMap::new();
        for (key, value) in callbacks {
            let func = value.as_fn()?;

            res.insert(
                func.name.to_string(),
                CallbackFn::new(func, key.name.to_string()),
            );
        }

        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub struct CallbackFn {
    pub func: Function,
    /// event name
    pub event: String,
}

impl CallbackFn {
    pub fn new(func: Function, event: String) -> Self {
        CallbackFn { func, event }
    }
}
