use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SvgEvent {
    Clicked,
    HoverIn,
    HoverOut,
    Focus,
    FocusLost,
}

impl_widget_event! {
    SvgEvent {
        SvgEvent::Clicked => "GSvgClickedParam" => "clicked",
        SvgEvent::HoverIn => "GSvgHoverParam" => "hover_in",
        SvgEvent::HoverOut => "GSvgHoverParam" => "hover_out",
        SvgEvent::Focus => "GSvgFocusParam" => "focus",
        SvgEvent::FocusLost => "GSvgFocusLostParam" => "focus_lost"
    }
}
