use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub CheckboxUsagePage = {{CheckboxUsagePage}}{
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
            text: "Basic CheckBox Usage",
        }
        <GLabel>{
            width: Fill,
            text: "CheckBox is a component that allows users to select one option from a set of options. It is a round control that can be selected or deselected.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GCheckbox>{
                    theme: Success,
                    checkbox_type: Round,
                }
                <GCheckbox>{
                    theme: Info,
                    checkbox_type: Tick,
                    selected: true
                }
                <GCheckbox>{
                    theme: Error,
                    checkbox_type: Cross,
                    text: "Error Cross CheckBox"
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
                <GCheckbox>{
                    theme: Success,
                    checkbox_type: Round,
                }
                <GCheckbox>{
                    theme: Info,
                    checkbox_type: Tick,
                    selected: true
                }
                <GCheckbox>{
                    theme: Error,
                    checkbox_type: Cross,
                    text: "Error Cross CheckBox"
                }
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            width: Fill,
            text: "CheckBoxGroup can help you manage multiple CheckBoxes. It can has multiple selected CheckBoxes.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GCheckboxGroup>{
                    <GCheckbox>{
                        theme: Success,
                        checkbox_type: Round,
                    }
                    <GCheckbox>{
                        theme: Info,
                        checkbox_type: Tick,
                        selected: true
                    }
                    <GCheckbox>{
                        theme: Error,
                        checkbox_type: Cross,
                        text: "Error Cross CheckBox"
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
                <GCheckbox>{
                    theme: Success,
                    checkbox_type: Round,
                }
                <GCheckbox>{
                    theme: Info,
                    checkbox_type: Tick,
                    selected: true
                }
                <GCheckbox>{
                    theme: Error,
                    checkbox_type: Cross,
                    text: "Error Cross CheckBox"
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
pub struct CheckboxUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for CheckboxUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for CheckboxUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
