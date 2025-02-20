use gen_components::{
    components::{button::GButtonWidgetExt, router::GRouterWidgetExt, view::GView},
    utils::{
        lifetime::{Executor, Lifetime},
        HeapLiveIdPathExp,
    },
};
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    
    use link::gen_components::*;
    TPage = {{TPage}}{
        height: Fill,
        width: Fill,
        flow: Down,

        menu = <GView>{
            height: 36.0,
            width: Fill,
            spacing: 8.0,
            to_a = <GButton>{
                slot: {
                    text: "a"
                }
            }
            to_b =<GButton>{
                slot: {
                    text: "b"
                }
            }
            to_c =<GButton>{
                slot: {
                    text: "c"
                }
            }
            to_d = <GButton>{
                slot: {
                    text: "d"
                }
            }
            to_e= <GButton>{
                slot: {
                    text: "e"
                }
            }
        }
        app_router = <GRouter>{
            background_visible: false,
            bar_pages = {
                page1 = <GView>{
                    visible:false,
                    height: Fill,
                    width: Fill,
                    theme: Warning,
                    border_radius: 0.0,
                    <GLabel>{
                        text: "APP PAGE1"
                    }
                },
                page2 = <GView>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    theme: Error,
                    border_radius: 0.0,
                    <GLabel>{
                        text: "APP PAGE2"
                    }
                },
                page3 = <GView>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    theme: Success,
                    border_radius: 0.0,
                    <GLabel>{
                        text: "APP PAGE3"
                    }
                }
                tabbar = <GTabbar>{
                    height: 46.0,
                    width: Fill,
                    selected: 0,
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
                nav_page1 = <GPage>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    border_radius: 0.0,
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
                nav_page2 = <GPage>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    border_radius: 0.0,
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
                        <Button>{
                            text: "origin"
                        }
                    }
                },
                nav_page3 = <GPage>{
                    visible: false,
                    height: Fill,
                    width: Fill,
                    border_radius: 0.0,
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
                            text: "APP PAGE3"
                        }

                    }
                },
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct TPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    pub lifetime: Lifetime,
}

impl LiveHook for TPage {}

impl Widget for TPage {
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
                    if !router.scope_path.is_empty() {
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
        // you can handle in here if define tabbar----------------------
        // self.gtabbar(id!(tabbar)).borrow().map(|x| {
        //     if let Some(e) = x.changed(&actions) {
        //         // call nav to
        //         router.borrow_mut().map(|mut route| {
        //             let path = route.bar_pages[e.selected].last();
        //             route.nav_to(cx, &[path]);
        //         });
        //     }
        // });
        // -------------------------------------------------------------
        if self.gbutton(id!(to_a)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                x.nav_to(cx, id!(page1));
            });
        }
        if self.gbutton(id!(to_b)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                x.nav_to(cx, id!(page2));
            });
        }
        if self.gbutton(id!(to_c)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                x.nav_to(cx, id!(page3));
            });
        }
        if self.gbutton(id!(to_d)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                x.nav_to(cx, id!(nav_page1));
            });
        }
        if self.gbutton(id!(to_e)).clicked(&actions).is_some() {
            router.borrow_mut().map(|mut x| {
                x.nav_to(cx, id!(nav_page2));
            });
        }
        router.borrow_mut().map(|mut route| {
            route.handle_nav_events(cx, &actions);
        });
    }
}
