use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ProgressUsagePage = {{ProgressUsagePage}}{
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
            text: "Basic Progress Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Progress is a component that can show the progress of a task. It can be used in a variety of scenarios.",
        }
        <GLabel>{
            width: Fill,
            text: "You can set the value of the progress bar to show the progress of the task.",
        }
        <GLabel>{
            width: Fill,
            text: "You can change the direction of the progress bar by setting the progress_type property.",
        }
        <CBox>{
            box_wrap = {
                spacing: 24.0,
                flow: Right,
                <GVLayout>{
                    width: Fit,
                    spacing: 16.0,
                    <GProgress>{
                        value: 0.5,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Success,
                        height: 20.0,
                        border_radius: 2.0,
                        value: 0.36,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Info,
                        value: 0.0,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Warning,
                        value: 1.0,
                        read_only: false,
                    }
                }
                <GHLayout>{
                    height: Fit,
                    width: Fill,
                    spacing: 20.0,
                    <GProgress>{
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                        value: 0.8,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Success,
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                        border_radius: 2.0,
                        value: 0.36,
                        read_only: false,
                        background_visible: false,
                    }
                    <GProgress>{
                        theme: Info,
                        value: 0.0,
                        read_only: false,
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                    }
                    <GProgress>{
                        theme: Warning,
                        value: 1.0,
                        read_only: false,
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 240.0,
                        scroll_bars: <GScrollBars>{}
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GVLayout>{
                    width: Fit,
                    spacing: 16.0,
                    <GProgress>{
                        value: 0.5,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Success,
                        height: 20.0,
                        border_radius: 2.0,
                        value: 0.36,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Info,
                        value: 0.0,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Warning,
                        value: 1.0,
                        read_only: false,
                    }
                }
                <GHLayout>{
                    height: Fit,
                    width: Fill,
                    spacing: 20.0,
                    <GProgress>{
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                        value: 0.8,
                        read_only: false,
                    }
                    <GProgress>{
                        theme: Success,
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                        border_radius: 2.0,
                        value: 0.36,
                        read_only: false,
                        background_visible: false,
                    }
                    <GProgress>{
                        theme: Info,
                        value: 0.0,
                        read_only: false,
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                    }
                    <GProgress>{
                        theme: Warning,
                        value: 1.0,
                        read_only: false,
                        progress_type: Vertical,
                        height: 200.0,
                        width: 16.0,
                    }
                }
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            width: Fill,
            text: "Use read_only to make the progress bar read-only.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GProgress>{
                   theme: Error,
                   value: 0.8
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 240.0,
                        scroll_bars: <GScrollBars>{}
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
              
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct ProgressUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ProgressUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ProgressUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
