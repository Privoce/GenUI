use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub IconAnPage = {{IconAnPage}}{
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
                text: "Icon has two animation effects: Hover and Focus(Press).But you need to set animation_key to true.",
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
                <GIcon>{
                    icon_type: Max,
                    cursor: Hand,
                    stroke_hover_color: #F00,
                    stroke_focus_color: #0F0,
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
                <GIcon>{
                    icon_type: Max,
                    cursor: Hand,
                    stroke_hover_color: #F00,
                    stroke_focus_color: #0F0,
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
                    an_icon1 = <GIcon>{
                        icon_type: Max,
                        cursor: Hand,
                        stroke_hover_color: #F00,
                        stroke_focus_color: #0F0,
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
                    an_icon2 = <GIcon>{
                        icon_type: Max,
                        cursor: Hand,
                        stroke_hover_color: #F00,
                        stroke_focus_color: #0F0,
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let an_icon1 = self.gicon(id!(an_icon1));
        let an_icon2 = self.gicon(id!(an_icon2));
        let hover_btn = self.gbutton(id!(hover_btn));
        let focus_btn = self.gbutton(id!(focus_btn));
        if hover_btn.clicked(&actions).is_some() {
            an_icon1.animate_hover_on(cx);
        }
        if focus_btn.clicked(&actions).is_some() {
            an_icon2.animate_focus_on(cx);
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
pub struct IconAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for IconAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for IconAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let an_icon1 = self.gicon(id!(an_icon1));
        let an_icon2 = self.gicon(id!(an_icon2));
        let hover_btn = self.gbutton(id!(hover_btn));
        let focus_btn = self.gbutton(id!(focus_btn));
        if hover_btn.clicked(&actions).is_some() {
            an_icon1.animate_hover_on(cx);
        }
        if focus_btn.clicked(&actions).is_some() {
            an_icon2.animate_focus_on(cx);
        }
    }
}
