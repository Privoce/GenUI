use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ToggleUsagePage = {{ToggleUsagePage}}{
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
            text: "Basic Toggle Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Toggle is a component just like a checkbox, it can be switched on or off.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GToggle>{
           
                }
                <GToggle>{
                    theme: Error,
                    toggle_type: Rect,
                    animation_key: false,
                }
                <GToggle>{
                    theme: Warning,
                    selected: true,
                }
                <GToggle>{
                    theme: Warning,
                    selected: true,
                    background_visible: false,
                }
                <GToggle>{
                    theme: Success,
                    selected: true,
                    toggle_type: Rect,
                    background_visible: false,
                    height: 28.0,
                    width: 60.0
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 240.0,
                        scroll_bars: <GScrollBars>{}
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GToggle>{
           
                }
                <GToggle>{
                    theme: Error,
                    toggle_type: Rect,
                    animation_key: false,
                }
                <GToggle>{
                    theme: Warning,
                    selected: true,
                }
                <GToggle>{
                    theme: Warning,
                    selected: true,
                    background_visible: false,
                }
                <GToggle>{
                    theme: Success,
                    selected: true,
                    toggle_type: Rect,
                    background_visible: false,
                    height: 28.0,
                    width: 60.0
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
pub struct ToggleUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ToggleUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ToggleUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
