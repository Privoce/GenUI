use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub DividerAnPage = {{DividerAnPage}}{
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
                text: "Divider inherits View, so it has Hover and Focus too. But you still need to set animation_key to true.",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GDivider>{
                    theme: Success,
                    height: Fit,
                    stroke_width: 2.0,
                    animation_key: true,
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    <GView>{
                        height: 20.0,
                        width: Fit,
                        padding: {left: 8.0, right: 8.0},
                        <GLabel>{
                            text: "Hello!"
                        }
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 200.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GDivider>{
                    theme: Success,
                    height: Fit,
                    stroke_width: 2.0,
                    animation_key: true,
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    <GView>{
                        height: 20.0,
                        width: Fit,
                        padding: {left: 8.0, right: 8.0},
                        <GLabel>{
                            text: "Hello!"
                        }
                    }
                }
                            "#;
                        }
                    }
                }
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Down,
                an_divider = <GDivider>{
                    theme: Success,
                    height: Fit,
                    stroke_width: 2.0,
                    animation_key: true,
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    <GView>{
                        height: 20.0,
                        width: Fit,
                        padding: {left: 8.0, right: 8.0},
                        <GLabel>{
                            text: "Call Animation"
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
                    an_btn1 = <GButton>{
                        slot: {
                            text: "Call Hover Animation"
                        }
                    }
                    an_btn2 = <GButton>{
                        slot: {
                            text: "Call Focus Animation"
                        }
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 200.0,
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
pub struct DividerAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for DividerAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for DividerAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let an_divider = self.gdivider(id!(an_divider));
        let an_btn1 = self.gbutton(id!(an_btn1));
        let an_btn2 = self.gbutton(id!(an_btn2));
        if an_btn1.clicked(&actions).is_some() {
            an_divider.animate_hover_on(cx);
        }
        if an_btn2.clicked(&actions).is_some() {
            an_divider.animate_focus_on(cx);
        }
    }
}
