use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    pub ProgressAnPage = {{ProgressAnPage}}{
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
                text: "GProgress has 2 animations: 1. Hover 2. Focus",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                cb = <GProgress>{
                    theme: Info,
                    value: 0.3,
                    read_only: false,
                    background_color: #FF0000,
                    hover_color: #00FF00,
                    focus_color: #FF00FF,
                    stroke_hover_color: #0000FF,
                    stroke_focus_color: #FFFF00,
                }
                a_btn1 = <GButton>{
                    slot: {
                        text: "Hover"
                    }
                }
                a_btn2 = <GButton>{
                    slot: {
                        text: "Focus(Drag)"
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 240.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                    cb = <GProgress>{
                        theme: Info,
                        value: 0.3,
                        read_only: false,
                        background_color: #FF0000,
                        hover_color: #00FF00,
                        focus_color: #FF00FF,
                        stroke_hover_color: #0000FF,
                        stroke_focus_color: #FFFF00,
                    }
                    a_btn1 = <GButton>{
                        slot: {
                            text: "Hover"
                        }
                    }
                    a_btn2 = <GButton>{
                        slot: {
                            text: "Focus(Drag)"
                        }
                    }
                    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                        let cb = self.gprogress(id!(cb));
                        let a_btn1 = self.gbutton(id!(a_btn1));
                        let a_btn2 = self.gbutton(id!(a_btn2));
                        if a_btn1.clicked(&actions).is_some() {
                            cb.animate_hover_on(cx);
                        }
                        if a_btn2.clicked(&actions).is_some() {
                            cb.animate_focus_on(cx);
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
pub struct ProgressAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ProgressAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ProgressAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let cb = self.gprogress(id!(cb));
        let a_btn1 = self.gbutton(id!(a_btn1));
        let a_btn2 = self.gbutton(id!(a_btn2));
        if a_btn1.clicked(&actions).is_some() {
            cb.animate_hover_on(cx);
        }
        if a_btn2.clicked(&actions).is_some() {
            cb.animate_focus_on(cx);
        }
    }
}
