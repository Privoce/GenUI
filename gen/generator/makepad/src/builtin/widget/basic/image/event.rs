use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageEvent {
    HoverIn,
    HoverOut,
    Clicked,
}

impl_widget_event! {
    ImageEvent {
        ImageEvent::HoverIn => "GImageHoverParam" => "hover_in",
        ImageEvent::HoverOut => "GImageHoverParam" => "hover_out",
        ImageEvent::Clicked => "GImageClickedParam" => "clicked"
    }
}