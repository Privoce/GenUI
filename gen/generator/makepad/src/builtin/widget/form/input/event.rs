use crate::impl_widget_event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputEvent {
    Changed,
    KeyDownUnhandled,
    Escaped,
    KeyFocus,
    KeyFocusLost,
}

impl_widget_event! {
    InputEvent {
        InputEvent::Changed => "GInputChangedParam" => "changed",
        InputEvent::KeyDownUnhandled => "KeyEvent" => "keydown_unhandled",
        InputEvent::Escaped => "_" => "escaped",
        InputEvent::KeyFocus => "_" => "key_focus",
        InputEvent::KeyFocusLost => "_" => "key_focus_lost"
    }
}
