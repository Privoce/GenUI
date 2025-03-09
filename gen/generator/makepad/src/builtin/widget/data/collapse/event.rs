use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CollapseEvent {
    Hover,
    Opened,
    Closed,
}

impl_widget_event! {
    CollapseEvent {
        CollapseEvent::Hover => "FingerHoverEvent" => "hover",
        CollapseEvent::Opened => "FingerUpEvent" => "opened",
        CollapseEvent::Closed => "FingerUpEvent" => "closed"
    }
}