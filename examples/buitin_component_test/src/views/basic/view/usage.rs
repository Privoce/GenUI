use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;
    
    pub ViewUsagePage = {{ViewUsagePage}}{
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
            text: "Basic View Usage",
        }
        <GLabel>{
            width: Fill,
            text: "View is a basic component that can be used to wrap other components.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Info,
                }
                <GView>{
                    height: 60.0,
                    width: 60.0,
                    theme: Error,
                }
                <GView>{
                    height: 60.0,
                    width: 120.0,
                    background_color: #DC9CAC,
                }
                <GView>{
                    height: 60.0,
                    width: 120.0,
                    background_color: #DC9CAC,
                    spread_radius: 5.2,
                    blur_radius: 5.2,
                    shadow_color: #FF0000,
                    clip_x: false,
                    clip_y: false,
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 200.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            height: Fit,
                            theme: Dark,
                            width: Fill,
                            text: r#"
                    <GView>{
                        height: 60.0,
                        width: 60.0,
                        theme: Info,
                    }
                    <GView>{
                        height: 60.0,
                        width: 60.0,
                        theme: Error,
                    }
                    <GView>{
                        height: 60.0,
                        width: 120.0,
                        background_color: #DC9CAC,
                    }
                    <GView>{
                        height: 60.0,
                        width: 120.0,
                        background_color: #DC9CAC,
                        spread_radius: 5.2,
                        blur_radius: 5.2,
                        shadow_color: #FF0000,
                        clip_x: true,
                        clip_y: true,
                    }
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            width: Fill,
            text: "Followings are VLayout and HLayout, layout's background_visible is false by default.",
        }
        <GLabel>{
            width: Fill,
            text: "height and width are Fill by default.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
                <GHLayout>{
                    spacing: 8.0,
                    height: 90.0,
                    border_width: 1.0,
                    border_color: #FFF,
                    <GButton>{}
                    <GButton>{theme: Info}
                }
                <GVLayout>{
                    height: 90.0,
                    spacing: 8.0,
                    border_width: 1.0,
                    border_color: #FFF,
                    <GButton>{}
                    <GButton>{theme: Error}
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 200.0,
                        scroll_bars: <GScrollBars>{},
                        <GLabel>{
                            height: Fit,
                            theme: Dark,
                            width: Fill,
                            text: r#"
                <GHLayout>{
                    spacing: 8.0,
                    height: 90.0,
                    border_width: 1.0,
                    border_color: #FFF,
                    <GButton>{}
                    <GButton>{theme: Info}
                }
                <GVLayout>{
                    height: 90.0,
                    spacing: 8.0,
                    border_width: 1.0,
                    border_color: #FFF,
                    <GButton>{}
                    <GButton>{theme: Error}
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
pub struct ViewUsagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ViewUsagePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ViewUsagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
