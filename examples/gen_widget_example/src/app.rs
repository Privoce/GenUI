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
    import crate::components::badges::*;
    import crate::components::bread_crumbs::*;
    import crate::components::tabs::*;

    App = {{App}}{
        root: <Root>{
            main_window = <Window>{
                show_bg: true,
                width: Fill,
                height: Fill,
                draw_bg: {color: #b},
                window: {inner_size: vec2(600, 800)},
                body = <ScrollYView>{
                    height: All,
                    width: All,
                    flow: Down,
                    spacing: 10.0,
                    padding: 10.0,
                    <GVLayout>{
                        height: 300.0,
                        spacing: 6.0,
                        width:Fill,
                        <GHLayout>{
                            height: Fit,
                            spacing: 6.0,
                            <GToolButton>{
                                button_type: Default,
                            }
                            <GToolButton>{
                                button_type: Min,
                            }
                            <GToolButton>{
                                button_type: Max,
                            }
                            <GToolButton>{
                                button_type: FullScreen,
                            }
                            <GToolButton>{
                                button_type: Left,
                            }
                            <GToolButton>{
                                button_type: Right,
                            }
                            <GToolButton>{
                                button_type: More,
                            }
                            <GToolButton>{
                                button_type: Close,
                            }
                            <GToolButton>{
                                button_type: Up,
                            }
                            <GToolButton>{
                                button_type: Down,
                            }
                        }
                        <GHLayout>{
                            spacing: 6.0,
                            height: Fit,
                            <GToolButton>{
                                button_type: Switch,
                            }
                            <GToolButton>{
                                button_type: Exit,
                            }
                            <GToolButton>{
                                button_type: Expand,
                            }
                            <GToolButton>{
                                button_type: ExpandTop,
                            }
                            <GToolButton>{
                                button_type: ExpandLeft,
                            }
                            <GToolButton>{
                                button_type: ExpandRight,
                            }
                            <GToolButton>{
                                button_type: ExpandBottom,
                            }
                            <GToolButton>{
                                button_type: Add,
                            }
                            <GToolButton>{
                                button_type: Delete,
                            }
                            <GToolButton>{
                                button_type: Correct,
                            }
                        }
                    }
                    <GCollapse>{
                        height: 100.0,
                        width: 300.0
                    }
                    <GUpload>{

                    }
                    <GCard>{
                        width: 200.0,
                        height: 200.0,
                        <GSplitter>{
                        
                        }
                    }
                    <GTabsExample>{}
                    <GBreadCrumbExample>{}
                    <GBadgeExample>{}
                    <GStatesExample>{}
                    // loading pass, test ok
                    // <GLoadingExample>{}
                    <GProgressExample>{}
                    <GToggleExample>{}

                    <GPopupExample>{}
                    // <GShaderExample>{}
                    // <GSelectExample>{}
                    <GLabelExample>{}

                    <GButtonExample>{}
                    <GCardExample>{}
                    <GHLayoutExample>{}
                    <GVLayoutExample>{}
                    <GLinkExample>{}
                    <GRadioExample>{}
                    <GCheckBoxExample>{}
                    <GIconExample>{}
                    <GImageExample>{}
                    <GDividerExample>{}
                    <GInputExample>{}

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
        crate::components::badges::live_design(cx);
        crate::components::bread_crumbs::live_design(cx);
        crate::components::tabs::live_design(cx);
        // crate::gen_components::live_design!(cx);
    }
}

impl MatchEvent for App {}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
