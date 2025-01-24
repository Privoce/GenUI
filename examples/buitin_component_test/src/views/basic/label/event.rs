use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub LabelEnPage = {{LabelEnPage}}{
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
                text: "When an event needs to be started, event_key needs to be set to true. See [`GLabelEvent`]",
            }
        }

        an_box = <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GVLayout>{
                    height: Fit,
                    spacing: 12.0,
                    lb_hover = <GLabel>{
                        event_key: true,
                        animation_key: true,
                        text: "Label Event: HoverIn | HoverOut"
                    }
                    lb_focus = <GLabel>{
                        event_key: true,
                        animation_key: true,
                        text: "Label Event: Focus (Press)"
                    }
                }
                event_res = <GLabel>{
                    margin: {right: 12.0},
                    text: "Wait Label Event",
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 300.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
    lb_hover = <GLabel>{
        event_key: true,
        animation_key: true,
        text: "Label Event: HoverIn | HoverOut"
    }
    lb_focus = <GLabel>{
        event_key: true,
        animation_key: true,
        text: "Label Event: Focus (Press)"
    }
    event_res = <GLabel>{
        margin: {right: 12.0},
        text: "Wait Label Event",
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let lb_hover = self.glabel(id!(lb_hover));
        let lb_focus = self.glabel(id!(lb_focus));
        let event_res = self.glabel(id!(event_res));

        if lb_hover.hover_in(&actions).is_some(){
            
            event_res.set_text(cx, "HoverIn");
        }
        if lb_hover.hover_out(&actions).is_some(){
            event_res.set_text(cx, "HoverOut");
        }
        if lb_focus.focus(&actions).is_some(){
            event_res.set_text(cx, "Focus");
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
pub struct LabelEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LabelEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LabelEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let lb_hover = self.glabel(id!(lb_hover));
        let lb_focus = self.glabel(id!(lb_focus));
        let event_res = self.glabel(id!(event_res));

        if lb_hover.hover_in(&actions).is_some() {
            event_res.set_text(cx, "HoverIn".to_string());
        }
        if lb_hover.hover_out(&actions).is_some() {
            event_res.set_text(cx, "HoverOut".to_string());
        }
        if lb_focus.focus(&actions).is_some() {
            event_res.set_text(cx, "Focus".to_string());
        }

        if let Some(e) = lb_hover.hover_in(&actions){
            dbg!(e);
            lb_hover.set_text(cx, "HoverIn".to_string());
        }
    }
}
