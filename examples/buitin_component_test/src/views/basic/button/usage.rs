use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub ButtonUsagePage = {{ButtonUsagePage}}{
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
            text: "Basic Button Usage",
        }
        <GLabel>{
            width: Fill,
            text: "Button is a basic component for user interaction",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GHLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GButton>{  }
                    <GButton>{theme: Dark}
                    <GButton>{theme: Info}
                    <GButton>{theme: Success}
                    <GButton>{theme: Warning}
                    <GButton>{theme: Error}
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 8.0,
                    <GButton>{
                        border_radius: 8.0,
                        slot: {
                            stroke_hover_color: #FF0000,
                            animation_key: true,
                            text: "Round Button",
                        }
                    }
                    <GButton>{
                        padding: 12.0,
                        theme: Error,
                        border_radius: 9.0,
                        slot: <GIcon>{
                            height: 12.0,
                            width: 12.0,
                            icon_type: Add,
                            stroke_width: 1.2,
                        }
                    }
                    <GButton>{
                        padding: 12.0,
                        theme: Warning,
                        slot: <GIcon>{
                            height: 12.0,
                            width: 12.0,
                            icon_type: Left,
                            stroke_width: 1.2,
                        }
                    }
                    <GButton>{
                        theme: Success,
                        border_radius: 8.0,
                        slot: <GHLayout>{
                            height: Fit,
                            spacing: 8.0,
                            <GIcon>{
                                height: 12.0,
                                width: 12.0,
                                icon_type: Add,
                                stroke_width: 1.2,
                            }
                            <GLabel>{
                                text: "Icon Button",
                            }
                        }
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
                    <GButton>{ }
                    <GButton>{theme: Dark}
                    <GButton>{theme: Info}
                    <GButton>{theme: Success}
                    <GButton>{theme: Warning}
                    <GButton>{theme: Error}
                    <GButton>{
                            border_radius: 8.0,
                            slot: {
                                text: "Round Button",
                            }
                        }
                        <GButton>{
                            padding: 12.0, 
                            theme: Error,
                            border_radius: 9.0,
                            slot: <GIcon>{
                                height: 12.0,
                                width: 12.0,
                                icon_type: Add,
                                stroke_width: 1.2,
                            }
                        }
                        <GButton>{
                            padding: 12.0, 
                            theme: Warning,
                            slot: <GIcon>{
                                height: 12.0,
                                width: 12.0,
                                icon_type: Left,
                                stroke_width: 1.2,
                            }
                        }
                        <GButton>{
                            theme: Success,
                            border_radius: 8.0,
                            slot: <GHLayout>{
                                height: Fit,
                                spacing: 8.0,
                                <GIcon>{
                                    height: 12.0,
                                    width: 12.0,
                                    icon_type: Add,
                                    stroke_width: 1.2,
                                }
                                <GLabel>{
                                    text: "Icon Button",
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
pub struct ButtonUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ButtonUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ButtonUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
