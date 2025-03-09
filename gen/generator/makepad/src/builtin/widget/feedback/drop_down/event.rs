use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DropDownEvent {
    Changed
}

impl_widget_event!{
    DropDownEvent {
        DropDownEvent::Changed => "GDropDownChangedParam" => "changed"
    }
}