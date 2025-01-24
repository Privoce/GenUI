use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ScrollEnPage = {{ScrollEnPage}}{
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
            text: "This example will show you how to adjust the x and y position of the scroll bar.",
        }
        <CBox>{
            box_wrap = {
                spacing: 12.0,
                flow: Down,
                align: {x: 0.5},
                scroll_view = <GHLayout>{
                    clip_x: true,
                    clip_y: true,
                    height: 200.0,
                    width: Fill,
                    spacing: 16.0,
                    scroll_bars: <GScrollBars>{},
                    <GView>{
                        theme: Error,
                        height: 300.0,
                        width: 300.0,
                    }
                    <GView>{
                        theme: Success,
                        height: 100.0,
                        width: 600.0,
                    }
                }
                e_btn = <GButton>{
                    slot: {
                        text: "Adjust ScrollBar",
                    }
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
                scroll_view = <GHLayout>{
                    clip_x: true,
                    clip_y: true,
                    height: 200.0,
                    width: Fill,
                    spacing: 16.0,
                    scroll_bars: <GScrollBars>{},
                    <GView>{
                        theme: Error,
                        height: 300.0,
                        width: 300.0,
                    }
                    <GView>{
                        theme: Success,
                        height: 100.0,
                        width: 600.0,
                    }
                }
                e_btn = <GButton>{
                    slot: {
                        text: "Adjust ScrollBar",
                    }
                }
                fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                    let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

                    let scroll_view = self.gview(id!(scroll_view));
                    let e_btn = self.gbutton(id!(e_btn));
                    if e_btn.clicked(&actions).is_some() {
                        scroll_view.set_scroll_pos(cx, Some(32.0), Some(88.0));
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
pub struct ScrollEnPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ScrollEnPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ScrollEnPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let scroll_view = self.gview(id!(scroll_view));
        let e_btn = self.gbutton(id!(e_btn));
        if e_btn.clicked(&actions).is_some() {
            scroll_view.set_scroll_pos(cx, Some(32.0), Some(88.0));
        }
    }
}
