use gen_components::components::{button::GButtonWidgetExt, checkbox::GCheckBoxWidgetExt, view::GView};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::styles::*;
    ProgressAnPage = {{ProgressAnPage}}{
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
                text: "",
            }
            <GLabel>{
                width: Fill,
                text: r#""#,
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
                    <GLabel>{
                        theme: Dark,
                        width: Fill,
                        text: r#"
                <GCheckBox>{
                    theme: Success,
                    checkbox_type: Tick,
                    value: "Success_Cross",
                    text: "act as button",
                    background_visible: true,
                    padding: {
                        left: 12.0, right: 12.0, top: 8.0, bottom: 8.0
                    },
                    background_color: #6F3121,
                    checkbox_background_color: #2D7D9A,
                    checkbox_border_color: #FF0000,
                    checkbox_hover_color: #00FF00,
                    checkbox_selected_color: #FF00FF,
                    border_radius: 2.0
                }
                        "#;
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
        let cb = self.gcheck_box(id!(cb));
        let a_btn1 = self.gbutton(id!(a_btn1));
        if a_btn1.clicked(&actions).is_some(){
            cb.animate_hover_on(cx);
        }
    }
}
