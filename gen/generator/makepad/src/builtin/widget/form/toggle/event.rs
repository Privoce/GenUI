use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToggleEvent {
    Clicked,
    HoverIn,
    HoverOut,
}

impl_widget_event!{
    ToggleEvent {
        ToggleEvent::Clicked => "GToggleClickedParam" => "clicked",
        ToggleEvent::HoverIn => "GToggleHoverParam" => "hover_in",
        ToggleEvent::HoverOut => "GToggleHoverParam" => "hover_out"
    }
}