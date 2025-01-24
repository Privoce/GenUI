use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub SvgAnPage = {{SvgAnPage}}{
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
                text: "Svg has two animation effects: Hover and Focus(Press).But you need to set animation_key to true.",
            }
            <GLabel>{
                width: Fill,
                text: "Hover: stroke_hover_color\nFocus(Press): stroke_focus_color",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GSvg>{
                    height: 32.0,
                    width: 32.0,
                    animation_key: true,
                    stroke_hover_color: #FF0000,
                    stroke_focus_color: #00FF00,
                    src: dep("crate://self/resources/all.svg"),
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
                <GSvg>{
                    height: 32.0,
                    width: 32.0,
                    stroke_hover_color: #FF0000,
                    stroke_focus_color: #00FF00,
                    src: dep("crate://self/resources/all.svg"),
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
                    an_svg1 = <GSvg>{
                        height: 32.0,
                        width: 32.0,
                        animation_key: true,
                        stroke_hover_color: #FF0000,
                        stroke_focus_color: #00FF00,
                        src: dep("crate://self/resources/all.svg"),
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
                    an_svg2 = <GSvg>{
                        height: 32.0,
                        width: 32.0,
                        animation_key: true,
                        stroke_hover_color: #FF0000,
                        stroke_focus_color: #00FF00,
                        src: dep("crate://self/resources/all.svg"),
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
pub struct SvgAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for SvgAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SvgAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let an_svg1 = self.gsvg(id!(an_svg1));
        let an_svg2 = self.gsvg(id!(an_svg2));
        let hover_btn = self.gbutton(id!(hover_btn));
        let focus_btn = self.gbutton(id!(focus_btn));
        if hover_btn.clicked(&actions).is_some() {
            an_svg1.animate_hover_on(cx);
        }
        if focus_btn.clicked(&actions).is_some() {
            an_svg2.animate_focus_on(cx);
        }
    }
}
