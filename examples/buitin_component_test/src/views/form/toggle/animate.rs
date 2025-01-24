use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    pub ToggleAnPage = {{ToggleAnPage}}{
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
                text: "Radio Animation has hover animation. Radio has 3 parts:\n1. CheckBox\n2. Label\n3. Wrapper",
            }
            <GLabel>{
                width: Fill,
                text: r#""#,
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                tg = <GToggle>{
                    theme: Warning,
                    selected: true,
                    hover_color: #00FF00,
                    stroke_hover_color: #FF0000,
                }
                a_btn1 = <GButton>{
                    slot: {
                        text: "Hover"
                    }
                }
            }
            code = {
                body: {
                    <GLabel>{
                        theme: Dark,
                        width: Fill,
                        text: r#"
                tg = <GToggle>{
                    theme: Warning,
                    selected: true,
                    hover_color: #00FF00,
                    stroke_hover_color: #FF0000,
                }
                a_btn1 = <GButton>{
                    slot: {
                        text: "Hover"
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
                    let tg = self.gtoggle(id!(tg));
                    let a_btn1 = self.gbutton(id!(a_btn1));
                    if a_btn1.clicked(&actions).is_some() {
                        tg.animate_hover_on(cx);
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
pub struct ToggleAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ToggleAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ToggleAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let tg = self.gtoggle(id!(tg));
        let a_btn1 = self.gbutton(id!(a_btn1));
        if a_btn1.clicked(&actions).is_some() {
            tg.animate_hover_on(cx);
        }
    }
}
