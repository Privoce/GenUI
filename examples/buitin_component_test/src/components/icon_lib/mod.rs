pub mod base;
pub mod arrow;
pub mod code;
pub mod emoji;
pub mod fs;
pub mod person;
pub mod relation;
pub mod state;
pub mod time;
pub mod tool;
pub mod ui;

use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::components::icon_lib::base::*;
    import crate::components::icon_lib::arrow::*;
    import crate::components::icon_lib::code::*;
    import crate::components::icon_lib::emoji::*;
    import crate::components::icon_lib::fs::*;
    import crate::components::icon_lib::person::*;
    import crate::components::icon_lib::relation::*;
    import crate::components::icon_lib::state::*;
    import crate::components::icon_lib::time::*;
    import crate::components::icon_lib::tool::*;
    import crate::components::icon_lib::ui::*;

    GIconLibExample = <ScrollYView>{
        height: 600.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GIconLib",
        }
        <IconLibBase>{}
        <IconLibArrow>{}
        <IconLibCode>{}
        <IconLibEmoji>{}
        <IconLibFs>{}
        <IconLibPerson>{}
        <IconLibRelation>{}
        <IconLibState>{}
        <IconLibTime>{}
        <IconLibTool>{}
        <IconLibUi>{}
    }
}
