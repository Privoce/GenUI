
use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckboxGroupEvent {
    Changed,
}


impl_widget_event! {
    CheckboxGroupEvent {
        CheckboxGroupEvent::Changed => "GCheckboxGroupEventParam" => "changed"
    }
}