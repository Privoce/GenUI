use std::collections::HashMap;

use crate::builtin::event::WidgetEvent;

pub trait WidgetImpl {
    type EventType: WidgetEvent;
    fn event_ty_map() -> Option<HashMap<String, String>> {
        Self::EventType::to_map()
    }
}