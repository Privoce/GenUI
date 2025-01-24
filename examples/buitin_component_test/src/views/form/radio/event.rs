use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub RadioEnPage = {{RadioEnPage}}{
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
            text: "Event Usage",
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GLabel>{
                width: Fill,
                text: "Radio has 3 events: 1. Clicked 2. HoverIn 3. HoverOut",
            }
            <GLabel>{
                text: "RadioGroup just has 1 event:\n1. Changed\nChanged event can get the selected index and value.",
            }
        }

        an_box = <CBox>{
            box_wrap = {
                spacing: 8.0,

                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,

                    radio_group = <GRadioGroup>{
                        spacing: 16.0,
                        <GRadio>{
                            theme: Success,
                            radio_type: Round,
                            value: "Success_Round"
                        }
                        <GRadio>{
                            theme: Info,
                            radio_type: Tick,
                            value: "Info_Tick"
                            text: "I just a label"
                        }
                        <GRadio>{
                            theme: Error,
                            radio_type: Cross,
                            value: "Error_Cross",
                            text: "act as button",
                            background_visible: true,
                            padding: {
                                left: 12.0, right: 12.0, top: 8.0, bottom: 8.0
                            },
                            background_color: #6F3121,
                            border_radius: 2.0
                        }
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GLabel>{
                            text: "Value: "
                        }
                        val_label = <GLabel>{
                            text: "",
                        }
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GLabel>{
                            text: "Selected: "
                        }
                        selected_label = <GLabel>{
                            text: "",
                        }
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
radio_group = <GRadioGroup>{
    spacing: 16.0,
    <GRadio>{
        theme: Success,
        radio_type: Round,
        value: "Success_Round"
    }
    <GRadio>{
        theme: Info,
        radio_type: Tick,
        value: "Info_Tick"
        text: "I just a label"
    }
    <GRadio>{
        theme: Error,
        radio_type: Cross,
        value: "Error_Cross",
        text: "act as button",
        background_visible: true,
        padding: {
            left: 12.0, right: 12.0, top: 8.0, bottom: 8.0
        },
        background_color: #6F3121,
        border_radius: 2.0
    }
}

fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

    let val_label = self.glabel(id!(val_label));
    let selected_label = self.glabel(id!(selected_label));

    let radio_group = self.gradio_group(id!(radio_group));

    if let Some(e) = radio_group.changed(&actions) {
        val_label.set_text(cx, &e.value.unwrap_or("Empty".to_string()));
        selected_label.set_text(cx, &e.selected.to_string());
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
pub struct RadioEnPage {
    #[deref]
    #[redraw]
    pub deref_widget: GView,
}

impl LiveHook for RadioEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for RadioEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let val_label = self.glabel(id!(val_label));
        let selected_label = self.glabel(id!(selected_label));

        let radio_group = self.gradio_group(id!(radio_group));

        if let Some(e) = radio_group.changed(&actions) {
            val_label.set_text(cx, e.value.unwrap_or("Empty".to_string()));
            selected_label.set_text(cx, e.selected.to_string());
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}