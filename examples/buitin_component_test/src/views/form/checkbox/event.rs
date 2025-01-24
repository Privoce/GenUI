use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub CheckboxEnPage = {{CheckboxEnPage}}{
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
                text: "CheckBox has 3 events: 1. Clicked 2. HoverIn 3. HoverOut",
            }
            <GLabel>{
                text: "CheckBoxGroup just has 1 event:\n1. Changed\nChanged event can get the selected index and value.",
            }
        }

        an_box = <CBox>{
            box_wrap = {
                spacing: 8.0,

                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,

                    checkbox_group = <GCheckboxGroup>{
                        spacing: 16.0,
                        <GCheckbox>{
                            theme: Success,
                            checkbox_type: Round,
                            value: "Success_Round"
                        }
                        <GCheckbox>{
                            theme: Info,
                            checkbox_type: Tick,
                            value: "Info_Tick"
                            text: "I just a label"
                        }
                        <GCheckbox>{
                            theme: Error,
                            checkbox_type: Cross,
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
                            text: "Values: "
                        }
                        val_label = <GLabel>{
                            text: "",
                        }
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GLabel>{
                            text: "Selecteds: "
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
checkbox_group = <GCheckboxGroup>{
    spacing: 16.0,
    <GCheckbox>{
        theme: Success,
        checkbox_type: Round,
        value: "Success_Round"
    }
    <GCheckbox>{
        theme: Info,
        checkbox_type: Tick,
        value: "Info_Tick"
        text: "I just a label"
    }
    <GCheckbox>{
        theme: Error,
        checkbox_type: Cross,
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

    let checkbox_group = self.gcheckbox_group(id!(checkbox_group));

    if let Some(e) = checkbox_group.changed(&actions) {
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
pub struct CheckboxEnPage {
    #[deref]
    #[redraw]
    pub deref_widget: GView,
}

impl LiveHook for CheckboxEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for CheckboxEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let val_label = self.glabel(id!(val_label));
        let selected_label = self.glabel(id!(selected_label));
        let checkbox_group = self.gcheckbox_group(id!(checkbox_group));
      

        if let Some(e) = checkbox_group.changed(&actions) {
            let val_str = e
                .values
                .iter()
                .map(|x| x.as_ref().unwrap_or(&String::new()).to_string())
                .collect::<Vec<String>>()
                .join(", ");

            val_label.set_text(cx, val_str);
            let selected = e
                .selected
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            selected_label.set_text(cx, selected);
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}
