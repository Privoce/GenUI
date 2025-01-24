use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    pub LabelAnPage = {{LabelAnPage}}{
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
                text: "When you need to enable the default animation, you need to set animation_key to true",
            }
            <GLabel>{
                width: Fill,
                text: "The default animation color is related to the passed theme",
            }
            <GLabel>{
                width: Fill,
                text: "Label animations are divided into two types: Hover and Focus (Press)",
            }
            <GLabel>{
                width: Fill,
                text: "Hover: stroke_hover_color\nFocus(Press): stroke_focus_color",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GLabel>{
                    cursor: Hand,
                    animation_key: true,
                    text: "Label with animation (cursor Hand)"
                }
                <GLabel>{
                    cursor: Hand,
                    animation_key: true,
                    theme: Error,
                    text: "Label with animation (theme Error)"
                }
                <GLabel>{
                    animation_key: true,
                    stroke_hover_color: #FF0000,
                    stroke_focus_color: #00FF00,
                    text: "with hover and focus(press) color"
                }
            }
            code = {
                body: {
                    <GLabel>{
                        theme: Dark,
                        width: Fill,
                        text: r#"
                <GLabel>{
                    cursor: Hand,
                    animation_key: true,
                    text: "Label with animation (cursor Hand)"
                }
                <GLabel>{
                    cursor: Hand,
                    animation_key: true,
                    theme: Error,
                    text: "Label with animation (theme Error)"
                }
                <GLabel>{
                    animation_key: true,
                    stroke_hover_color: #FF0000,
                    stroke_focus_color: #00FF00,
                    text: "with hover and focus(press) color"
                }
                        "#;
                    }
                }
            }
        }
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "Aniamtion Functions",
        }
        <GLabel>{
            width: Fill,
            text: "The component provides corresponding methods to activate using code",
        }
        an_box = <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,
                    align: {x: 0.5},
                    lb_hover = <GLabel>{
                        cursor: Hand,
                        stroke_hover_color: #DAAA38,
                        animation_key: true,
                        text: "Label: Hover"
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 24.0,
                        do_hover = <GButton>{
                            slot: {
                                text: "Active Hover"
                            }
                        }
                        clear_hover = <GButton>{
                            slot: {
                                text: "Clear Hover"
                            }
                        }
                    }
                }
                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,
                    align: {x: 0.5},
                    lb_focus = <GLabel>{
                        animation_key: true,
                        stroke_hover_color: #FF0000,
                        stroke_focus_color: #00FF00,
                        text: "Label: Focus (Press)"
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 24.0,
                        do_focus = <GButton>{
                            slot: {
                                text: "Active Focus"
                            }
                        }
                        clear_focus = <GButton>{
                            slot: {
                                text: "Clear Focus"
                            }
                        }
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 400.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
        lb_hover = <GLabel>{
            cursor: Hand,
            animation_key: true,
            text: "Label: Hover"
        }
    
        do_hover = <GButton>{
            slot: {
                text: "Active Hover"
            }
        }
        clear_hover = <GButton>{
            slot: {
                text: "Clear Hover"
            }
        }
    
        lb_focus = <GLabel>{
            animation_key: true,
            stroke_hover_color: #FF0000,
            stroke_focus_color: #00FF00,
            text: "Label: Focus (Press)"
        }
    
        do_focus = <GButton>{
            slot: {
                text: "Active Focus"
            }
        }
        clear_focus = <GButton>{
            slot: {
                text: "Clear Focus"
            }
        }
    
        fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
            let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

            let hover_btn = self.gbutton(id!(do_hover));
            let focus_btn = self.gbutton(id!(do_focus));
            let clear_hover = self.gbutton(id!(clear_hover));
            let clear_focus = self.gbutton(id!(clear_focus));
            let hover_lb = self.glabel(id!(lb_hover));
            let focus_lb = self.glabel(id!(lb_focus));
            if hover_btn.clicked(&actions).is_some() {
                // hover_lb.animate_hover_on(cx); // on animate duration and ease
                hover_lb.play_hover_on(cx);
            }
            if focus_btn.clicked(&actions).is_some() {
                // focus_lb.animate_focus_on(cx); // on animate duration and ease
                focus_lb.play_focus_on(cx);
            }
            if clear_hover.clicked(&actions).is_some() {
                hover_lb.animate_hover_off(cx);
            }
            if clear_focus.clicked(&actions).is_some() {
                focus_lb.animate_focus_off(cx);
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
pub struct LabelAnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LabelAnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LabelAnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let hover_btn = self.gbutton(id!(do_hover));
        let focus_btn = self.gbutton(id!(do_focus));
        let clear_hover = self.gbutton(id!(clear_hover));
        let clear_focus = self.gbutton(id!(clear_focus));
        let hover_lb = self.glabel(id!(lb_hover));
        let focus_lb = self.glabel(id!(lb_focus));
        if hover_btn.clicked(&actions).is_some() {
            // hover_lb.animate_hover_on(cx); // on animate duration and ease
            hover_lb.play_hover_on(cx);
        }
        if focus_btn.clicked(&actions).is_some() {
            // focus_lb.animate_focus_on(cx); // on animate duration and ease
            focus_lb.play_focus_on(cx);
        }
        if clear_hover.clicked(&actions).is_some() {
            hover_lb.animate_hover_off(cx);
        }
        if clear_focus.clicked(&actions).is_some() {
            focus_lb.animate_focus_off(cx);
        }
    }
}

