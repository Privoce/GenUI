use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ImageEnPage = {{ImageEnPage}}{
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
        <GLabel>{
            width: Fill,
            text: "Image has 3 events: HoverIn, HoverOut, Clicked.",
        }
        <CBox>{
            box_wrap = {
                spacing: 48.0,
                flow: Right,
                e_image = <GImage>{
                    height: 56.0,
                    width: 64.0,
                    src: dep("crate://self/resources/rust.png"),
                }
                e_res = <GLabel>{
                    text: "Event Result"
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
                e_image = <GImage>{
                    height: 56.0,
                    width: 64.0,
                    src: dep("crate://self/resources/rust.png"),
                }
                e_res = <GLabel>{
                    text: "Event Result"
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let e_image = self.gimage(id!(e_image));
                    let e_res = self.glabel(id!(e_res));
                    if e_image.hover_in(&actions).is_some() {
                        e_res.set_text(cx, "HoverIn");
                    }
                    if e_image.hover_out(&actions).is_some() {
                        e_res.set_text(cx, "HoverOut");
                    }
                    if e_image.clicked(&actions).is_some() {
                        e_res.set_text(cx, "Clicked");
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
pub struct ImageEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ImageEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ImageEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e_image = self.gimage(id!(e_image));
        let e_res = self.glabel(id!(e_res));
        if e_image.hover_in(&actions).is_some() {
            e_res.set_text(cx, "HoverIn".to_string());
        }
        if e_image.hover_out(&actions).is_some() {
            e_res.set_text(cx, "HoverOut".to_string());
        }
        if e_image.clicked(&actions).is_some() {
            e_res.set_text(cx, "Clicked".to_string());
        }
    }
}
