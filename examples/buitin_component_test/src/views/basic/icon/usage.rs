use gen_components::components::view::GView;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::styles::*;
    
    ImageUsagePage = {{ImageUsagePage}}{
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
            text: "Basic Icon(Icon Lib) Usage",
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GLabel>{
                width: Fill,
                text: "The current icon library is only an experimental function. It provided a set of commonly used icon collections",
            }
            <GLabel>{
                font_size: 10.0,
                font_family: (BOLD_FONT),
                text: "Advantages: ",
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                <GLabel>{
                    text: "1. No copyright, completely free for commercial use",
                }
                <GLabel>{
                    text: "2. Small size",
                }
                <GLabel>{
                    text: "3. No need to download from the internet for introduction",
                }
                <GLabel>{
                    text: "4. Directly use Shader for drawing",
                }
            }
            <GLabel>{
                font_size: 10.0,
                font_family: (BOLD_FONT),
                text: "Subsequent update plan: ",
            }
            <GVLayout>{
                height: Fit,
                spacing: 8.0,
                <GLabel>{
                    text: "1. Add icons to approximately 150 commonly used icons",
                }
                <GLabel>{
                    text: "2. Optimize icon experience",
                }
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 24.0,
                flow: Right,
                <GIcon>{
                    height: 32.0,
                    width: 32.0,
                    icon_type: Max,
                }
                <GIcon>{
                    height: 32.0,
                    width: 32.0,
                    icon_type: FullScreenExpand,
                }
                <GIcon>{
                    height: 32.0,
                    width: 32.0,
                    icon_type: Picture,
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
