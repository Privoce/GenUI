use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub LinkUsagePage = {{LinkUsagePage}}{
        height: Fit,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        clip_x: true,
        clip_y: true,
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "Basic Link Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Link is a component that can be used to jump to a specified URL. It is a simple and effective way to navigate.",
        }
        <CBox>{
            box_wrap = {
                spacing: 12.0,
                flow: Right,
                <GLink>{
                    text: "To Github",
                    href: "http://github.com",
                }
                <GLink>{
                    theme: Error,
                    text: "GenUI Components Lib",
                    href: "https://github.com/Privoce/GenUI/gen/components",
                }
                <GLink>{
                    theme: Error,
                    background_visible: true,
                    padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                    border_radius: 4.0,
                    underline_visible: false,
                    text: "Act as Button",
                    href: "https://github.com/Privoce/GenUI/gen/components",
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 300.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                    <GLink>{
                        text: "To Github",
                        href: "http://github.com",
                    }
                    <GLink>{
                        theme: Error,
                        text: "GenUI Components Lib",
                        href: "https://github.com/Privoce/GenUI/gen/components",
                    }
                    <GLink>{
                        theme: Error,
                        background_visible: true,
                        padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                        border_radius: 4.0,
                        underline_visible: false,
                        text: "Act as Button",
                        href: "https://github.com/Privoce/GenUI/gen/components",
                    }
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct LinkUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LinkUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LinkUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
