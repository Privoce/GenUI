use makepad_widgets::Cx;

// pub mod usage;
// pub mod animate;
// pub mod event;
// pub mod virt;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::components::view::GView;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::styles::*;

    CollapsePage = {{CollapsePage}}{
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
                    height: Fit,
                    spacing: 12.0,
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Top,
                        header: {
                            theme: Info,
                            <GLabel>{
                                text: "Open Top",
                            }
                            width: Fill,
                            height: 40.0,
                        }
                        body: {
                            theme: Info,
                            height: 140.0,
                            width: Fill,
                        }
                    }
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Bottom,
                        header: {
                            theme: Info,
                            <GLabel>{
                                text: "Open Bottom",
                            }
                            width: Fill,
                            height: 40.0,
                        }
                        body: {
                            theme: Info,
                            height: 140.0,
                            width: Fill,
                        }
                    }
                }
                <GHLayout>{
                    height: Fit,
                    <GCollapse>{
                        height: 180.0,
                        width: 300.0,
                        opened: false,
                        position: Left,
                        header: {
                            theme: Info,
                            <GLabel>{
                                text: "Open Left",
                            }
                            width: 60.0,
                            height: Fill,
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
                            <GLabel>{
                                text: "Open Right",
                            }
                            width: 60.0,
                            height: Fill,
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
                            <GSplitter>{
                                height: 200.0,
                                align: FromA(100),
                                a: <GView>{
                                    height: 200.0,
                                    width: 200.0,
                                    theme: Error
                                },
                                b: <GView>{
                                    height: 200.0,
                                    width: 200.0,
                                    theme: Success
                                }
                            }
                            <GSplitter>{
                                height: 200.0,
                                align: Weighted(0.5),
                                axis: Vertical,
                                a: <GView>{
                                    height: 200.0,
                                    width: Fill,
                                    theme: Error
                                },
                                b: <GView>{
                                    height: 200.0,
                                    width: Fill,
                                    theme: Success
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
