use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::views::main_page::*;
    App = {{App}}{
        root: <Root>{
            main_window = <GWindow>{
                os_type: Linux,
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
                body = <AppMainPage>{}
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
        crate::views::main_page::live_design(cx);
        crate::views::overall::register(cx);
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