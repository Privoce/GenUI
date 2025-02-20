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

    pub CollapsePage = {{CollapsePage}}{
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
                text: "Collapse",
            }

        }
        <GLabel>{
            width: Fill,
            text: "Collapse usually use to hide or show the content, it has a header and a body.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GHLayout>{
                    height: 200.0,
                    spacing: 12.0,
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Top,
                        header: {
                            theme: Info,
                            height: 40.0,
                            <GLabel>{
                                text: "Open Top",
                            }
                        }
                        body: {
                            theme: Info,
                            height: 140.0,
                            <View>{
                                <Button>{
                                    text: "Click Me",
                                }
                            }
                        }
                    }
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Bottom,
                        header: {
                            theme: Info,
                            height: 40.0,
                            <GLabel>{
                                text: "Open Bottom",
                            }
                        }
                        body: {
                            theme: Info,
                            height: 140.0,
                        }
                    }
                }
                <GHLayout>{
                    height: 200.0,
                    spacing: 12.0,
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Left,
                        header: {
                            theme: Info,
                            height: Fill,
                            width: 60.0,
                            <GLabel>{
                                text: "Left",
                            }
                        }
                        body: {
                            height: Fill,
                            width: 240.0,
                            theme: Info,
                        }
                    }
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Right,
                        header: {
                            theme: Info,
                            width: 60.0,
                            height: Fill,
                            <GLabel>{
                                text: "Right",
                            }
                        }
                        body: {
                            height: Fill,
                            width: 240.0,
                            theme: Info,
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
                <GHLayout>{
                    height: 200.0,
                    spacing: 12.0,
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Top,
                        header: {
                            theme: Info,
                            height: 40.0,
                            <GLabel>{
                                text: "Open Top",
                            }
                        }
                        body: {
                            theme: Info,
                            height: 140.0,
                        }
                    }
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Bottom,
                        header: {
                            theme: Info,
                            height: 40.0,
                            <GLabel>{
                                text: "Open Bottom",
                            }
                        }
                        body: {
                            theme: Info,
                            height: 140.0,
                        }
                    }
                }
                <GHLayout>{
                    height: 200.0,
                    spacing: 12.0,
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Left,
                        header: {
                            theme: Info,
                            height: Fill,
                            width: 60.0,
                            <GLabel>{
                                text: "Left",
                            }
                        }
                        body: {
                            height: Fill,
                            width: 240.0,
                            theme: Info,
                        }
                    }
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Right,
                        header: {
                            theme: Info,
                            width: 60.0,
                            height: Fill,
                            <GLabel>{
                                text: "Right",
                            }
                        }
                        body: {
                            height: Fill,
                            width: 240.0,
                            theme: Info,
                        }
                    }
                }
                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            width: Fill,
            text: "You can make collapses as a list, and you can also make it as a group.",
        }
        <CBox>{
            box_wrap = {
                <GVLayout>{
                    height: 300.0,
                    width: Fill,
                    background_visible: true,
                    background_color: #FFF,
                    <GCollapse>{
                        header: {
                            theme: Info,
                            height: 40.0,
                            <GLabel>{
                                text: "Click Me A",
                            }        
                        }
                        body: {
                            height: Fill,
                            <GLabel>{
                                width: Fill,
                                text: "Francesco Centemeri joined, Jean changed their profile picture 2 times"
                            }
                        }
                    }
                    <GCollapse>{
                        header: {
                            theme: Info,
                            height: 40.0,
                            <GLabel>{
                                text: "Click Me B",
                            }        
                        }
                        body: {
                            height: Fill,
                            <GLabel>{
                                width: Fill,
                                text: "simpleguy3 left, ririshere changed their name and changed their profile picture, ic made no changes, Snapychanged their name, nergzd723 joined and made no changes, nergzd joined"
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
        <GVLayout>{
            height: 300.0,
            width: Fill,
            background_visible: true,
            background_color: #FFF,
            <GCollapse>{
                header: {
                    theme: Info,
                    height: 40.0,
                    <GLabel>{
                        text: "Click Me A",
                    }        
                }
                body: {
                    height: Fill,
                    <GLabel>{
                        width: Fill,
                        text: "Francesco Centemeri joined, Jean changed their profile picture 2 times"
                    }
                }
            }
            <GCollapse>{
                header: {
                    theme: Info,
                    height: 40.0,
                    <GLabel>{
                        text: "Click Me B",
                    }        
                }
                body: {
                    height: Fill,
                    <GLabel>{
                        width: Fill,
                        text: "simpleguy3 left, ririshere changed their name and changed their profile picture, ic made no changes, Snapychanged their name, nergzd723 joined and made no changes, nergzd joined"
                    }
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
pub struct CollapsePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for CollapsePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for CollapsePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
