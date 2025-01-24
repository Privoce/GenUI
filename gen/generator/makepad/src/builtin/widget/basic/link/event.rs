use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkEvent {
    HoverIn,
    HoverOut,
    Clicked,
    Focus,
    FocusLost,
}

impl_widget_event! {
    LinkEvent {
        LinkEvent::HoverIn => "GLinkHoverParam" => "hover_in",
        LinkEvent::HoverOut => "GLinkHoverParam" => "hover_out",
        LinkEvent::Clicked => "GLinkClickedParam" => "clicked",
        LinkEvent::Focus => "GLinkFocusParam" => "focus",
        LinkEvent::FocusLost => "GLinkFocusLostParam" => "focus_lost"
    }
}
