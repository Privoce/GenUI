use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Event<T>(pub Vec<T>)
where
    T: WidgetEvent;

impl<T> Event<T>
where
    T: WidgetEvent,
{
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
}

impl<T> Default for Event<T>
where
    T: WidgetEvent,
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

pub trait WidgetEvent: Debug + ToString {
    fn ty(&self) -> String;
    fn to_map() -> Option<HashMap<String, String>>;
}

#[macro_export]
macro_rules! impl_widget_event {
    ($T: ty {
        $(
            $K: path => $V: expr => $S: expr
        ),*
    }) => {
        impl crate::builtin::event::WidgetEvent for $T {
            fn ty(&self) -> String {
                match self {
                    $(
                        $K => $V,
                    )*
                }
                .to_string()
            }
        
            fn to_map() -> Option<std::collections::HashMap<String, String>> {
                Some(
                    vec![
                        $(
                            $K,
                        )*
                    ]
                    .into_iter()
                    .map(|event| (event.to_string(), event.ty()))
                    .collect(),
                )
            }
        }
        
        impl ToString for $T {
            fn to_string(&self) -> String {
                match self {
                    $(
                        $K => $S,
                    )*
                }.to_string()
            }
        }
    };
}