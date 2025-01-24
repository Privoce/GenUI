pub mod arrow;
pub mod base;
pub mod code;
pub mod emoji;
pub mod fs;
pub mod person;
pub mod relation;
pub mod state;
pub mod time;
pub mod tool;
pub mod ui;

pub fn register(cx: &mut Cx) {
    self::base::live_design(cx);
    self::arrow::live_design(cx);
    self::code::live_design(cx);
    self::emoji::live_design(cx);
    self::fs::live_design(cx);
    self::person::live_design(cx);
    self::relation::live_design(cx);
    self::state::live_design(cx);
    self::time::live_design(cx);
    self::tool::live_design(cx);
    self::ui::live_design(cx);
    self::live_design(cx);
}

use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::views::basic::icon::icon_lib::base::*;
    use crate::views::basic::icon::icon_lib::arrow::*;
    use crate::views::basic::icon::icon_lib::code::*;
    use crate::views::basic::icon::icon_lib::emoji::*;
    use crate::views::basic::icon::icon_lib::fs::*;
    use crate::views::basic::icon::icon_lib::person::*;
    use crate::views::basic::icon::icon_lib::relation::*;
    use crate::views::basic::icon::icon_lib::state::*;
    use crate::views::basic::icon::icon_lib::time::*;
    use crate::views::basic::icon::icon_lib::tool::*;
    use crate::views::basic::icon::icon_lib::ui::*;

    pub GIconLibExample = <ScrollYView>{
        height: Fit,
        width: Fill,
        spacing: 12.0,
        flow: Down,
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
