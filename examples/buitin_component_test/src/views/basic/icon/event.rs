use gen_components::components::{icon::GIconWidgetExt, label::GLabelWidgetExt, view::GView};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::styles::*;

    IconEnPage = {{IconEnPage}}{
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
        <CBox>{
            box_wrap = {
                spacing: 48.0,
                flow: Right,
                e_icon = <GIcon>{
                    icon_type: Max,
                    cursor: Hand,
                    stroke_hover_color: #F00,
                    stroke_focus_color: #0F0,
                    animation_key: true,
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
            e_icon = <GIcon>{
                icon_type: Max,
                cursor: Hand,
                stroke_hover_color: #F00,
                stroke_focus_color: #0F0,
                animation_key: true,
            }
            e_res = <GLabel>{
                text: "Event Result"
            }
            fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                let e_icon = self.gicon(id!(e_icon));
                let e_res = self.glabel(id!(e_res));
                if e_icon.hover_in(&actions).is_some() {
                    e_res.set_text_and_redraw(cx, "HoverIn");
                }
                if e_icon.hover_out(&actions).is_some() {
                    e_res.set_text_and_redraw(cx, "HoverOut");
                }
                if e_icon.clicked(&actions).is_some() {
                    e_res.set_text_and_redraw(cx, "Clicked");
                }
                if e_icon.focus(&actions).is_some() {
                    e_res.set_text_and_redraw(cx, "Focus");
                }
                if e_icon.focus_lost(&actions).is_some() {
                    e_res.set_text_and_redraw(cx, "FocusLost");
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
pub struct IconEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for IconEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for IconEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let e_icon = self.gicon(id!(e_icon));
        let e_res = self.glabel(id!(e_res));
        if e_icon.hover_in(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "HoverIn");
        }
        if e_icon.hover_out(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "HoverOut");
        }
        if e_icon.clicked(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "Clicked");
        }
        if e_icon.focus(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "Focus");
        }
        if e_icon.focus_lost(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "FocusLost");
        }
    }
}

