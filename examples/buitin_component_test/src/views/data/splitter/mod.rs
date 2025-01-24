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

    pub SplitterPage = {{SplitterPage}}{
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
                text: "Splitter",
            }

        }
        <GLabel>{
            width: Fill,
            text: "Splitter can help you split the view, you can use it to split the view into two parts, and you can drag the splitter to change the size of the two parts.",
        }
        <CBox>{
            box_wrap = {
                spacing: 8.0,
                flow: Right,
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
pub struct SplitterPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for SplitterPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SplitterPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
