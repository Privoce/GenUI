use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelEvent {
    HoverIn,
    HoverOut,
    Focus,
    FocusLost,
}

impl_widget_event! {
    LabelEvent {
        LabelEvent::HoverIn => "GLabelHoverParam" => "hover_in",
        LabelEvent::HoverOut => "GLabelHoverParam" => "hover_out",
        LabelEvent::Focus => "GLabelFocusParam" => "focus",
        LabelEvent::FocusLost => "GLabelFocusLostParam" => "focus_lost"
    }
}
