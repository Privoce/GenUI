mod prop;

pub use prop::Props as PopupProps;

use crate::{
    builtin::{prop::{FromGenProps, Prop}, widget::View},
    to_live_design, to_live_design_inherits, try_from_props,
    two_way_binding::TwoWayBindImpl,
};
#[derive(Debug, Clone)]
pub struct Popup {
    pub prop: Option<Prop<PopupProps>>,
}

#[derive(Debug, Clone)]
pub struct Drawer(pub Popup);

#[derive(Debug, Clone)]
pub struct Dialog(pub Popup);

#[derive(Debug, Clone)]
pub struct ToolTip(pub Popup);


#[derive(Debug, Clone)]
pub struct PopupContainer(pub View);

to_live_design!(Popup: "GPopup");
to_live_design_inherits!(Drawer: "GDrawer");
to_live_design_inherits!(Dialog: "GDialog");
to_live_design_inherits!(ToolTip: "GToolTip");
to_live_design_inherits!(PopupContainer: "GPopupContainer");

try_from_props! {
    Popup {
       |props|  Ok(Self { prop: Prop::<PopupProps>::from_prop(props)? })
    }
}

try_from_props! {
    Drawer {
       |props|  Ok(Self(Popup::try_from(props)?))
    }
}

try_from_props! {
    Dialog {
       |props|  Ok(Self(Popup::try_from(props)?))
    }
}

try_from_props! {
    ToolTip {
       |props|  Ok(Self(Popup::try_from(props)?))
    }
}

try_from_props! {
    PopupContainer {
       |props|  Ok(Self(View::try_from(props)?))
    }
}

impl TwoWayBindImpl for Popup {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}

impl TwoWayBindImpl for Drawer {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}

impl TwoWayBindImpl for Dialog {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}

impl TwoWayBindImpl for ToolTip {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}

impl TwoWayBindImpl for PopupContainer {
    fn twb_event(_prop: &str) -> Option<String> {
        None
    }
}