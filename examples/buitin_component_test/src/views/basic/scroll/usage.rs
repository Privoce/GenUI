use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ScrollUsagePage = {{ScrollUsagePage}}{
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
            text: "Basic ScrollBar Usage",
        }
        <GLabel>{
            width: Fill,
            text: "ScrollBar can be used to scroll the content of the container. It is exist in the View component.What you need to do is to set the scroll_bars property to GScrollBars{} if you want to use it.",
        }
        <CBox>{
            box_wrap = {
                spacing: 12.0,
                flow: Right,
                <GHLayout>{
                    height: 200.0,
                    width: Fill,
                    spacing: 16.0,
                    scroll_bars: <GScrollBars>{},
                    <GView>{
                        theme: Error,
                        height: 300.0,
                        width: 300.0,
                    }
                    <GView>{
                        theme: Success,
                        height: 100.0,
                        width: 600.0,
                    }
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
                <GHLayout>{
                    height: 200.0,
                    width: Fill,
                    spacing: 16.0,
                    scroll_bars: <GScrollBars>{},
                    <GView>{
                        theme: Error,
                        height: 300.0,
                        width: 300.0,
                    }
                    <GView>{
                        theme: Success,
                        height: 100.0,
                        width: 600.0,
                    }
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
pub struct ScrollUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ScrollUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ScrollUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
