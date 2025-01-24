use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    
    pub SvgUsagePage = {{SvgUsagePage}}{
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
            text: "Basic Svg Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Svg can show .svg files and color them.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GSvg>{
                    height: 32.0,
                    width: 32.0,
                    src: dep("crate://self/resources/all.svg"),
                }
                <GSvg>{
                    height: 32.0,
                    width: 32.0,
                    src: dep("crate://self/resources/upload.svg"),
                    color: #FF0000,
                }
                <GSvg>{
                    height: 32.0,
                    width: 32.0,
                    src: dep("crate://self/resources/config.svg"),
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 200.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            height: Fit,
                            theme: Dark,
                            width: Fill,
                            text: r#"
                    <GSvg>{
                        height: 32.0,
                        width: 32.0,
                        src: dep("crate://self/resources/all.svg"),
                    }
                    <GSvg>{
                        height: 32.0,
                        width: 32.0,
                        src: dep("crate://self/resources/upload.svg"),
                        color: #FF0000,
                    }
                    <GSvg>{
                        height: 32.0,
                        width: 32.0,
                        src: dep("crate://self/resources/config.svg"),
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
pub struct SvgUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for SvgUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SvgUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
