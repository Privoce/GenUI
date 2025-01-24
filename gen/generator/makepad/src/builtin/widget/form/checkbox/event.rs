use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckboxEvent {
    Clicked,
    HoverIn,
    HoverOut,
}

impl_widget_event! {
    CheckboxEvent {
        CheckboxEvent::Clicked => "GCheckboxClickedParam" => "clicked",
        CheckboxEvent::HoverIn => "GCheckboxHoverParam" => "hover_in",
        CheckboxEvent::HoverOut => "GCheckboxHoverParam" => "hover_out"
    }
}
