use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    self::live_design(cx);
}

use gen_components::*;
use makepad_widgets::*;
use utils::lifetime::{Executor, Lifetime};

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    use crate::styles::*;

    pub RouterPage = {{RouterPage}}{
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
                text: "Router",
            }
        }
        <GLabel>{
            width: Fill,
            text: "Router can help you navigate between different pages, usually used in the main content of the page.",
        }
        <CBox>{
            box_wrap = {
                spacing: 16.0,
                height: 400.0,
                <GHLayout>{
                    height: Fit,
                    spacing: 16.0,
                    to_d = <GButton>{
                        slot: {
                            text: "nav1"
                        }
                    }
                    to_e= <GButton>{
                        slot: {
                            text: "nav2"
                        }
                    }
                    to_f= <GButton>{
                        slot: {
                            text: "nav3"
                        }
                    }
                }
                app_router = <GRouter>{
                    bar_pages = {
                        page1 = <GBarPage>{
                            background_visible: true,
                            theme: Warning,
                            <GLabel>{
                                text: "APP PAGE1"
                            }
                        },
                        page2 = <GBarPage>{
                            background_visible: true,
                            theme: Error,
                            <GLabel>{
                                text: "APP PAGE2"
                            }
                        },
                        page3 = <GBarPage>{
                            background_visible: true,
                            theme: Success,
                            <GLabel>{
                                text: "APP PAGE3"
                            }
                        }
                        tabbar = <GTabbar>{
                            theme: Info,
                            <GTabbarItem>{
                                icon_slot: {
                                    src: dep("crate://self/resources/config.svg"),
                                }
                                text_slot: {
                                    text: "Config"
                                }
                            }
                            <GTabbarItem>{}
                            <GTabbarItem>{
                                icon_slot: {
                                    src: dep("crate://self/resources/all.svg"),
                                }
                                text_slot: {
                                    text: "All"
                                }
                            }
                        }
                    }
                    nav_pages = {
                        background_visible: true,
                        background_color:#FF0000,
                        nav_page1 = <GNavPage>{
                            header = {
                                title_wrap = {
                                    title = {
                                        text: "Page1"
                                    }
                                }
                                tool_wrap = {
                                    <GIcon>{
                                        theme: Dark,
                                        icon_type: OpenBottom,
                                        stroke_width: 1.2
                                    }
                                }
                            }
                            body = {
                                theme: Warning,
                                <GLabel>{
                                    text: "APP PAGE1"
                                }
                                <GButton>{}
                            }
                        },
                        nav_page2 = <GNavPage>{
                            header = {
                                title_wrap = {
                                    title = {
                                        text: "Page2"
                                    }
                                }
                                tool_wrap = {
                                    <GIcon>{
                                        theme: Dark,
                                        icon_type: OpenBottom,
                                        stroke_width: 1.2
                                    }
                                }
                            }
                            body = {
                                theme: Warning,
                                <GLabel>{
                                    text: "APP PAGE2"
                                }
                            }
                        },
                        nav_page3 = <GNavPage>{
                            header = {
                                title_wrap = {
                                    title = {
                                        text: "Page3"
                                    }
                                }
                                tool_wrap = {
                                    <GIcon>{
                                        theme: Dark,
                                        icon_type: OpenBottom,
                                        stroke_width: 1.2
                                    }
                                }
                            }
                            body = {
                                theme: Warning,
                                <GLabel>{
                                    text: "APP PAGE2"
                                }
                            }
                        },
                    }
                }
            }
            code = {
                body: {
                    <GVLayout>{
                        height: 40.0,
                        scroll_bars: <GScrollBars>{}
                        <GLabel>{
                            theme: Dark,
                            width: Fill,
                            text: r#"
You should directly read this file to understand the usage of the router component.
                            "#;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct RouterPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    pub lifetime: Lifetime,
}

impl LiveHook for RouterPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for RouterPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        
        self.lifetime
            .init()
            .execute(|| {
                let router = self.grouter(id!(app_router));

                router.borrow_mut().map(|mut router| {
                    let _ = router
                        .init(
                            ids!(page1, page2, page3),
                            Some(ids!(nav_page1, nav_page2, nav_page3)),
                            None,
                        )
                        .active(id!(page1))
                        .build(cx);
                    // let _ = router.init_auto().build(cx);
                });
            })
            .map(|_| {
                let router = self.grouter(id!(app_router));
                router.borrow().map(|router| {
                    if router.scope_path.is_some() {
                        // if is empty do not do next
                        self.lifetime.next();
                    }
                })
            });
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let router = self.grouter(id!(app_router));
        if self.gbutton(id!(to_d)).clicked(&actions).is_some() {
            router.nav_to(cx, id!(nav_page1));
        }
        if self.gbutton(id!(to_e)).clicked(&actions).is_some() {
            router.nav_to(cx, id!(nav_page2));
        }
        if self.gbutton(id!(to_f)).clicked(&actions).is_some() {
            router.nav_to(cx, id!(nav_page3));
        }
        
        router.handle_nav_events(cx, &actions);
    }
}
