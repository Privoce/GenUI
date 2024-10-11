use gen_components::components::view::GView;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    BOLD_FONT = dep("crate://self/resources/OPPOSans-Bold.ttf");
    CBox = <GVLayout>{
        height: Fit,
        box_wrap = <GView>{
            height: Fit,
            padding: 16.0,
            width: Fill,
            flow: Down,
        }
        code = <GCollapse>{
            width: Fill,
            height: Fit,
            header: {
                padding: 12.0,
                width: Fill,
                <GIcon>{
                    height: 12.0,
                    width: 12.0,
                    icon_type: Code,
                }
                <GLabel>{
                    text: "Example Code"
                }
            },
            body: {
                height: Fit,
                width: Fill,
                scroll_bars: <GScrollBars>{},

            }
        }
    }

    LabelUsagePage = {{LabelUsagePage}}{
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
            text: "Basic Label Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Label is a basic component for displaying text, and usually does not enable animations or events.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GLabel>{
                    text: "Label without animation",
                }
                <GLabel>{
                    color: #F67D37,
                    text: "font color is #F67D37",
                }
            }
            code = {
                body: {
                    <GLabel>{
                        height: 180.0,
                        theme: Dark,
                        width: Fill,
                        text: r#"
                <GLabel>{
                    text: "Label without animation",
                }
                <GLabel>{
                    color: #F67D37,
                    text: "font color is #F67D37",
                }
                        "#;
                    }
                }
            }
        }
        
        
    }
}

#[derive(Live, Widget)]
pub struct LabelUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LabelUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LabelUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
