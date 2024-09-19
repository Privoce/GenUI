use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::components::cards::*;
    import crate::components::label::*;
    import crate::components::buttons::*;
    import crate::components::hlayout::*;
    import crate::components::vlayout::*;
    import crate::components::links::*;
    import crate::components::icons::*;
    import crate::components::radios::*;
    import crate::components::checkboxs::*;
    import crate::components::images::*;
    import crate::components::inputs::*;
    import crate::components::dividers::*;
    import crate::components::shaders::*;
    import crate::components::select::*;
    import crate::components::popups::*;
    import crate::components::toggles::*;
    import crate::components::progresss::*;
    import crate::components::loadings::*;
    import crate::components::states::*;
    import crate::components::tags::*;
    import crate::components::bread_crumbs::*;
    import crate::components::tabs::*;
    import crate::components::icon_lib::*;
    import crate::components::collapses::*;
    import crate::components::tables::*;
    import crate::components::notifications::*;
    import crate::components::tool_btns::*;
    // FontA = { font: { path: dep("crate://self/resources/AlimamaFangYuanTiVF-Thin.ttf") } };

    App = {{App}}{
        root: <Root>{
            
            main_window = <GWindow>{
                os_type: Windows,
                window_bar = {
                    window_title = {
                        title = {
                            text: "GenUI Builtin Components",
                        }
                        icon = {
                            src: dep("crate://self/resources/google.png"),
                        }
                    }
                }
                width: Fill,
                height: Fill,
                window: {inner_size: vec2(880, 800)},
                body = <ScrollYView>{
                    height: All,
                    width: All,
                    flow: Right,
                    // spacing: 10.0,
                    // padding: 10.0,
                    
                    // <GToolButtonExample>{}
                    // <Note>{}
                    // <GTabsExample>{}
                    // <GTableExample>{}
                    <GPopupExample>{}
                    // <GInputExample>{}
                    // <GShaderExample>{}
                    // <GCollapseExample>{}
                    // <GProgressExample>{}
                    // <GDividerExample>{}
                    // <GCard>{
                    //     width: 200.0,
                    //     height: 200.0,
                    //     <GSplitter>{
                    //     }
                    // }
                    // <GLoadingExample>{}
                    // <GRadioExample>{}
                    // <GCheckBoxExample>{}
                    // <GToggleExample>{}
                    // <GImageExample>{}
                    // <GStatesExample>{}
                    // <GCardExample>{}
                    // <GHLayoutExample>{}
                    // <GVLayoutExample>{}
                    // <GTagExample>{}
                    // <GCard>{
                    //     theme: Dark,
                    //     height: 100.0,
                    //     width: 300.0,
                    //     flow: Down,
                    //     align: {x: 0.5, y: 0.5},
                    //     <GUpload>{
                    //         height: 70.0,
                    //         filters: ["png"]
                    //     }
                    //     <GLabel>{
                    //         height: Fit,
                    //         text: "Upload!",
                    //     }
                    // }
                    // <GBreadCrumbExample>{}
                    // <GLinkExample>{}
                    // <GSvgExample>{}
                    // <GIconLibExample>{}
                    // <GButtonExample>{}
                    // <GLabelExample>{}
                    //
                    // --------------------------------
                    // pop = <GPopup>{
                    //     height: 100.0,
                    //     width: 200.0,
                    // }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    root: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx);
        crate::components::cards::live_design(cx);
        crate::components::label::live_design(cx);
        crate::components::buttons::live_design(cx);
        crate::components::hlayout::live_design(cx);
        crate::components::vlayout::live_design(cx);
        crate::components::links::live_design(cx);
        crate::components::icons::live_design(cx);
        crate::components::radios::live_design(cx);
        crate::components::checkboxs::live_design(cx);
        crate::components::images::live_design(cx);
        crate::components::inputs::live_design(cx);
        crate::components::dividers::live_design(cx);
        crate::components::shaders::live_design(cx);
        crate::components::select::live_design(cx);
        crate::components::popups::live_design(cx);
        crate::components::toggles::live_design(cx);
        crate::components::progresss::live_design(cx);
        crate::components::loadings::live_design(cx);
        crate::components::states::live_design(cx);
        crate::components::tags::live_design(cx);
        crate::components::bread_crumbs::live_design(cx);
        crate::components::tabs::live_design(cx);
        crate::components::icon_lib::live_design(cx);
        crate::components::icon_lib::base::live_design(cx);
        crate::components::icon_lib::arrow::live_design(cx);
        crate::components::icon_lib::code::live_design(cx);
        crate::components::icon_lib::emoji::live_design(cx);
        crate::components::icon_lib::fs::live_design(cx);
        crate::components::icon_lib::person::live_design(cx);
        crate::components::icon_lib::relation::live_design(cx);
        crate::components::icon_lib::state::live_design(cx);
        crate::components::icon_lib::time::live_design(cx);
        crate::components::icon_lib::tool::live_design(cx);
        crate::components::icon_lib::ui::live_design(cx);
        crate::components::collapses::live_design(cx);
        crate::components::tables::live_design(cx);
        crate::components::notifications::live_design(cx);
        crate::components::tool_btns::live_design(cx);
        // crate::gen_components::live_design!(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        log!("App started!");
       
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
