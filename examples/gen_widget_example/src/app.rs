use gen_components::components::button::GButtonWidgetRefExt;
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

    App = {{App}}{
        root: <Root>{
            main_window = <Window>{
                show_bg: true,
                width: Fill,
                height: Fill,
                draw_bg: {color: #888},
                window: {inner_size: vec2(600, 800)},
                body = <ScrollYView>{
                    height: All,
                    width: All,
                    flow: Down,
                    spacing: 10.0,
                    padding: 10.0,
                    <View>{
                        height: 200.0,
                        width: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 0.5},
                        <GLoading>{
                            theme: Error,
                        }
                        <GLabel>{
                            text: "Loading ...",
                        }
                    }
                    <View>{
                        height: 100.0,
                        width: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 0.5},
                        <GLoading>{
                            loading_type: CircleDot
                        }
                        <GLabel>{
                            text: "Loading ...",
                        }
                    }
                    <View>{
                        height: 100.0,
                        width: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 0.5},
                        <GLoading>{
                            loading_type: DotLine
                        }
                        <GLabel>{
                            text: "Loading ...",
                        }
                    }

                    <GProgressExample>{}
                    <GToggleExample>{}
                    <GDropDown>{
                        
                        height: Fit,
                        width: Fit,
                        trigger = <GButton>{text:"open"},
                        popup :<GPopup> {
                            height: 150.0,
                            width: 200.0,
                            container: <GPopupContainer> {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
                                <GButton>{
                                    theme: Dark,
                                    text: "Options"
                                }
                                <View>{
                                    show_bg: true,
                                    draw_bg: {color: #f00},
                                    height: 40.0,
                                    width: 40.0,
                                }
                            }
                        }
                    }
                    
                    // <GShaderExample>{}
                    // <GPopupExample>{}
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
        // crate::gen_components::live_design!(cx);
    }
}

impl MatchEvent for App {
    
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
