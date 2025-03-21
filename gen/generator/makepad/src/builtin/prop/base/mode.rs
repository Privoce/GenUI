use crate::try_from_enum_one_leaf;

#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub enum TriggerMode {
    #[default]
    Click,
    Hover,
    Press,
}

#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub enum CloseMode {
    /// Virtual Close, means you can not close if you click the outer, you must call close by code
    Virtual,
    #[default]
    /// Only Outer Can Close Popup, always use when you have no close button in the popup
    Out,
}
#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub enum PopupMode {
    #[default]
    Popup,
    ToolTip,
    Dialog,
    Drawer,
}

#[derive(Clone, PartialEq, Default, Copy, Debug)]
pub enum NavMode {
    #[default]
    /// History mode
    /// - use history to navigate (nav_to or nav_back) till stack is empty
    /// ```
    /// nav_to: A -> B -> C -> D
    /// nav_back: D -> C -> B -> A
    /// ```
    History,
    /// Stack mode
    /// ```
    /// nav_to: A -> B -> C -> D
    /// nav_back: D -> C -> D -> C
    /// ```
    Switch,
}

try_from_enum_one_leaf! {
    TriggerMode, "TriggerMode",
    TriggerMode::Click = "Click",
    TriggerMode::Hover = "Hover",
    TriggerMode::Press = "Press"
}

try_from_enum_one_leaf! {
    CloseMode, "CloseMode",
    CloseMode::Virtual = "Virtual",
    CloseMode::Out = "Out"
}

try_from_enum_one_leaf! {
    PopupMode, "PopupMode",
    PopupMode::Popup = "Popup",
    PopupMode::ToolTip = "ToolTip",
    PopupMode::Dialog = "Dialog",
    PopupMode::Drawer = "Drawer"
}

try_from_enum_one_leaf! {
    NavMode, "NavMode",
    NavMode::History = "History",
    NavMode::Switch = "Switch"
}