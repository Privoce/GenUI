use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ButtonEnPage = {{ButtonEnPage}}{
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
                text: "Button has a series of events:",
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
                e_btn = <GButton>{
                    slot: {
                        text: "Event Button!"
                    }
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
                e_btn = <GButton>{
                    slot: {
                        text: "Event Button!"
                    }
                }
                e_res = <GLabel>{
                    text: "Event Result"
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let e_btn = self.gbutton(id!(e_btn));
                    let e_res = self.glabel(id!(e_res));
                    if e_btn.clicked(&actions).is_some() {
                        e_res.set_text(cx, "Button Clicked!");
                    }
                    if e_btn.hover_in(&actions).is_some() {
                        e_res.set_text(cx, "Button Hover In!");
                    }
                    if e_btn.hover_out(&actions).is_some() {
                        e_res.set_text(cx, "Button Hover Out!");
                    }
                    if e_btn.focus(&actions).is_some() {
                        e_res.set_text(cx, "Button Focus!");
                    }
                    if e_btn.focus_lost(&actions).is_some() {
                        e_res.set_text(cx, "Button Focus Lost!");
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
pub struct ButtonEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ButtonEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ButtonEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e_btn = self.gbutton(id!(e_btn));
        let e_res = self.glabel(id!(e_res));
        if e_btn.clicked(&actions).is_some() {
            e_res.set_text(cx, "Button Clicked!".to_string());
        }
        if e_btn.hover_in(&actions).is_some() {
            e_res.set_text(cx, "Button Hover In!".to_string());
        }
        if e_btn.hover_out(&actions).is_some() {
            e_res.set_text(cx, "Button Hover Out!".to_string());
        }
        if e_btn.focus(&actions).is_some() {
            e_res.set_text(cx, "Button Focus!".to_string());
        }
        if e_btn.focus_lost(&actions).is_some() {
            e_res.set_text(cx, "Button Focus Lost!".to_string());
        }
    }
}
