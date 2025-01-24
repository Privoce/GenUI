use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ToggleEnPage = {{ToggleEnPage}}{
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
                text: "Toggle has 3 events: 1. Clicked 2. HoverIn 3. HoverOut",
            }
        }

        an_box = <CBox>{
            box_wrap = {
                spacing: 8.0,
                tg = <GToggle>{
                    theme: Warning,
                    hover_color: #00FF00,
                    stroke_hover_color: #FF0000,
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GLabel>{
                            text: "Event: "
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
                tg = <GToggle>{
                    theme: Warning,
                    hover_color: #00FF00,
                    stroke_hover_color: #FF0000,
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        <GLabel>{
                            text: "Event: "
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
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let val_label = self.glabel(id!(val_label));
                    let selected_label = self.glabel(id!(selected_label));

                    let tg = self.gtoggle(id!(tg));

                    if let Some(e) = tg.clicked(&actions) {
                        val_label.set_text(cx, "Clicked");
                        if e.selected {
                            selected_label.set_text(cx, "Selected");
                        } else {
                            selected_label.set_text(cx, "Unselected");
                        }
                    }
                    if tg.hover_in(&actions).is_some() {
                        val_label.set_text(cx, "Hover In");
                    }
                    if tg.hover_out(&actions).is_some() {
                        val_label.set_text(cx, "Hover Out");
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
pub struct ToggleEnPage {
    #[deref]
    #[redraw]
    pub deref_widget: GView,
}

impl LiveHook for ToggleEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ToggleEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let val_label = self.glabel(id!(val_label));
        let selected_label = self.glabel(id!(selected_label));

        let tg = self.gtoggle(id!(tg));

        if let Some(e) = tg.clicked(&actions) {
            val_label.set_text(cx, "Clicked".to_string());
            if e.selected {
                selected_label.set_text(cx, "Selected".to_string());
            } else {
                selected_label.set_text(cx, "Unselected".to_string());
            }
        }
        if tg.hover_in(&actions).is_some() {
            val_label.set_text(cx, "Hover In".to_string());
        }
        if tg.hover_out(&actions).is_some() {
            val_label.set_text(cx, "Hover Out".to_string());
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}
