use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub WindowPage = {{WindowPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        padding: 12.0,
        scroll_bars: <GScrollBars>{},
        clip_x: true,
        clip_y: true,
        <GHLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "Window",
            }
        }
        <GLabel>{
            width: Fill,
            text: "Window can help you create a new window, it use in Root and you can call to open a new window.",
        }
        <GLabel>{
            width: Fill,
            text: "You can use os_type to set window act like MacOs, Windows, Linux or Others.",
        }
        <GLabel>{
            width: Fill,
            text: "You can use show_title, show_icon to control the title and icon of the window.",
        }
        <GView>{
            height: Fit,
            padding: 12.0,
            width: Fill,
            <GLabel>{
                width: Fill,
                text: r#"
        App = {{App}}{
            root: <Root>{
                main_window = <GWindow>{
                    os_type: Mac,
                    window_bar = {
                        window_title = {
                            title = {
                                text: "GenUI Builtin Components",
                            }
                            icon = {
                                src: Live(dep("crate://self/resources/google.png")),
                            }
                        }
                    }
                    width: Fill,
                    height: Fill,
                    window: {inner_size: vec2(920, 800)},
                    body = <AppMainPage>{}
                }
            }
        }
                "#,
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct WindowPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for WindowPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for WindowPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
