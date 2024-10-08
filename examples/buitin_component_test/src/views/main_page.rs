use gen_components::{
    components::{
        menu::{
            menu_item::{GMenuItem, GMenuItemWidgetExt},
            sub_menu::GSubMenuWidgetExt,
            GMenuWidgetExt,
        },
        router::{GRouterRef, GRouterWidgetExt},
        view::GView,
    },
    shader::manual::RouterIndicatorMode,
    utils::{
        lifetime::{Executor, Lifetime},
        HeapLiveIdPathExp,
    },
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import gen_components::components::*;
    import crate::views::overall::all::*;
    import crate::views::overall::color::*;

    AppMainPage = {{AppMainPage}}{
        height: Fill,
        width: Fill,
        flow: Right,
        background_visible: true,
        background_color: #2C313A,
        border_radius: 0.0,
        menu = <GMenu>{
            width: 180.0,
            header: {
                height: 32.0,
                width: Fill,
                visible: true,
                align: {
                    x: 0.0,
                    y: 0.5
                },
                <GLabel>{
                    font_size: 12.0,
                    text: "GenUI Components",
                }
            }
            body: {
                sub1 = <GSubMenu>{
                    id: 0,
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Overall 组件总览",
                        }
                    }
                    items: {
                        tab_overall = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Overall 组件总览",
                            }
                        }
                        tab_color = <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Color 颜色",
                            }
                        }
                    }
                }
                <GSubMenu>{
                    id: 1,
                    title: {
                        <GLabel>{
                            font_size: 11.0,
                            text: "Basic 基础组件",
                        }
                    }
                    items: {
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Label 文本",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "View 视图",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Button 按钮",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Svg 图标",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Image 图片",
                            }
                        }
                        <GMenuItem>{
                            icon_slot: {
                                visible: false,
                            }
                            text_slot: {
                                text: "Icon Lib 图标库",
                            }
                        }
                    }
                }

            }
        }
        app_router = <GRouter>{
            bar_pages = {
                flow: Down,
                overall_page = <GBarPage>{
                    <OverallPage>{}
                }
                color_page = <GBarPage>{
                    <ColorPage>{}
                }

            }
        }
    }
}

#[derive(Live, Widget)]
pub struct AppMainPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    lifetime: Lifetime,
}

impl LiveHook for AppMainPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for AppMainPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        self.lifetime
            .init()
            .execute(|| {
                let router = self.grouter(id!(app_router));

                router.borrow_mut().map(|mut router| {
                    let _ = router
                        .init(
                            ids!(overall_page, color_page),
                            None,
                            Some(RouterIndicatorMode::Define),
                        )
                        .active(id!(overall_page))
                        .build(cx);
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
        self.gmenu(id!(menu)).borrow().map(|menu| {
            menu.body.gsub_menu(id!(sub1)).borrow().map(|sub| {
                // sub.items.gmenu_item(id!(tab_overall)).borrow().map(|item| {

                //     let _ = nav_to(item, cx, &actions, &router, id!(overall_page));
                // });
                // sub.items.gmenu_item(id!(tab_color)).borrow().map(|item| {
                //     let _ = nav_to(item, cx, &actions, &router, id!(color_page));
                // });
                if let Some(e) = sub.changed(&actions) {
                    router.borrow_mut().map(|mut router| {
                        dbg!(e.selected_id);
                        // router.nav_to(cx, &[e.selected_id]);
                    });
                }
            });
        });

        router.borrow_mut().map(|mut route| {
            route.handle_nav_events(cx, &actions);
        });
    }
}
