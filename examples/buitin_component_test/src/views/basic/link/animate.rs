use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub LinkAnPage = {{LinkAnPage}}{
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
        <GLabel>{
            width: Fill,
            text: "When the animation of Link is activated, the text, underline, and background will all have animation effects.",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                flow: Right,
                <GLink>{
                    theme: Error,
                    text: "GenUI Components Lib",
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    underline_hover_color: #0000FF,
                    underline_focus_color: #FFFF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                <GLink>{
                    theme: Error,
                    background_visible: true,
                    padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                    border_radius: 4.0,
                    underline_visible: false,
                    text: "Act as Button",
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                <GLink>{
                    theme: Error,
                    background_visible: true,
                    padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                    border_radius: 4.0,
                    underline_visible: false,
                    text: "More Define Colors",
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
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
                <GLink>{
                    theme: Error,
                    text: "GenUI Components Lib",
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    underline_hover_color: #0000FF,
                    underline_focus_color: #FFFF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                <GLink>{
                    theme: Error,
                    background_visible: true,
                    padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                    border_radius: 4.0,
                    underline_visible: false,
                    text: "Act as Button",
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                <GLink>{
                    theme: Error,
                    background_visible: true,
                    padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                    border_radius: 4.0,
                    underline_visible: false,
                    text: "More Define Colors",
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                            "#;
                        }
                    }
                }
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                flow: Right,
                an_link = <GLink>{
                    theme: Error,
                    background_visible: true,
                    padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                    border_radius: 4.0,
                    underline_visible: false,
                    text: "More Define Colors",
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                an_btn1 = <GButton>{
                    slot: {
                        text: "Hover"
                    }
                }
                an_btn2 = <GButton>{
                    slot: {
                        text: "Focus"
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
                an_link = <GLink>{
                    theme: Error,
                    background_visible: true,
                    padding: {left: 12.0, right: 12.0, top: 10.0, bottom: 10.0},
                    border_radius: 4.0,
                    underline_visible: false,
                    text: "More Define Colors",
                    hover_color: #F67D37,
                    focus_color: #FFD54F,
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                an_btn1 = <GButton>{
                    slot: {
                        text: "Hover"
                    }
                }
                an_btn2 = <GButton>{
                    slot: {
                        text: "Focus"
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let an_link = self.glink(id!(an_link));
                    let an_btn1 = self.gbutton(id!(an_btn1));
                    let an_btn2 = self.gbutton(id!(an_btn2));
                    if an_btn1.clicked(&actions).is_some() {
                        an_link.animate_hover_on(cx);
                    }
                    if an_btn2.clicked(&actions).is_some() {
                        an_link.animate_focus_on(cx);
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
pub struct LinkAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LinkAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LinkAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let an_link = self.glink(id!(an_link));
        let an_btn1 = self.gbutton(id!(an_btn1));
        let an_btn2 = self.gbutton(id!(an_btn2));
        if an_btn1.clicked(&actions).is_some() {
            an_link.animate_hover_on(cx);
        }
        if an_btn2.clicked(&actions).is_some() {
            an_link.animate_focus_on(cx);
        }
    }
}
