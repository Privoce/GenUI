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
        InputEvent::Escaped => "KeyEvent" => "escaped",
        InputEvent::KeyFocus => "KeyFocusEvent" => "key_focus",
        InputEvent::KeyFocusLost => "KeyFocusEvent" => "key_focus_lost"
    }
}
