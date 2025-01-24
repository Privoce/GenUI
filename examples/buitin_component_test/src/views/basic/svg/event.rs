use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub SvgEnPage = {{SvgEnPage}}{
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
                text: "Svg has a series of events, but it only works when event_key is set to true.",
            }
            <GLabel>{
                width: Fill,
                text: "1. HoverIn\n2. HoverOut\n3. Focus\n4. FocusLost\n5. Clicked",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 48.0,
                flow: Right,
                e_svg = <GSvg>{
                    height: 32.0,
                    width: 32.0,
                    animation_key: true,
                    stroke_hover_color: #FF0000,
                    stroke_focus_color: #00FF00,
                    src: dep("crate://self/resources/all.svg"),
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
                e_svg = <GSvg>{
                    height: 32.0,
                    width: 32.0,
                    animation_key: true,
                    stroke_hover_color: #FF0000,
                    stroke_focus_color: #00FF00,
                    src: dep("crate://self/resources/all.svg"),
                }
                e_res = <GLabel>{
                    text: "Event Result"
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let e_svg = self.gview(id!(e_svg));
                    let e_res = self.glabel(id!(e_res));
                    if e_svg.hover_in(&actions).is_some() {
                        e_res.set_text(cx, "HoverIn");
                    }
                    if e_svg.hover_out(&actions).is_some() {
                        e_res.set_text(cx, "HoverOut");
                    }
                    if e_svg.focus(&actions).is_some() {
                        e_res.set_text(cx, "Focus");
                    }
                    if e_svg.focus_lost(&actions).is_some() {
                        e_res.set_text(cx, "FocusLost");
                    }
                    if e_svg.clicked(&actions).is_some() {
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
pub struct SvgEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for SvgEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SvgEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e_svg = self.gsvg(id!(e_svg));
        let e_res = self.glabel(id!(e_res));
        if e_svg.hover_in(&actions).is_some() {
            e_res.set_text(cx, "HoverIn".to_string());
        }
        if e_svg.hover_out(&actions).is_some() {
            e_res.set_text(cx, "HoverOut".to_string());
        }
        if e_svg.focus(&actions).is_some() {
            e_res.set_text(cx, "Focus".to_string());
        }
        if e_svg.focus_lost(&actions).is_some() {
            e_res.set_text(cx, "FocusLost".to_string());
        }
        if e_svg.clicked(&actions).is_some() {
            e_res.set_text(cx, "Clicked".to_string());
        }
    }
}
