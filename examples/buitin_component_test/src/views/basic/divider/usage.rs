use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub DividerUsagePage = {{DividerUsagePage}}{
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
            text: "Basic Divider Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Divider is a component that can be used to separate content. It is a simple and effective way to divide content.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Down,
                <GDivider>{margin: { top: 8.0, bottom: 8.0 }, background_color: #FFF}
                <GDivider>{theme: Error, margin: { top: 8.0, bottom: 8.0 }}
                <GDivider>{theme: Info, margin: { top: 8.0, bottom: 8.0 }}
                <GDivider>{theme: Success, margin: { top: 8.0, bottom: 8.0 }}
                <GDivider>{theme: Warning, margin: { top: 8.0, bottom: 8.0 }}
            }
            code = {
                body: {
                    <GVLayout>{
                        height: Fit,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GDivider>{margin: { top: 8.0, bottom: 8.0 }, background_color: #FFF}
                <GDivider>{theme: Error, margin: { top: 8.0, bottom: 8.0 }}
                <GDivider>{theme: Info, margin: { top: 8.0, bottom: 8.0 }}
                <GDivider>{theme: Success, margin: { top: 8.0, bottom: 8.0 }}
                <GDivider>{theme: Warning, margin: { top: 8.0, bottom: 8.0 }}
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            width: Fill,
            text: "You can also add some other components to the Divider, such as Label.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Down,
                <GDivider>{
                    theme: Success,
                    height: Fit,
                    stroke_width: 2.0,
                    <GView>{
                        height: 20.0,
                        width: Fit,
                        padding: {left: 8.0, right: 8.0},
                        <GLabel>{
                            text: "Hello!"
                        }
                    }
                }
                <GDivider>{
                    theme: Error,
                    height: Fit,
                    stroke_width: 2.0,
                    align: {x: 0.8},
                    <GView>{
                        height: 20.0,
                        width: Fit,
                        padding: {left: 8.0, right: 8.0},
                        <GLabel>{
                            text: "Different Position!"
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    width: Fill,
                    <GLabel>{
                        text: "Left"
                    }
                    <GDivider>{
                        direction: Vertical,
                        height: 60.0,
                        theme: Error,
                        width: Fit,
                        stroke_width: 2.0,
                        margin: { left: 8.0, right: 8.0 },
                        <GIcon>{
                            icon_type: Open
                        }
                    }
                    <GLabel>{
                        text: "Right"
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: Fit,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GDivider>{
                    theme: Success,
                    height: Fit,
                    stroke_width: 2.0,
                    <GView>{
                        height: 20.0,
                        width: Fit,
                        padding: {left: 8.0, right: 8.0},
                        <GLabel>{
                            text: "Hello!"
                        }
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
pub struct DividerUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for DividerUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for DividerUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
