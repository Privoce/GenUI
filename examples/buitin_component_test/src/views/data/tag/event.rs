use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub TagEnPage = {{TagEnPage}}{
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
                text: "Tag has a series of events:",
            }
            <GLabel>{
                width: Fill,
                text: "1. HoverIn\n2. HoverOut\n3. Focus\n4. FocusLost\n5. Clicked\n6. Closed",
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
                se_btn = <GButton>{
                    slot: {
                        text: "Back Tag Visible"
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
               
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct TagEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TagEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TagEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let mut tg = self.gtag(id!(tg));
        let se_btn = self.gbutton(id!(se_btn));
        let e_res = self.glabel(id!(e_res));
        if tg.clicked(&actions).is_some() {
            e_res.set_text(cx, "Tag Clicked!".to_string());
        }
        if tg.hover_in(&actions).is_some() {
            e_res.set_text(cx, "Tag Hover In!".to_string());
        }
        if tg.hover_out(&actions).is_some() {
            e_res.set_text(cx, "Tag Hover Out!".to_string());
        }
        if tg.focus(&actions).is_some() {
            e_res.set_text(cx, "Tag Focus!".to_string());
        }
        if tg.focus_lost(&actions).is_some() {
            e_res.set_text(cx, "Tag Focus Lost!".to_string());
        }
        if tg.closed(&actions).is_some() {
            e_res.set_text(cx, "Tag Closed!".to_string());
            tg.set_visible(cx, false);
        }
        if se_btn.clicked(&actions).is_some() {
            tg.set_visible(cx, true);
        }

    }
}
