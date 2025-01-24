use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    
    pub ImageUsagePage = {{ImageUsagePage}}{
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
        // <GLabel>{
        //     width: Fit,
        //     text: "Image is a component that displays an image, now supports .jpg and .png.",
        // }
        <GHLayout>{
            height: Fit,
            align: {y: 0.5}
            <GLabel>{
                width: Fit,
                text: "Image is a component that displays an image, now supports .jpg and .png.",
            }
            <GIcon>{
                icon_type: Add,
            }
            <GLabel>{
                width: Fit,
                text: "Image is a component that displays an image, now supports .jpg and .png.",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 24.0,
                flow: Right,
                <GImage>{
                    height: 32.0,
                    width: 32.0,
                    src: dep("crate://self/resources/google.png"),
                }
                <GImage>{
                    height: 56.0,
                    width: 64.0,
                    src: dep("crate://self/resources/rust.png"),
                }
                <GImage>{
                    height: 56.0,
                    width: 96.0,
                    src: dep("crate://self/resources/rust2.jpg"),
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
                <GImage>{
                    height: 32.0,
                    width: 32.0,
                    src: dep("crate://self/resources/google.png"),
                }
                <GImage>{
                    height: 56.0,
                    width: 64.0,
                    src: dep("crate://self/resources/rust.png"),
                }
                <GImage>{
                    height: 56.0,
                    width: 64.0,
                    src: dep("crate://self/resources/rust2.jpg"),
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
pub struct ImageUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ImageUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ImageUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
