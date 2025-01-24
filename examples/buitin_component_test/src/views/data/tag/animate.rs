use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub TagAnPage = {{TagAnPage}}{
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
                text: "Tag has two animation effects: Hover and Focus(Press).",
            }
            <GLabel>{
                width: Fill,
                text: "Hover: hover_color\nFocus(Press): focus_color",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                tg = <GTag>{
                    hover_color: #FF0000,
                    focus_color: #00FF00,
                    text_hover_color: #FFFFFF,
                    text_focus_color: #4C4C4C,
                    stroke_hover_color: #0000FF,
                    stroke_focus_color: #000000,
                    text: "Animation Tag",
                    closeable: true,
                    src: dep("crate://self/resources/config.svg"),
                }

                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
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
pub struct TagAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TagAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TagAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let tg = self.gtag(id!(tg));
        let an_btn1 = self.gbutton(id!(an_btn1));
        let an_btn2 = self.gbutton(id!(an_btn2));
        if an_btn1.clicked(&actions).is_some() {
            tg.animate_hover_on(cx);
        }
        if an_btn2.clicked(&actions).is_some() {
            tg.animate_focus_on(cx);
        }
    }
}
