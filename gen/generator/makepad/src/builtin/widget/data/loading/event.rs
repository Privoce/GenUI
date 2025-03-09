use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingEvent {
    Opened,
    Closed,
}

impl_widget_event! {
    LoadingEvent {
        LoadingEvent::Opened => "_" => "opened",
        LoadingEvent::Closed => "_" => "closed"
    }
}
