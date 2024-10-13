use gen_components::components::{
    label::GLabelWidgetExt,
    view::{GView, GViewWidgetExt},
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::styles::*;

    SvgEnPage = {{SvgEnPage}}{
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
                text: "View has a series of events, but it only works when event_key is set to true.",
            }
            <GLabel>{
                width: Fill,
                text: "1. HoverIn\n2. HoverOut\n3. Focus\n4. FocusLost\n5. Clicked\n6. Drag\n8. KeyDown\n9. KeyUp",
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 48.0,
                flow: Right,
                e_view = <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Info,
                    event_key: true,
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
                e_view = <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Info,
                    event_key: true,
                }
                e_res = <GLabel>{
                    text: "Event Result"
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let e_view = self.gview(id!(e_view));
                    let e_res = self.glabel(id!(e_res));
                    if e_view.hover_in(&actions).is_some() {
                        e_res.set_text_and_redraw(cx, "HoverIn");
                    }
                    if e_view.hover_out(&actions).is_some() {
                        e_res.set_text_and_redraw(cx, "HoverOut");
                    }
                    if e_view.focus(&actions).is_some() {
                        e_res.set_text_and_redraw(cx, "Focus");
                    }
                    if e_view.focus_lost(&actions).is_some() {
                        e_res.set_text_and_redraw(cx, "FocusLost");
                    }
                    if e_view.clicked(&actions).is_some() {
                        e_res.set_text_and_redraw(cx, "Clicked");
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

        let e_view = self.gview(id!(e_view));
        let e_res = self.glabel(id!(e_res));
        if e_view.hover_in(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "HoverIn");
        }
        if e_view.hover_out(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "HoverOut");
        }
        if e_view.focus(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "Focus");
        }
        if e_view.focus_lost(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "FocusLost");
        }
        if e_view.clicked(&actions).is_some() {
            e_res.set_text_and_redraw(cx, "Clicked");
        }
    }
}
