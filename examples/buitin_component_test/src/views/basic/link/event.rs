use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub LinkEnPage = {{LinkEnPage}}{
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
                text: "Link has a variety of events, but actually you just need to focus on the Clicked.",
            }
            <GLabel>{
                width: Fill,
                text: "If href exists, the link will be opened in the browser. What's more, you can catch Clicked event to do some other things.",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 48.0,
                flow: Right,
                e_link = <GLink>{
                    theme: Error,
                    text: "GenUI Components Lib",
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    underline_hover_color: #0000FF,
                    underline_focus_color: #FFFF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
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
                e_link = <GLink>{
                    theme: Error,
                    text: "GenUI Components Lib",
                    text_hover_color: #FF0000,
                    text_focus_color: #00FF00,
                    underline_hover_color: #0000FF,
                    underline_focus_color: #FFFF00,
                    href: "https://github.com/Privoce/GenUI/tree/components/gen/components",
                }
                e_res = <GLabel>{
                    text: "Event Result"
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let e_link = self.glink(id!(e_link));
                    let e_res = self.glabel(id!(e_res));
                    if let Some(e) = e_link.clicked(&actions) {
                        let res = format!("{}: {}", e.ty, e.href.unwrap_or("empty".to_string()));
                        e_res.set_text(cx, &res);
                    }
                    if e_link.hover_in(&actions).is_some() {
                        e_res.set_text(cx, "Link Hover In!");
                    }
                    if e_link.hover_out(&actions).is_some() {
                        e_res.set_text(cx, "Link Hover Out!");
                    }
                    if e_link.focus(&actions).is_some() {
                        e_res.set_text(cx, "Link Focus!");
                    }
                    if e_link.focus_lost(&actions).is_some() {
                        e_res.set_text(cx, "Link Focus Lost!");
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
pub struct LinkEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LinkEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LinkEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e_link = self.glink(id!(e_link));
        let e_res = self.glabel(id!(e_res));
        if let Some(e) = e_link.clicked(&actions) {
            let res = format!("{}: {}", e.ty, e.href.unwrap_or("empty".to_string()));
            e_res.set_text(cx, res);
        }
        if e_link.hover_in(&actions).is_some() {
            e_res.set_text(cx, "Link Hover In!".to_string());
        }
        if e_link.hover_out(&actions).is_some() {
            e_res.set_text(cx, "Link Hover Out!".to_string());
        }
        if e_link.focus(&actions).is_some() {
            e_res.set_text(cx, "Link Focus!".to_string());
        }
        if e_link.focus_lost(&actions).is_some() {
            e_res.set_text(cx, "Link Focus Lost!".to_string());
        }
    }
}
