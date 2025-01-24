use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ViewAnPage = {{ViewAnPage}}{
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
                text: "View has two animation effects: Hover and Focus(Press).",
            }
            <GLabel>{
                width: Fill,
                text: "Hover: hover_color\nFocus(Press): focus_color",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Info,
                    hover_color: #FF0000,
                    focus_color: #00FF00,
                    animation_key: true,
                }
                <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Error,
                    animation_key: true,
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
                <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Info,
                    hover_color: #FF0000,
                    focus_color: #00FF00,
                    animation_key: true,
                }
                <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Error,
                    animation_key: true,
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
                flow: Right,
                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,
                    an_view1 = <GView>{
                        height: 60.0,
                        width: 60.0,
                        theme: Info,
                        hover_color: #FF0000,
                        focus_color: #00FF00,
                        animation_key: true,
                    }
                    hover_btn = <GButton>{
                        slot: {
                            text: "Hover Animation",
                        }
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,
                    an_view2 = <GView>{
                        height: 60.0,
                        width: 60.0,
                        theme: Error,
                        hover_color: #FF0000,
                        focus_color: #00FF00,
                        animation_key: true,
                    }
                    focus_btn = <GButton>{
                        slot: {
                            text: "Focus Animation",
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
pub struct ViewAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ViewAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ViewAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let an_view1 = self.gview(id!(an_view1));
        let an_view2 = self.gview(id!(an_view2));
        let hover_btn = self.gbutton(id!(hover_btn));
        let focus_btn = self.gbutton(id!(focus_btn));
        if hover_btn.clicked(&actions).is_some() {
            an_view1.animate_hover_on(cx);
        }
        if focus_btn.clicked(&actions).is_some() {
            an_view2.animate_focus_on(cx);
        }
    }
}
