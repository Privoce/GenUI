use makepad_widgets::Cx;

// pub mod usage;
// pub mod animate;
// pub mod event;
// pub mod virt;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub LoadingPage = {{LoadingPage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        padding: 12.0,
        scroll_bars: <GScrollBars>{},
        clip_x: true,
        clip_y: true,
        <GVLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "Shader",
            }

        }
        <GLabel>{
            width: Fill,
            text: "Shader is a component for drawing custom graphics, you can use self.opened to control the shader(if you need to use self.time).",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GVLayout>{
                    height: Fit,
                    <GView>{
                        height: 100.0,
                        width: Fill,
                        flow: Down,
                        spacing: 16.0,
                        align: {x: 0.5, y: 0.5},
                        ld1 = <GLoading>{
                            height: 64.0,
                            width: 64.0,
                            theme: Error,
                            visible: true,
                        }
                        <GLabel>{
                            text: "Loading ...",
                        }
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        align: {x: 0.5, y: 0.5},
                        open_btn1 = <GButton>{
                            slot: {
                                text: "Open Shader",
                            }
                        }
                        close_btn1 = <GButton>{
                            slot: {
                                text: "Close Shader",
                            }
                        }
                    }
                }
                <GVLayout>{
                    height: Fit,
                    <GView>{
                        height: 100.0,
                        width: Fill,
                        flow: Down,
                        spacing: 16.0,
                        align: {x: 0.5, y: 0.5},
                        ld2 = <GLoading>{
                            height: 64.0,
                            width: 64.0,
                            theme: Success,
                            loading_type: CircleDot
                        }
                        <GLabel>{
                            text: "Loading ...",
                        }
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        align: {x: 0.5, y: 0.5},
                        open_btn2 = <GButton>{
                            slot: {
                                text: "Open Shader",
                            }
                        }
                        close_btn2 = <GButton>{
                            slot: {
                                text: "Close Shader",
                            }
                        }
                    }
                }
                <GVLayout>{
                    height: Fit,
                    <GView>{
                        height: 100.0,
                        width: Fill,
                        flow: Down,
                        spacing: 16.0,
                        align: {x: 0.5, y: 0.5},
                        ld3 = <GLoading>{
                            theme: Warning,
                            loading_type: DotLine
                        }
                        <GLabel>{
                            text: "Loading ...",
                        }
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        align: {x: 0.5, y: 0.5},
                        open_btn3 = <GButton>{
                            slot: {
                                text: "Open Shader",
                            }
                        }
                        close_btn3 = <GButton>{
                            slot: {
                                text: "Close Shader",
                            }
                        }
                    }
                }
                o_label = <GLabel>{
                    text: "Target"
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
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct LoadingPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LoadingPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LoadingPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut ld1 = self.gloading(id!(ld1));
        let mut ld2 = self.gloading(id!(ld2));
        let mut ld3 = self.gloading(id!(ld3));
        let open_btn1 = self.gbutton(id!(open_btn1));
        let close_btn1 = self.gbutton(id!(close_btn1));
        let open_btn2 = self.gbutton(id!(open_btn2));
        let close_btn2 = self.gbutton(id!(close_btn2));
        let open_btn3 = self.gbutton(id!(open_btn3));
        let close_btn3 = self.gbutton(id!(close_btn3));
        let o_label = self.glabel(id!(o_label));
        if open_btn1.clicked(&actions).is_some() {
            ld1.open(cx);
            o_label.set_text(cx, "Opened Loading1".to_string());
        }
        if close_btn1.clicked(&actions).is_some() {
            ld1.close(cx);
            o_label.set_text(cx, "Closed Loading1".to_string());
        }

        if open_btn2.clicked(&actions).is_some() {
            ld2.open(cx);
            o_label.set_text(cx, "Opened Loading2".to_string());
        }
        if close_btn2.clicked(&actions).is_some() {
            ld2.close(cx);
            o_label.set_text(cx, "Closed Loading2".to_string());
        }

        if open_btn3.clicked(&actions).is_some() {
            ld3.open(cx);
            o_label.set_text(cx, "Opened Loading3".to_string());
        }
        if close_btn3.clicked(&actions).is_some() {
            ld3.close(cx);
            o_label.set_text(cx, "Closed Loading3".to_string());
        }
    }
}
