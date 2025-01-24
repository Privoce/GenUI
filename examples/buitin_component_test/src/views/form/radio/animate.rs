use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    pub RadioAnPage = {{RadioAnPage}}{
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
            text: "Animation Usage",
        }
        <GVLayout>{
            height: Fit,
            spacing: 8.0,
            <GLabel>{
                width: Fill,
                text: "Radio Animation has hover animation. Radio has 3 parts:\n1. Radio\n2. Label\n3. Wrapper",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GRadioGroup>{
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
                        radio_background_color: #2D7D9A,
                        radio_border_color: #FF0000,
                        radio_hover_color: #00FF00,
                        radio_selected_color: #FF00FF,
                        stroke_color: #0000FF,
                        border_radius: 2.0
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
                        radio_background_color: #2D7D9A,
                        radio_border_color: #FF0000,
                        radio_hover_color: #00FF00,
                        radio_selected_color: #FF00FF,
                        stroke_color: #0000FF,
                        border_radius: 2.0
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let hover_btn = self.gbutton(id!(do_hover));
                    let focus_btn = self.gbutton(id!(do_focus));
                    let clear_hover = self.gbutton(id!(clear_hover));
                    let clear_focus = self.gbutton(id!(clear_focus));
                    let hover_lb = self.glabel(id!(lb_hover));
                    let focus_lb = self.glabel(id!(lb_focus));
                    if hover_btn.clicked(&actions).is_some() {
                        hover_lb.animate_hover_on(cx);
                    }
                    if focus_btn.clicked(&actions).is_some() {
                        focus_lb.animate_focus_on(cx);
                    }
                    if clear_hover.clicked(&actions).is_some() {
                        hover_lb.animate_hover_off(cx);
                    }
                    if clear_focus.clicked(&actions).is_some() {
                        focus_lb.animate_focus_off(cx);
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
pub struct RadioAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for RadioAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for RadioAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let hover_btn = self.gbutton(id!(do_hover));
        let focus_btn = self.gbutton(id!(do_focus));
        let clear_hover = self.gbutton(id!(clear_hover));
        let clear_focus = self.gbutton(id!(clear_focus));
        let hover_lb = self.glabel(id!(lb_hover));
        let focus_lb = self.glabel(id!(lb_focus));
        if hover_btn.clicked(&actions).is_some() {
            hover_lb.animate_hover_on(cx);
        }
        if focus_btn.clicked(&actions).is_some() {
            focus_lb.animate_focus_on(cx);
        }
        if clear_hover.clicked(&actions).is_some() {
            hover_lb.animate_hover_off(cx);
        }
        if clear_focus.clicked(&actions).is_some() {
            focus_lb.animate_focus_off(cx);
        }
    }
}

