use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonEvent {
    HoverIn,
    HoverOut,
    Clicked,
    Focus,
    FocusLost,
}

impl_widget_event! {
    ButtonEvent {
        ButtonEvent::HoverIn => "GButtonHoverParam" => "hover_in",
        ButtonEvent::HoverOut => "GButtonHoverParam" => "hover_out",
        ButtonEvent::Clicked => "GButtonClickedParam" => "clicked",
        ButtonEvent::Focus => "GButtonFocusParam" => "focus",
        ButtonEvent::FocusLost => "GButtonFocusLostParam" => "focus_lost"
    }
}
