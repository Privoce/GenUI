use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub MenuPage = {{MenuPage}}{
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
        <GHLayout>{
            height: Fit,
            align: {x: 0.5},
            <GLabel>{
                font_size: 14.0,
                font_family: (BOLD_FONT),
                text: "Menu",
            }
        }
        <GLabel>{
            width: Fill,
            text: "Menu can help you navigate to different pages in the app.(This is a define way to use menu)",
        }
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "MenuItem",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                flow: Right,
                <GMenuItem>{
                    text_slot: {text: "No Icon No Right"},
                    icon_slot: {visible: false}
                }
                <GMenuItem>{
                    theme: Success
                    text_slot: {text: "With Icon No Right"},
                    icon_slot: {src: dep("crate://self/resources/dislike.svg")}
                }
                <GMenuItem>{
                    theme: Info,
                    text_slot: {text: "With Icon With Right"},
                    icon_slot: {src: dep("crate://self/resources/dislike.svg")},
                    right: {
                        align: {x: 0.5, y: 0.5},
                        visible: true,
                        <GTag>{
                            text: "0"
                        }
                    }
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
                <GMenuItem>{
                    text_slot: {text: "No Icon No Right"},
                    icon_slot: {visible: false}
                }
                <GMenuItem>{
                    theme: Success
                    text_slot: {text: "With Icon No Right"},
                    icon_slot: {src: dep("crate://self/resources/dislike.svg")}
                }
                <GMenuItem>{
                    theme: Info,
                    text_slot: {text: "With Icon With Right"},
                    icon_slot: {src: dep("crate://self/resources/dislike.svg")},
                    right: {
                        align: {x: 0.5, y: 0.5},
                        visible: true,
                        <GTag>{
                            text: "0"
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
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "SubMenu",
        }
        <CBox>{
            box_wrap = {
                spacing: 24.0,
                flow: Right,
                <GSubMenu>{
                    title: {
                        <GLabel> {text: "You can define"}
                    }
                    items: {
                        <GMenuItem>{
                            text_slot: {text: "No Icon No Right"},
                            icon_slot: {visible: false}
                        }
                        <GMenuItem>{
                            theme: Success
                            text_slot: {text: "With Icon No Right"},
                            icon_slot: {src: dep("crate://self/resources/dislike.svg")}
                        }
                        <GMenuItem>{
                            theme: Info,
                            text_slot: {text: "With Icon With Right"},
                            icon_slot: {src: dep("crate://self/resources/dislike.svg")},
                            right: {
                                align: {x: 0.5, y: 0.5},
                                visible: true,
                                <GTag>{
                                    text: "0"
                                }
                            }
                        }
                    }
                }
                <GSubMenu>{
                    title: {
                        <GLabel> {text: "Sub Menu Level1"}
                    }
                    items: {
                        <GMenuItem>{
                            theme: Warning,
                            text_slot: {text: "No Icon No Right"},
                            icon_slot: {visible: false}
                        }
                        <GMenuItem>{
                            theme: Success
                            text_slot: {text: "With Icon No Right"},
                            icon_slot: {src: dep("crate://self/resources/dislike.svg")}
                        }
                        <GSubMenu>{
                            margin: {left: 12.0}
                            title: {
                                <GLabel> {text: "Sub Menu Level2"}
                            }
                            items: {
                                <GMenuItem>{
                                    theme: Info,
                                    text_slot: {text: "With Icon With Right"},
                                    icon_slot: {src: dep("crate://self/resources/dislike.svg")},
                                    right: {
                                        align: {x: 0.5, y: 0.5},
                                        visible: true,
                                        <GTag>{
                                            text: "0"
                                        }
                                    }
                                }
                            }
                        }
                        <GMenuItem>{
                            theme: Error
                            text_slot: {text: "Child Item"},
                        }
                    }
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

                            "#;
                        }
                    }
                }
            }
        }
        <GLabel>{
            font_size: 12.0,
            font_family: (BOLD_FONT),
            text: "Menu",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                flow: Right,
                height: 400.0,
                <GMenu>{
                    height: Fit,
                    header: {
                        height: Fit,
                        visible: true,
                        <GLabel> {text: "define header"}
                    }
                    body: {
                        height: Fit,
                        <GSubMenu>{
                            title: {
                                <GLabel> {text: "Sub Menu Level1-1"}
                            }
                            items: {
                                <GMenuItem>{
                                    theme: Warning,
                                    text_slot: {text: "Menu Item1-1-1"},
                                    icon_slot: {visible: false}
                                }
                            }
                        }
                        <GSubMenu>{
                            title: {
                                <GLabel> {text: "Sub Menu Level1-2"}
                            }
                            items: {
                                <GMenuItem>{
                                    selected: true,
                                    theme: Warning,
                                    text_slot: {text: "Menu Item1-2-1"},
                                    icon_slot: {visible: false}
                                }
                                <GMenuItem>{
                                    theme: Success
                                    text_slot: {text: "Menu Item1-2-2"},
                                    icon_slot: {src: dep("crate://self/resources/dislike.svg")}
                                }
                                <GSubMenu>{
                                    margin: {left: 12.0}
                                    title: {
                                        <GLabel> {text: "Sub Menu Level1-2-1"}
                                    }
                                    items: {
                                        <GMenuItem>{
                                            theme: Info,
                                            text_slot: {text: "MenuItem1-2-1-1"},
                                            icon_slot: {src: dep("crate://self/resources/dislike.svg")},
                                            right: {
                                                align: {x: 0.5, y: 0.5},
                                                visible: true,
                                                <GTag>{
                                                    text: "0"
                                                }
                                            }
                                        }
                                    }
                                }
                                <GMenuItem>{
                                    theme: Error
                                    text_slot: {text: "MenuItem1-2-3"},
                                }
                            }
                        }
                    }
                    footer: {
                        height: Fit,
                        visible: true,
                        <GLabel> {text: "define footer"}
                    }
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

                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct MenuPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for MenuPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for MenuPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
    }
}
