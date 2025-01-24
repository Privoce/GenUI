use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RadioGroupEvent {
    Changed,
}

impl_widget_event! {
    RadioGroupEvent {
        RadioGroupEvent::Changed => "GRadioGroupEventParam" => "changed"
    }
}
