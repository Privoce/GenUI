use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub RadioUsagePage = {{RadioUsagePage}}{
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
            text: "Basic Radio Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Radio is a component that allows users to select one option from a set of options. It is a round control that can be selected or deselected.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GRadio>{
                    theme: Success,
                    radio_type: Round,
                }
                <GRadio>{
                    theme: Info,
                    radio_type: Tick,
                }
                <GRadio>{
                    theme: Error,
                    radio_type: Cross,
                }
                <GRadio>{
                    theme: Success,
                    radio_type: Round,
                    selected: true
                }
                <GRadio>{
                    theme: Info,
                    radio_type: Tick,
                    selected: true
                }
                <GRadio>{
                    theme: Error,
                    radio_type: Cross,
                    text: "Error Cross Radio"
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
                    <GRadio>{
                        theme: Success,
                        radio_type: Round,
                    }
                    <GRadio>{
                        theme: Info,
                        radio_type: Tick,
                    }
                    <GRadio>{
                        theme: Error,
                        radio_type: Cross,
                    }
                    <GRadio>{
                        theme: Success,
                        radio_type: Round,
                        selected: true
                    }
                    <GRadio>{
                        theme: Info,
                        radio_type: Tick,
                        selected: true
                    }
                    <GRadio>{
                        theme: Error,
                        radio_type: Cross,
                        text: "Error Cross Radio"
                    }
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            text: "Use RadioGroup can help you switch and group the radios."
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GRadioGroup>{
                    spacing: 16.0,
                    <GRadio>{
                        theme: Success,
                        radio_type: Round,
                    }
                    <GRadio>{
                        theme: Info,
                        radio_type: Tick,
                        text: "Info Tick"
                    }
                    <GRadio>{
                        theme: Error,
                        radio_type: Cross,
                    }
                }

            }
            code = {
                body: {
                    <GLabel>{
                        theme: Dark,
                        width: Fill,
                        text: r#"
                <GRadioGroup>{
                    spacing: 16.0,
                    <GRadio>{
                        theme: Success,
                        radio_type: Round,
                    }
                    <GRadio>{
                        theme: Info,
                        radio_type: Tick,
                    }
                    <GRadio>{
                        theme: Error,
                        radio_type: Cross,
                    }
                }
                        "#;
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct RadioUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for RadioUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for RadioUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
