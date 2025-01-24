use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RadioEvent {
    Clicked,
    HoverIn,
    HoverOut,
}

impl_widget_event! {
    RadioEvent {
        RadioEvent::Clicked => "GRadioClickedParam" => "clicked",
        RadioEvent::HoverIn => "GRadioHoverParam" => "hover_in",
        RadioEvent::HoverOut => "GRadioHoverParam" => "hover_out"
    }
}
