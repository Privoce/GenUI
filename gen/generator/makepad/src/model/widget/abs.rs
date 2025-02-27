use std::{collections::HashMap, hash::Hash};

use gen_analyzer::{value::Function, Props};
use gen_utils::common::{camel_to_snake, snake_to_camel};
use proc_macro2::TokenStream;
use syn::parse_str;

use crate::{
    builtin::{BuiltinWidget, BuiltinWidgetType},
    compiler::WidgetPoll,
};

use super::to_prop_map;

/// # 抽象Widget定义
/// 上下文Context中含有一个define_widget_poll，这个poll中存储了所有的define_widget（用户构建的Widget）
/// 需要存到Context中当产生新的Widget的时候
/// AbsWidget需要存储到HashMap中，对于Define来说只需要使用name作为specifier即可
#[derive(Debug, Clone)]
pub enum AbsWidget {
    Builtin(BuiltinWidgetType),
    Define {
        name: String,
        props: Option<HashMap<String, String>>,
        events: Option<HashMap<String, String>>,
    },
}

impl PartialEq for AbsWidget {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Builtin(l0), Self::Builtin(r0)) => l0 == r0,
            (
                Self::Define {
                    name: l_name,
                    props: _l_props,
                    events: _l_events,
                },
                Self::Define {
                    name: r_name,
                    props: _r_props,
                    events: _r_events,
                },
            ) => l_name == r_name,
            _ => false,
        }
    }
}

impl Eq for AbsWidget {}

impl Hash for AbsWidget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            AbsWidget::Builtin(builtin_widget_type) => builtin_widget_type.hash(state),
            AbsWidget::Define { name, .. } => name.hash(state),
        }
    }
}
impl AbsWidget {
    pub fn new(name: &str, props: Option<Props>) -> Self {
        if let Ok(builtin) = BuiltinWidget::is_built_in(name) {
            AbsWidget::Builtin(builtin)
        } else {
            AbsWidget::Define {
                name: name.to_string(),
                props: to_prop_map(props),
                events: None,
            }
        }
    }
    pub fn snake_name(&self) -> String {
        match self {
            AbsWidget::Builtin(builtin_widget_type) => builtin_widget_type.snake_name().to_string(),
            AbsWidget::Define { name, .. } => camel_to_snake(name),
        }
    }
    pub fn name(&self) -> String {
        match self {
            AbsWidget::Builtin(builtin_widget_type) => builtin_widget_type.name().to_string(),
            AbsWidget::Define { name, .. } => name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropWidget {
    /// id of the widget
    pub id: String,
    /// name of the widget
    pub widget: String,
    /// 绑定的prop的key
    pub prop: String,
    /// as prop (prop_widget_name, prop_ident)
    pub as_prop: Option<(String, String)>,
}

impl PropWidget {
    pub fn new(id: String, widget: String, prop: String) -> Self {
        Self { id, widget, prop, as_prop: None }
    }
    pub fn widget_name(&self) -> String {
        snake_name(&self.widget)
    }
}

fn snake_name(name: &str) -> String {
    BuiltinWidget::is_built_in(name).map_or_else(
        |_| camel_to_snake(name),
        |builtin| builtin.snake_name().to_string(),
    )
}

/// 处理带有Callbacks的Widget
#[derive(Debug, Clone)]
pub struct CallbackWidget {
    pub id: String,
    pub name: String,
    /// key: callback name, value: callback function
    /// example: `@hover_in="lb_hover_in()"`
    /// => key: lb_hover_in, value: {name: "hover_in", func: Function(xxx)}
    pub callbacks: HashMap<String, CallbackFn>,
}

impl CallbackWidget {
    pub fn id_tk(&self) -> TokenStream {
        parse_str::<TokenStream>(&self.id).unwrap()
    }
    pub fn widget_ref(&self) -> String {
        BuiltinWidget::is_built_in(&self.name).map_or_else(
            |_| format!("{}Ref", snake_to_camel(self.name.as_str())),
            |builtin| format!("{}Ref", builtin.name()),
        )
    }
    pub fn widget_ref_tk(&self) -> TokenStream {
        parse_str::<TokenStream>(&self.widget_ref()).unwrap()
    }
    pub fn name(&self) -> String {
        snake_name(&self.name)
    }
    /// make sure event is valid if call this function
    pub fn get_event_tk(&self, event: &str) -> TokenStream {
        parse_str::<TokenStream>(
            &self
                .get(event)
                .expect("CallbackWidget::get_event_tk: event is not valid")
                .event(),
        )
        .unwrap()
    }
    pub fn get(&self, key: &str) -> Option<&CallbackFn> {
        self.callbacks.get(key)
    }
    pub fn has(&self, key: &str) -> bool {
        self.callbacks.contains_key(key)
    }
    pub fn event_ty(&self, widget_poll: &WidgetPoll, target_fn: &str) -> Option<String> {
        let func = self.callbacks.get(target_fn).unwrap().name.to_string();
        if let Ok(builtin) = BuiltinWidget::is_built_in(&self.name) {
            builtin
                .event_ty_map()
                .map(|map| map.get(&func).cloned())
                .flatten()
        } else {
            // define widget
            if let AbsWidget::Define { events, .. } = widget_poll.get(self.id.as_str()).unwrap() {
                events
                    .as_ref()
                    .and_then(|events| events.get(&func).cloned())
            } else {
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallbackFn {
    pub func: Function,
    /// event name
    pub name: String,
}

impl CallbackFn {
    pub fn new(func: Function, name: String) -> Self {
        CallbackFn { func, name }
    }
    pub fn event(&self) -> String {
        self.name.to_string()
    }
}
