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

    pub ToolTipPage = {{ToolTipPage}}{
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
                text: "ToolTip",
            }

        }
        <GLabel>{
            width: Fill,
            text: "ToolTips are a great way to provide additional information to users. They are displayed when users hover over an element with their mouse or focus on an element using a keyboard or other device. Tooltips are usually short and appear in a small box near the element.",
        }
        <GLabel>{
            width: Fill,
            text: "You can set different positions for the tooltip. The tooltip will automatically adjust its position to fit the screen.",
        }
        <GLabel>{
            width: Fill,
            text: "Position: \nTopLeft, Top, TopRight, RightTop, Right, RightBottom, LeftTop, Left, LeftBottom, BottomRight, Bottom, BottomLeft",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
                    align: {x: 0.5},
                    <GDropDown>{
                        position: TopLeft,
                        trigger = <GButton>{slot: {
                            text:"open top left"
                        }},
                        popup :<GToolTip> {
                            height: 100.0,
                            width: 200.0,
                            container: {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
                            }
                        }
                    }
                    <GDropDown>{
                        position: Top,
                        trigger = <GButton>{slot: {
                            text:"open top"
                        }},
                        popup :<GToolTip> {
                            height: 100.0,
                            width: 200.0,
                            container: {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
                            }
                        }
                    }
                    <GDropDown>{
                        position: TopRight,
                        trigger = <GButton>{slot: {
                            text:"open top right"
                        }},
                        popup :<GToolTip> {
                            height: 100.0,
                            width: 200.0,
                            container: {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
                            }
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    <GVLayout>{
                        height: Fit,
                        spacing: 16.0,
                        align: {x: 0.0},
                        <GDropDown>{
                            position: RightTop,
                            trigger = <GButton>{slot: {
                                text:"open right top"
                            }},
                            popup :<GToolTip> {
                                height: 100.0,
                                width: 200.0,
                                container: {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                        <GDropDown>{
                            position: Right,
                            trigger = <GButton>{slot: {
                                text:"open right"
                            }},
                            popup :<GToolTip> {
                                height: 100.0,
                                width: 200.0,
                                container: {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                        <GDropDown>{
                            position: RightBottom,
                            trigger = <GButton>{slot: {
                                text:"open right bottom"
                            }},
                            popup :<GToolTip> {
                                height: 100.0,
                                width: 200.0,
                                container: {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                    }
                    <GVLayout>{
                        height: Fit,
                        spacing: 16.0,
                        align: {x: 1.0},
                        <GDropDown>{
                            position: LeftTop,
                            trigger = <GButton>{slot: {
                                text:"open left top"
                            }},
                            popup :<GToolTip> {
                                height: 100.0,
                                width: 200.0,
                                container: {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                        <GDropDown>{
                            position: Left,
                            trigger = <GButton>{slot: {
                                text:"open left"
                            }},
                            popup :<GToolTip> {
                                height: 100.0,
                                width: 200.0,
                                container: {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                        <GDropDown>{
                            position: LeftBottom,
                            trigger = <GButton>{slot: {
                                text:"open left bottom"
                            }},
                            popup :<GToolTip> {
                                height: 100.0,
                                width: 200.0,
                                container: {
                                    height: Fill,
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10.0,
                                    padding: 10.0,
                                    <GLabel>{
                                        text:"This is a popup",
                                    }
                                }
                            }
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
                    align: {x: 0.5},
                    <GDropDown>{
                        position: BottomRight,
                        trigger = <GButton>{slot: {
                            text:"open bottom right"
                        }},
                        popup :<GToolTip> {
                            height: 150.0,
                            width: 200.0,
                            container: {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
                            }
                        }
                    }
                    <GDropDown>{
                        position: Bottom,
                        trigger = <GButton>{slot: {
                            text:"open bottom"
                        }},
                        popup :<GToolTip> {
                            height: 150.0,
                            width: 200.0,
                            container: {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
                            }
                        }
                    }
                    <GDropDown>{
                        position: BottomLeft,
                        trigger = <GButton>{slot: {
                            text:"open bottom left"
                        }},
                        popup :<GToolTip> {
                            height: 150.0,
                            width: 200.0,
                            container: {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
                                }
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
                    <GDropDown>{
                        position: BottomLeft,
                        trigger = <GButton>{slot: {
                            text:"open bottom left"
                        }},
                        popup :<GToolTip> {
                            height: 150.0,
                            width: 200.0,
                            container: {
                                height: Fill,
                                width: Fill,
                                flow: Down,
                                spacing: 10.0,
                                padding: 10.0,
                                <GLabel>{
                                    text:"This is a popup",
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
pub struct ToolTipPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ToolTipPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ToolTipPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
