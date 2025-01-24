use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub DividerEnPage = {{DividerEnPage}}{
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
                text: "Although Divider has events, call these events are useless. What you need to do is handle child components.",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 48.0,
                flow: Right,
                e_divider = <GDivider>{
                    theme: Error,
                    height: Fit,
                    stroke_width: 2.0,
                    animation_key: true,
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    <GView>{
                        height: Fit,
                        width: Fit,
                        padding: 6.0,
                        spacing: 8.0,
                        e_res = <GLabel>{
                            text: "Click the divider!"
                        }
                        e_btn = <GButton>{
                            slot: {
                                text: "Click Me!",
                            }
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
                e_divider = <GDivider>{
                    theme: Error,
                    height: Fit,
                    stroke_width: 2.0,
                    animation_key: true,
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    <GView>{
                        height: Fit,
                        width: Fit,
                        padding: 6.0,
                        spacing: 8.0,
                        e_res = <GLabel>{
                            text: "Click the divider!"
                        }
                        e_btn = <GButton>{
                            slot: {
                                text: "Click Me!",
                            }
                        }
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let e_divider = self.gdivider(id!(e_divider));
                    let e_res = self.glabel(id!(e_res));
                    let e_btn = self.gbutton(id!(e_btn));
                    if e_btn.clicked(&actions).is_some() {
                        e_res.set_text(cx, "You clicked the button!");
                    }
                    if e_divider.clicked(&actions).is_some() {
                        e_res.set_text(cx, "Divider has been Clicked!");
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
pub struct DividerEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for DividerEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for DividerEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e_divider = self.gdivider(id!(e_divider));
        let e_res = self.glabel(id!(e_res));
        let e_btn = self.gbutton(id!(e_btn));
        if e_btn.clicked(&actions).is_some() {
            e_res.set_text(cx, "You clicked the button!".to_string());
        }
        if e_divider.clicked(&actions).is_some() {
            e_res.set_text(cx, "Divider has been Clicked!".to_string());
        }
    }
}
