use makepad_widgets::Cx;

// pub mod usage;
// pub mod animate;
// pub mod event;
// pub mod virt;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub StatePage = {{StatePage}}{
        height: Fill,
        width: Fill,
        flow: Down,
        background_visible: false,
        border_radius: 0.0,
        spacing: 12.0,
        padding: 12.0,
        scroll_bars: <GScrollBars>{},
        clip_x: true,
        clip_y: true,
        <GVLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "State(Test)",
            }

        }
        <GLabel>{
            width: Fill,
            text: "We provide a variety of states for you to use, such as 404, 500, success, error, etc.",
        }
        <GLabel>{
            width: Fill,
            text: "Actually State is Image. You can design your own state image and use it.(Next version it will be deprecated)",
        }
        <GLabel>{
            width: Fill,
            text: "I think the lib should be tiny, so these images should not be included in the lib.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GHLayout>{
                    height: 200.0,
                    width: Fill,
                    <GState404>{
                        height: 200.0,
                        width: Fill,
                    }
                    <GState502>{
                        height: 200.0,
                        width: Fill,
                    }
                    <GStateNoMsg>{
                        height: 200.0,
                        width: Fill,
                    }
                }
                <GHLayout>{
                    height: 200.0,
                    width: Fill,
                    <GStateNetWorkErr>{
                        height: 200.0,
                        width: Fill,
                    }
                    <GStateSearch>{
                        height: 200.0,
                        width: Fill,
                    }
                    <GStateNoData>{
                        height: 200.0,
                        width: Fill,
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
                            <GHLayout>{
                                height: 200.0,
                                width: Fill,
                                <GState404>{
                                    height: 200.0,
                                    width: Fill,
                                }
                                <GState502>{
                                    height: 200.0,
                                    width: Fill,
                                }
                                <GStateNoMsg>{
                                    height: 200.0,
                                    width: Fill,
                                }
                            }
                            <GHLayout>{
                                height: 200.0,
                                width: Fill,
                                <GStateNetWorkErr>{
                                    height: 200.0,
                                    width: Fill,
                                }
                                <GStateSearch>{
                                    height: 200.0,
                                    width: Fill,
                                }
                                <GStateNoData>{
                                    height: 200.0,
                                    width: Fill,
                                }
                            }
                            "#;
                        }
                    }
                }
            }
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GVLayout>{
                    theme: Success,
                    height: 200.0,
                    width: Fill,
                    spacing: 12.0,
                    align: {x: 0.5, y: 0.5},
                    <GIcon>{
                        color: #DDD,
                        height: 58.0,
                        width: 58.0,
                        icon_type: Help,
                        stroke_width: 2.0,
                    }
                    <GLabel>{
                        text: "This a help message"
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        align: {x: 0.5},
                        <GButton>{theme: Info, slot: {text: "Cancel"}}
                        <GButton>{theme: Success, slot: {text: "OK"}}
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
                <GVLayout>{
                    theme: Success,
                    height: 200.0,
                    width: Fill,
                    spacing: 12.0,
                    align: {x: 0.5, y: 0.5},
                    <GIcon>{
                        color: #DDD,
                        height: 58.0,
                        width: 58.0,
                        icon_type: Help,
                        stroke_width: 2.0,
                    }
                    <GLabel>{
                        text: "This a help message"
                    }
                    <GHLayout>{
                        height: Fit,
                        spacing: 16.0,
                        align: {x: 0.5},
                        <GButton>{theme: Info, slot: {text: "Cancel"}}
                        <GButton>{theme: Success, slot: {text: "OK"}}
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
pub struct StatePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for StatePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for StatePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
