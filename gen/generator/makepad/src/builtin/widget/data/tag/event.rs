use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagEvent {
    Clicked,
    HoverIn,
    HoverOut,
    Closed,
    Focus,
    FocusLost
}

impl_widget_event!{
    TagEvent {
        TagEvent::Clicked => "GTagClickedParam" => "clicked",
        TagEvent::HoverIn => "GTagHoverParam" => "hover_in",
        TagEvent::HoverOut => "GTagHoverParam" => "hover_out",
        TagEvent::Closed => "GTagClosedParam" => "closed",
        TagEvent::Focus => "GTagFocusParam" => "focus",
        TagEvent::FocusLost => "GTagFocusLostParam" => "focus_lost"
    }
}