use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub TagUsagePage = {{TagUsagePage}}{
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
            text: "Basic Tag Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Tag is a simple component that can be used to display a tag with different themes.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GHLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GTag>{ text: "Tag1"}
                    <GTag>{theme: Dark, text: "Tag2"}
                    <GTag>{theme: Info, text: "Tag3"}
                    <GTag>{theme: Success, text: "Tag4"}
                    <GTag>{theme: Warning, text: "Tag5"}
                    <GTag>{theme: Error, text: "Tag6"}
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
                    <GTag>{ text: "Tag1"}
                    <GTag>{theme: Dark, text: "Tag2"}
                    <GTag>{theme: Info, text: "Tag3"}
                    <GTag>{theme: Success, text: "Tag4"}
                    <GTag>{theme: Warning, text: "Tag5"}
                    <GTag>{theme: Error, text: "Tag6"}
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct TagUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TagUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TagUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
